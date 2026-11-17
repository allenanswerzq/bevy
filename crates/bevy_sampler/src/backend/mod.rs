//! Wire formats. Each backend encodes a [`Request`] into JSON and decodes
//! stream events into [`Chunk`]s; [`Builder`] assembles the chunks into a
//! [`Response`] the same way for all of them.

mod chat;
mod messages;
mod responses;

use bevy_tool::{ToolCall, ToolChoice};
use serde_json::Value;

use crate::{
    error::clip, Backend, CacheRetention, Delta, Reasoning, Request, Response, SampleError,
    SamplerSettings, StopReason, Usage,
};

/// Something a backend learned from one event.
#[derive(Debug, PartialEq)]
pub(crate) enum Chunk {
    Delta(Delta),
    Started {
        id: String,
        model: String,
    },
    /// A complete reasoning item; replaces the streamed reasoning text.
    Reasoning(Reasoning),
    /// The final form of the tool call at this index.
    ToolCallDone(u32, ToolCall),
    Hosted(Value),
    Usage(Usage),
    /// Why the model stopped, and the provider's own word for it.
    Stop(StopReason, String),
    DoomLoop(Vec<String>),
    Done,
}

pub(crate) fn encode(settings: &SamplerSettings, request: &Request) -> Value {
    let model = request.model.as_deref().unwrap_or(&settings.model);
    let mut body = match settings.backend {
        Backend::ChatCompletions => chat::encode(model, request, &settings.chat),
        Backend::Responses => responses::encode(model, request),
        Backend::Messages => messages::encode(model, request),
    };
    for (key, value) in &request.extra {
        body[key] = value.clone();
    }
    body
}

/// Decoding state for one stream.
pub(crate) enum Decoder {
    Chat,
    Responses,
    Messages(messages::Block),
}

impl Decoder {
    pub fn new(backend: Backend) -> Self {
        match backend {
            Backend::ChatCompletions => Self::Chat,
            Backend::Responses => Self::Responses,
            Backend::Messages => Self::Messages(messages::Block::None),
        }
    }

    pub fn decode(&mut self, data: &str) -> Result<Vec<Chunk>, SampleError> {
        if data == "[DONE]" {
            return Ok(vec![Chunk::Done]);
        }
        let event: Value = serde_json::from_str(data)
            .map_err(|e| SampleError::Decode(clip(format!("{e}: {data}"))))?;
        let mut out = Vec::new();
        match self {
            Self::Chat => chat::decode(&event, &mut out),
            Self::Responses => responses::decode(&event, &mut out),
            Self::Messages(block) => messages::decode(block, &event, &mut out),
        }?;
        Ok(out)
    }
}

/// Assembles chunks into a [`Response`].
#[derive(Default)]
pub(crate) struct Builder {
    pub response: Response,
    /// The stream sent its end marker.
    pub done: bool,
    /// The stream reported why the model stopped.
    pub stopped: bool,
    reasoning: String,
    calls: Vec<(u32, ToolCall)>,
}

impl Builder {
    pub fn push(&mut self, chunk: Chunk) {
        let response = &mut self.response;
        match chunk {
            Chunk::Delta(Delta::Text(text)) => response.text.push_str(&text),
            Chunk::Delta(Delta::Reasoning(text)) => self.reasoning.push_str(&text),
            Chunk::Delta(Delta::ToolCall {
                index,
                id,
                name,
                arguments,
            }) => {
                let call = self.call(index);
                if let Some(id) = id {
                    call.id = id;
                }
                if let Some(name) = name {
                    call.name = name;
                }
                call.arguments.push_str(&arguments);
            }
            Chunk::Delta(Delta::HostedTool { .. }) | Chunk::DoomLoop(_) => {}
            Chunk::Started { id, model } => {
                if response.id.is_empty() {
                    (response.id, response.model) = (id, model);
                }
            }
            Chunk::Reasoning(reasoning) => response.reasoning.push(reasoning),
            Chunk::ToolCallDone(index, done) => *self.call(index) = done,
            Chunk::Hosted(item) => response.hosted.push(item),
            Chunk::Usage(usage) => response.usage.merge(usage),
            Chunk::Stop(stop, raw) => {
                (response.stop_reason, response.raw_stop_reason) = (stop, raw);
                self.stopped = true;
            }
            Chunk::Done => self.done = true,
        }
    }

    fn call(&mut self, index: u32) -> &mut ToolCall {
        let at = match self.calls.iter().position(|(i, _)| *i == index) {
            Some(at) => at,
            None => {
                self.calls.push((index, ToolCall::default()));
                self.calls.len() - 1
            }
        };
        &mut self.calls[at].1
    }

    pub fn finish(mut self) -> Response {
        let mut response = self.response;
        if response.reasoning.is_empty() && !self.reasoning.is_empty() {
            response.reasoning.push(Reasoning {
                text: self.reasoning,
                ..Default::default()
            });
        }
        self.calls.sort_by_key(|(index, _)| *index);
        response.tool_calls = self
            .calls
            .into_iter()
            .map(|(_, mut call)| {
                if !is_json(&call.arguments) {
                    let repaired = repair_json(&call.arguments);
                    if is_json(&repaired) {
                        call.arguments = repaired;
                    }
                }
                call
            })
            .collect();
        if response.stop_reason == StopReason::Stop && !response.tool_calls.is_empty() {
            response.stop_reason = StopReason::ToolCalls;
        }
        let usage = &mut response.usage;
        if usage.total == 0 {
            usage.total = usage.prompt + usage.completion;
        }
        response
    }
}

/// Set `body[key]` when there is a value.
fn put(body: &mut Value, key: &str, value: Option<impl Into<Value>>) {
    if let Some(value) = value {
        body[key] = value.into();
    }
}

/// The prompt cache fields of Chat Completions and Responses.
fn openai_cache(body: &mut Value, request: &Request) {
    if request.cache != CacheRetention::Off {
        put(body, "prompt_cache_key", request.prompt_cache_key.clone());
    }
    if request.cache == CacheRetention::Long {
        body["prompt_cache_retention"] = "24h".into();
    }
}

fn text(value: &Value) -> Option<String> {
    value.as_str().map(Into::into)
}

fn is_json(text: &str) -> bool {
    serde_json::from_str::<Value>(text).is_ok()
}

/// Fix two mistakes models make in JSON strings: raw control characters such
/// as newlines, and backslashes before characters that cannot be escaped.
fn repair_json(json: &str) -> String {
    let mut out = String::with_capacity(json.len());
    let mut in_string = false;
    let mut chars = json.chars();
    while let Some(c) = chars.next() {
        if !in_string || c == '"' {
            in_string ^= c == '"';
            out.push(c);
            continue;
        }
        match c {
            '\\' => {
                let rest = chars.as_str();
                let escaped = rest.chars().next().filter(|&next| match next {
                    'u' => rest
                        .get(1..5)
                        .is_some_and(|hex| hex.chars().all(|h| h.is_ascii_hexdigit())),
                    next => "\"\\/bfnrt".contains(next),
                });
                out.push('\\');
                match escaped {
                    Some(next) => {
                        out.push(next);
                        chars.next();
                    }
                    // A lone backslash: escape it.
                    None => out.push('\\'),
                }
            }
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < ' ' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn count(value: &Value) -> u64 {
    value.as_u64().unwrap_or(0)
}

fn stream_error(error: &Value) -> SampleError {
    SampleError::Stream {
        message: clip(
            text(&error["message"])
                .or_else(|| text(error))
                .unwrap_or_else(|| error.to_string()),
        ),
        code: text(&error["code"]).or_else(|| text(&error["type"])),
    }
}

/// `OpenAI`-style tool choice; `function` wraps the name for this backend.
fn openai_tool_choice(choice: &ToolChoice, function: impl Fn(&str) -> Value) -> Value {
    match choice {
        ToolChoice::Auto => "auto".into(),
        ToolChoice::None => "none".into(),
        ToolChoice::Required => "required".into(),
        ToolChoice::Function(name) => function(name),
    }
}

#[cfg(test)]
mod tests {
    use super::{is_json, repair_json};

    #[test]
    fn repairs_raw_newlines_and_bad_escapes() {
        let broken = "{\"cmd\": \"echo a\nb \\d \\u00e9\", \"ok\": \"\\\"q\\\"\"}";
        assert!(!is_json(broken));
        let repaired = repair_json(broken);
        let value: serde_json::Value = serde_json::from_str(&repaired).unwrap();
        assert_eq!(value["cmd"], "echo a\nb \\d \u{e9}");
        assert_eq!(value["ok"], "\"q\"");
    }
}
