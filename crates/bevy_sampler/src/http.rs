//! The HTTP side of one attempt: send the request, parse the server-sent
//! events, and decode them into deltas and a [`Response`]. Plain async code
//! with no Bevy types.

use std::{
    mem::{replace, take},
    str::FromStr,
    time::{Duration, Instant, SystemTime},
};

use async_io::Timer;
use bevy_model::ModelInfo;
use bevy_tasks::futures_lite::future;
use reqwest::{
    header::{HeaderMap, ACCEPT, CONTENT_ENCODING, CONTENT_TYPE, RETRY_AFTER},
    Client, RequestBuilder,
};
use serde_json::Value;

use crate::{
    backend::{self, Builder, Chunk, Decoder},
    error::clip,
    Backend, Delta, Request, Response, SampleError, SamplerSettings,
};

/// Request bodies at least this big are compressed when [`SamplerSettings::compress`] is on.
const COMPRESS_MIN: usize = 64 * 1024;
/// Repeated runs this many tokens long count as a loop.
const EXACT_REPETITION_TOKENS: u64 = 64;
const ANTHROPIC_VERSION: &str = "2023-06-01";

/// The shared client. HTTP/2 pings find dead connections so requests do not
/// hang on them.
pub(crate) fn client() -> Client {
    Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .pool_idle_timeout(Duration::from_secs(90))
        .tcp_keepalive(Duration::from_secs(30))
        .http2_keep_alive_interval(Duration::from_secs(15))
        .http2_keep_alive_timeout(Duration::from_secs(5))
        .http2_keep_alive_while_idle(true)
        .build()
        .expect("failed to build the HTTP client")
}

/// Open a connection to the API so the first request skips the TLS handshake.
pub(crate) async fn prewarm(client: Client, base_url: String) {
    if let Ok(url) = reqwest::Url::parse(&base_url) {
        let origin = url.origin().ascii_serialization();
        let _ = client
            .head(origin)
            .timeout(Duration::from_secs(15))
            .send()
            .await;
    }
}

/// Everything one attempt needs.
pub(crate) struct Attempt {
    pub client: Client,
    pub settings: SamplerSettings,
    pub request: Request,
    /// Abort when the server reports a doom loop.
    pub armed: bool,
}

/// How an attempt ended.
pub(crate) struct Outcome {
    /// The decoded response. [`retry::verdict`](crate::retry::verdict) still checks it.
    pub result: Result<Response, SampleError>,
    /// Text, reasoning, or tool calls were streamed.
    pub output_seen: bool,
    /// Text streamed before an error, replayed after a doom loop.
    pub partial: String,
    pub time_to_first_token: Option<Duration>,
}

/// Run one attempt, passing each streamed delta to `emit`.
pub(crate) async fn run(attempt: Attempt, mut emit: impl FnMut(Delta)) -> Outcome {
    let started = Instant::now();
    let mut first_token = None;
    let mut builder = Builder::default();
    let streamed = stream(&attempt, &mut builder, &mut |delta: Delta| {
        if !matches!(delta, Delta::HostedTool { .. }) {
            first_token.get_or_insert_with(|| started.elapsed());
        }
        emit(delta);
    })
    .await;
    let (result, partial) = match streamed {
        Ok((model_info, models_etag)) => {
            let mut response = Response {
                model_info,
                models_etag,
                ..builder.finish()
            };
            let model = attempt
                .request
                .model
                .as_ref()
                .unwrap_or(&attempt.settings.model);
            for reasoning in &mut response.reasoning {
                reasoning.model = Some(model.clone());
            }
            (Ok(response), String::new())
        }
        Err(error) => (Err(error), builder.response.text),
    };
    Outcome {
        result,
        output_seen: first_token.is_some(),
        partial,
        time_to_first_token: first_token,
    }
}

/// Send the request and feed the reply into `builder`. Returns the model
/// limits and model list version from the response headers.
async fn stream(
    attempt: &Attempt,
    builder: &mut Builder,
    emit: &mut impl FnMut(Delta),
) -> Result<(ModelInfo, Option<String>), SampleError> {
    let settings = &attempt.settings;
    let mut idle = Idle::new(settings.idle_timeout);

    let mut response = idle.guard(build(attempt).send()).await??;
    if !response.status().is_success() {
        return Err(idle.guard(status_error(response)).await?);
    }
    let metadata = metadata(response.headers());
    let mut parser = SseParser::default();
    let mut decoder = Decoder::new(settings.backend);
    loop {
        let bytes = idle.guard(response.chunk()).await??;
        let events = match &bytes {
            Some(bytes) => parser.push(bytes),
            None => parser.finish(),
        };
        for event in events {
            let chunks = decoder.decode(&event)?;
            if !chunks.is_empty() {
                // Keepalive pings decode to nothing and do not count as progress.
                idle.reset();
            }
            for chunk in chunks {
                match &chunk {
                    Chunk::Delta(delta) => emit(delta.clone()),
                    Chunk::DoomLoop(triggers)
                        if attempt.armed && is_doom_loop(triggers, settings) =>
                    {
                        return Err(SampleError::DoomLoop {
                            triggers: triggers.clone(),
                        });
                    }
                    _ => {}
                }
                builder.push(chunk);
            }
        }
        // Some servers close after the stop reason without an end marker.
        if builder.done || (bytes.is_none() && builder.stopped) {
            return Ok(metadata);
        }
        if bytes.is_none() {
            return Err(SampleError::Stream {
                message: "stream ended before completion".into(),
                code: None,
            });
        }
    }
}

/// Build the POST for one attempt.
fn build(attempt: &Attempt) -> RequestBuilder {
    let Attempt {
        client,
        settings,
        request,
        armed,
    } = attempt;
    let url = format!(
        "{}/{}",
        settings.base_url.trim_end_matches('/'),
        settings.backend.path()
    );
    let mut http = client
        .post(url)
        .query(&settings.query)
        .header(ACCEPT, "text/event-stream")
        .header(CONTENT_TYPE, "application/json");
    if let Some(key) = &settings.api_key {
        http = if settings.auth.uses_x_api_key(settings.backend) {
            http.header("x-api-key", key)
        } else {
            http.bearer_auth(key)
        };
    }
    if settings.backend == Backend::Messages {
        http = http.header("anthropic-version", ANTHROPIC_VERSION);
    }
    if *armed && let Some(policy) = &settings.doom_loop {
        http = http
            .header("x-grok-doom-loop-check", policy.window_tokens)
            .header("x-grok-exact-repetition-check", EXACT_REPETITION_TOKENS);
    }
    for (name, value) in settings.headers.iter().chain(&request.headers) {
        http = http.header(name, value);
    }

    let body = backend::encode(settings, request).to_string().into_bytes();
    if settings.compress
        && body.len() >= COMPRESS_MIN
        && let Ok(compressed) = zstd::encode_all(&body[..], 3)
    {
        return http.header(CONTENT_ENCODING, "zstd").body(compressed);
    }
    http.body(body)
}

/// Fails an attempt that makes no progress for `limit`.
struct Idle {
    limit: Duration,
    deadline: Instant,
}

impl Idle {
    fn new(limit: Duration) -> Self {
        Self {
            limit,
            deadline: Instant::now() + limit,
        }
    }

    fn reset(&mut self) {
        self.deadline = Instant::now() + self.limit;
    }

    /// Run `future`, failing with [`SampleError::IdleTimeout`] if the deadline passes first.
    async fn guard<T>(&self, future: impl Future<Output = T>) -> Result<T, SampleError> {
        let expired = async {
            Timer::at(self.deadline).await;
            Err(SampleError::IdleTimeout(self.limit))
        };
        future::or(async { Ok(future.await) }, expired).await
    }
}

/// Turns server-sent event bytes into event payloads. Lines may be split
/// anywhere. Every backend puts the event type inside the JSON data, so only
/// `data:` lines matter; `event:`, `id:`, and comments are skipped.
#[derive(Default)]
struct SseParser {
    buf: Vec<u8>,
    data: String,
}

impl SseParser {
    /// Feed bytes and return the data of every event they complete.
    fn push(&mut self, bytes: &[u8]) -> Vec<String> {
        self.buf.extend_from_slice(bytes);
        let Some(end) = self.buf.iter().rposition(|&b| b == b'\n') else {
            return Vec::new();
        };
        let rest = self.buf.split_off(end + 1);
        let lines = replace(&mut self.buf, rest);
        let mut events = Vec::new();
        for line in lines[..end].split(|&b| b == b'\n') {
            let line = String::from_utf8_lossy(line);
            let line = line.strip_suffix('\r').unwrap_or(&line);
            if line.is_empty() && !self.data.is_empty() {
                events.push(take(&mut self.data));
            } else if let Some(value) = line.strip_prefix("data:") {
                if !self.data.is_empty() {
                    self.data.push('\n');
                }
                self.data.push_str(value.strip_prefix(' ').unwrap_or(value));
            }
        }
        events
    }

    /// Flush an event the server did not end with a blank line.
    fn finish(&mut self) -> Vec<String> {
        self.push(b"\n\n")
    }
}

async fn status_error(response: reqwest::Response) -> SampleError {
    let status = response.status().as_u16();
    let headers = response.headers().clone();
    let body = response.text().await.unwrap_or_default();
    parse_status_error(status, &headers, body)
}

/// Read a failed response. Providers send one of these bodies:
/// `{"error": {"message", "code" | "type"}}`, `{"error": "message"}`, or
/// `{"message", "code"}`. Anything else becomes the message as is.
fn parse_status_error(status: u16, headers: &HeaderMap, body: String) -> SampleError {
    let json: Value = serde_json::from_str(&body).unwrap_or_default();
    let error = json.get("error").filter(|e| e.is_object()).unwrap_or(&json);
    SampleError::Status {
        status,
        message: clip(
            field(error, "message")
                .or_else(|| field(&json, "error"))
                .unwrap_or(body),
        ),
        code: field(error, "code").or_else(|| field(error, "type")),
        retry_after: retry_after(headers),
        should_retry: header(headers, "x-should-retry"),
    }
}

/// How long the server asks us to wait: `retry-after-ms`, or `Retry-After` in
/// seconds or as a date.
fn retry_after(headers: &HeaderMap) -> Option<Duration> {
    if let Some(millis) = header::<f64>(headers, "retry-after-ms") {
        return Duration::try_from_secs_f64(millis / 1000.0).ok();
    }
    let value = headers.get(RETRY_AFTER)?.to_str().ok()?.trim();
    match value.parse::<f64>() {
        Ok(secs) => Duration::try_from_secs_f64(secs).ok(),
        Err(_) => {
            let at = httpdate::parse_http_date(value).ok()?;
            Some(at.duration_since(SystemTime::now()).unwrap_or_default())
        }
    }
}

fn field(json: &Value, key: &str) -> Option<String> {
    Some(json.get(key)?.as_str()?.to_owned())
}

fn metadata(headers: &HeaderMap) -> (ModelInfo, Option<String>) {
    let info = ModelInfo {
        context_window: header(headers, "x-grok-context-window"),
        max_output_tokens: header(headers, "x-grok-max-completion-tokens"),
        ..ModelInfo::default()
    };
    (info, header(headers, "x-models-etag"))
}

/// Parse a header value, ignoring surrounding whitespace.
fn header<T: FromStr>(headers: &HeaderMap, name: &str) -> Option<T> {
    headers.get(name)?.to_str().ok()?.trim().parse().ok()
}

/// The server flags loops with triggers like `tail_repetition:12@thinking`. Only
/// thinking triggers at or below the policy's `max_threshold` count.
fn is_doom_loop(triggers: &[String], settings: &SamplerSettings) -> bool {
    let max = settings.doom_loop.as_ref().map_or(0, |d| d.max_threshold) as f64;
    triggers.iter().any(|trigger| {
        trigger
            .strip_prefix("tail_repetition:")
            .and_then(|rest| rest.strip_suffix("@thinking"))
            .and_then(|value| value.parse::<f64>().ok())
            .is_some_and(|value| value <= max)
    })
}
