//! Pure decisions about finished attempts: accept, retry, or fail.

use std::time::Duration;

use crate::{Item, LengthPolicy, Request, Response, SampleError, SamplerSettings, StopReason};

/// Appended after a looping reply before the model tries again.
pub(crate) const RECOVERY_REMINDER: &str = "<system_reminder>Your messages have been flagged as looping. Your response has been flagged as repeating the same text pattern. Avoid excessive repetition. If you are having trouble ask the user for guidance.</system_reminder>";

/// Most partial text replayed after a doom loop.
const PARTIAL_CAP: usize = 4096;

/// Retry bookkeeping for one sample.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct RetryState {
    pub retries: u32,
    pub doom_retries: u32,
    /// Stop checking for doom loops.
    pub disarmed: bool,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Decision {
    Fail,
    Retry { delay: Duration, stripped: usize },
}

/// Decide what to do after `error`. May edit `request` (strip images, add a
/// recovery reminder) and `state`. `jitter` is a random number in `0..1`.
pub(crate) fn decide(
    settings: &SamplerSettings,
    state: &mut RetryState,
    request: &mut Request,
    error: &SampleError,
    output_seen: bool,
    partial: &str,
    jitter: f64,
) -> Decision {
    let policy = &settings.retry;
    let retry = |delay, stripped| Decision::Retry { delay, stripped };
    let status = error.status();

    if status == Some(401)
        || (status == Some(400) && error.server_message().contains("encrypted_content"))
    {
        return Decision::Fail;
    }
    if let SampleError::DoomLoop { .. } = error {
        let budget = settings.doom_loop.as_ref().map_or(0, |d| d.max_retries);
        if state.doom_retries >= budget {
            return Decision::Fail;
        }
        state.doom_retries += 1;
        state.disarmed = state.doom_retries >= budget;
        if !partial.is_empty() {
            let end = partial.floor_char_boundary(PARTIAL_CAP);
            request.items.push(Item::Assistant {
                text: partial[..end].into(),
                tool_calls: Vec::new(),
            });
        }
        request.items.push(Item::user(RECOVERY_REMINDER));
        return retry(Duration::from_secs_f64(0.25 * jitter), 0);
    }
    if policy.max_retries == 0 || (policy.only_before_output && output_seen) {
        return Decision::Fail;
    }
    if error.is_too_large() || error.is_bad_image() {
        return match request.strip_images() {
            0 => Decision::Fail,
            stripped => retry(Duration::ZERO, stripped),
        };
    }
    let rate_limited = status == Some(429);
    let retry_after = match error {
        SampleError::Status { retry_after, .. } => {
            let size_veto = error.is_context_length() && !(rate_limited && retry_after.is_some());
            // A wait longer than the policy allows fails now; the error keeps the wait.
            let too_long = retry_after.is_some_and(|wait| wait > policy.max_server_delay);
            if size_veto || too_long {
                return Decision::Fail;
            }
            *retry_after
        }
        _ if error.is_context_length() => return Decision::Fail,
        _ => None,
    };

    let next = state.retries + 1;
    let limit = if rate_limited {
        policy
            .max_retries
            .min(policy.rate_limit_attempts.saturating_sub(1))
    } else {
        policy.max_retries
    };
    if !error.is_retryable() || next > limit {
        return Decision::Fail;
    }
    state.retries = next;

    let stripped = match error {
        SampleError::Http {
            kind: crate::HttpKind::Request,
            ..
        } => request.strip_images(),
        _ => 0,
    };
    let backoff = policy
        .base_delay
        .saturating_mul(1 << (next - 1).min(16))
        .min(policy.max_delay);
    let delay = retry_after.unwrap_or(backoff.mul_f64(0.8 + 0.4 * jitter));
    retry(delay, stripped)
}

/// Check a response that streamed to the end. `context_window` catches
/// servers that cut the input to fit without saying so.
pub(crate) fn verdict(
    mut response: Response,
    policy: LengthPolicy,
    context_window: Option<u64>,
) -> Result<Response, SampleError> {
    let usage = response.usage;
    let length = response.stop_reason == StopReason::Length;
    if let Some(window) = context_window {
        let over = !length && usage.prompt > window;
        let filled = length && usage.completion == 0 && usage.prompt * 100 >= window * 99;
        if over || filled {
            return Err(SampleError::ContextOverflow {
                input: usage.prompt,
                window,
            });
        }
    }
    let valid = |args: &str| {
        args.trim().is_empty() || serde_json::from_str::<serde_json::Value>(args).is_ok()
    };
    match response.stop_reason {
        StopReason::ContentFilter => return Ok(response),
        StopReason::Length => {
            let calls_complete = !response.tool_calls.is_empty()
                && response.tool_calls.iter().all(|c| valid(&c.arguments));
            let keep = match policy {
                LengthPolicy::Fail => false,
                LengthPolicy::CompleteToolCalls => calls_complete,
                LengthPolicy::CompletePartial => {
                    response.tool_calls.retain(|c| valid(&c.arguments));
                    calls_complete || !response.text.is_empty()
                }
            };
            if !keep {
                return Err(SampleError::Truncated);
            }
        }
        _ => {}
    }
    if response.text.is_empty() && response.tool_calls.is_empty() {
        return Err(SampleError::Empty {
            reasoning_only: !response.reasoning.is_empty(),
        });
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use bevy_tool::ToolCall;

    use super::*;
    use crate::{DoomLoopPolicy, HttpKind, Part};

    fn status(status: u16, message: &str, retry_after: Option<u64>) -> SampleError {
        SampleError::Status {
            status,
            message: message.into(),
            code: None,
            retry_after: retry_after.map(Duration::from_secs),
            should_retry: None,
        }
    }

    fn run(error: &SampleError, state: &mut RetryState, request: &mut Request) -> Decision {
        let settings = SamplerSettings {
            doom_loop: Some(DoomLoopPolicy::default()),
            ..Default::default()
        };
        decide(&settings, state, request, error, true, "partial", 0.5)
    }

    #[test]
    fn decisions() {
        let secs = |s| Decision::Retry {
            delay: Duration::from_secs(s),
            stripped: 0,
        };
        let http = SampleError::Http {
            kind: HttpKind::Connect,
            message: String::new(),
        };
        let cases = [
            (http, 0, secs(2)),
            (status(401, "", None), 0, Decision::Fail),
            (status(400, "bad", None), 0, Decision::Fail),
            (status(500, "", None), 0, secs(2)),
            (status(503, "", None), 2, secs(8)),
            (status(503, "", None), 15, Decision::Fail),
            (status(526, "", None), 0, Decision::Fail),
            (status(408, "", None), 0, secs(2)),
            (status(409, "", None), 0, secs(2)),
            (status(429, "", Some(7)), 0, secs(7)),
            (status(429, "", Some(61)), 0, Decision::Fail),
            (status(429, "", None), 1, Decision::Fail),
            (status(400, "prompt is too long", None), 0, Decision::Fail),
            (
                status(503, "exceeds the limit of 128000", None),
                0,
                Decision::Fail,
            ),
            (status(429, "request too large", Some(3)), 0, secs(3)),
            (SampleError::IdleTimeout(Duration::ZERO), 0, Decision::Fail),
            (
                SampleError::Empty {
                    reasoning_only: true,
                },
                0,
                secs(2),
            ),
            (SampleError::Truncated, 0, Decision::Fail),
        ];
        for (error, retries, expected) in cases {
            let mut state = RetryState {
                retries,
                ..Default::default()
            };
            assert_eq!(
                run(&error, &mut state, &mut Request::default()),
                expected,
                "{error}"
            );
        }
    }

    #[test]
    fn bad_image_strips_images_once() {
        let mut request = Request::new(vec![Item::User(vec![Part::Image("data:x".into())])]);
        let error = status(400, "Could not process image", None);
        let mut state = RetryState::default();
        assert_eq!(
            run(&error, &mut state, &mut request),
            Decision::Retry {
                delay: Duration::ZERO,
                stripped: 1
            }
        );
        assert_eq!(request.items, vec![Item::user(crate::IMAGE_REMOVED)]);
        assert_eq!(run(&error, &mut state, &mut request), Decision::Fail);
    }

    #[test]
    fn doom_loop_replays_partial_then_disarms() {
        let error = SampleError::DoomLoop { triggers: vec![] };
        let mut request = Request::default();
        let mut state = RetryState::default();
        assert!(matches!(
            run(&error, &mut state, &mut request),
            Decision::Retry { .. }
        ));
        assert!(!state.disarmed);
        assert_eq!(request.items[1], Item::user(RECOVERY_REMINDER));
        run(&error, &mut state, &mut request);
        assert!(state.disarmed);
        assert_eq!(run(&error, &mut state, &mut request), Decision::Fail);
    }

    #[test]
    fn length_policy() {
        let call = |arguments: &str| ToolCall {
            arguments: arguments.into(),
            ..Default::default()
        };
        let response = |calls: Vec<ToolCall>, text: &str| Response {
            stop_reason: StopReason::Length,
            tool_calls: calls,
            text: text.into(),
            ..Default::default()
        };
        assert!(verdict(
            response(vec![call("{}")], ""),
            LengthPolicy::CompleteToolCalls,
            None
        )
        .is_ok());
        assert!(verdict(
            response(vec![call("{\"a\":")], ""),
            LengthPolicy::CompleteToolCalls,
            None
        )
        .is_err());
        assert!(verdict(response(vec![call("{}")], ""), LengthPolicy::Fail, None).is_err());
        let kept = verdict(
            response(vec![call("{")], "hi"),
            LengthPolicy::CompletePartial,
            None,
        )
        .unwrap();
        assert!(kept.tool_calls.is_empty());
        assert_eq!(
            verdict(Response::default(), LengthPolicy::Fail, None),
            Err(SampleError::Empty {
                reasoning_only: false
            })
        );
        // The server filled the window and could not write anything.
        let mut full = response(vec![], "");
        full.usage.prompt = 1000;
        assert_eq!(
            verdict(full, LengthPolicy::CompletePartial, Some(1000)),
            Err(SampleError::ContextOverflow {
                input: 1000,
                window: 1000
            })
        );
    }
}
