//! Wire formats. Each backend encodes a [`Request`] into JSON and decodes
//! stream events into [`Chunk`]s; [`Builder`] assembles the chunks into a
//! [`Response`] the same way for all of them.

mod chat;
mod messages;
mod responses;

use bevy_tool::{ToolCall, ToolChoice};
use serde_json::Value;

use crate::{
    error::clip, Backend, Delta, Reasoning, Request, Response, SampleError, SamplerSettings,
    StopReason, Usage,
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
    Stop(StopReason),
    DoomLoop(Vec<String>),
    Done,
}

pub(crate) fn encode(settings: &SamplerSettings, request: &Request) -> Value {
    let model = request.model.as_deref().unwrap_or(&settings.model);
    match settings.backend {
        Backend::ChatCompletions => chat::encode(model, request),
        Backend::Responses => responses::encode(model, request),
        Backend::Messages => messages::encode(model, request),
    }
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
            Chunk::Stop(stop) => {
                response.stop_reason = stop;
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
        response.tool_calls = self.calls.into_iter().map(|(_, call)| call).collect();
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

fn text(value: &Value) -> Option<String> {
    value.as_str().map(Into::into)
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
