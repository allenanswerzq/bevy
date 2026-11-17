//! Provider-neutral conversation types. Backends translate these to and from
//! their wire formats, so game code only ever sees this module.

use std::time::Duration;

use serde_json::Value;

/// Text that replaces an image when a request is retried without images.
pub const IMAGE_REMOVED: &str = "[image removed]";

/// One entry in a conversation.
#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    /// Instructions from the developer.
    System(String),
    /// A user turn.
    User(Vec<Part>),
    /// A model turn.
    Assistant {
        /// Visible text.
        text: String,
        /// Tools the model asked to call.
        tool_calls: Vec<ToolCall>,
    },
    /// The output of one tool call.
    ToolResult {
        /// The [`ToolCall::id`] this answers.
        call_id: String,
        /// Text output.
        content: String,
        /// Image URLs or `data:` URLs produced by the tool.
        images: Vec<String>,
    },
    /// Reasoning to replay so the model keeps its chain of thought.
    Reasoning(Reasoning),
    /// A provider-native item replayed verbatim to the backend that produced
    /// it: Responses server tool items, or Anthropic `server_tool_use`,
    /// tool result, and `redacted_thinking` blocks. Chat Completions skips it.
    Hosted(Value),
}

impl Item {
    /// A user turn with plain text.
    pub fn user(text: impl Into<String>) -> Self {
        Self::User(vec![Part::Text(text.into())])
    }

    /// A system instruction.
    pub fn system(text: impl Into<String>) -> Self {
        Self::System(text.into())
    }
}

/// A piece of user content.
#[derive(Clone, Debug, PartialEq)]
pub enum Part {
    /// Plain text.
    Text(String),
    /// An image URL or `data:` URL.
    Image(String),
}

/// A tool call made by the model.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToolCall {
    /// Id that the matching [`Item::ToolResult`] refers to.
    pub id: String,
    /// Tool name.
    pub name: String,
    /// Arguments as a JSON string.
    pub arguments: String,
}

/// A function the model may call.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolSpec {
    /// Tool name.
    pub name: String,
    /// What the tool does, for the model.
    pub description: String,
    /// JSON schema of the arguments.
    pub parameters: Value,
}

/// Whether and which tools the model must call.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum ToolChoice {
    /// The model decides.
    #[default]
    Auto,
    /// No tool calls.
    None,
    /// At least one tool call.
    Required,
    /// Call this tool.
    Function(String),
}

/// Model reasoning.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Reasoning {
    /// Provider id of the reasoning item, if any.
    pub id: Option<String>,
    /// Reasoning or summary text.
    pub text: String,
    /// Opaque proof needed to replay it: the Anthropic signature or the
    /// `OpenAI`/xAI `encrypted_content`.
    pub signature: Option<String>,
}

/// What to do when the model hits its output limit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LengthPolicy {
    /// Always fail with [`SampleError::Truncated`](crate::SampleError::Truncated).
    Fail,
    /// Keep the response if every tool call has complete JSON arguments.
    #[default]
    CompleteToolCalls,
    /// Also keep partial text.
    CompletePartial,
}

/// Why the model stopped.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StopReason {
    /// Natural end of the turn.
    #[default]
    Stop,
    /// The output limit was reached.
    Length,
    /// The model is waiting for tool results.
    ToolCalls,
    /// The provider filtered the output.
    ContentFilter,
}

/// Token accounting.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Usage {
    /// Input tokens, including cached ones.
    pub prompt: u64,
    /// Output tokens, including reasoning.
    pub completion: u64,
    /// Prompt plus completion.
    pub total: u64,
    /// Reasoning tokens inside `completion`.
    pub reasoning: u64,
    /// Input tokens served from the prompt cache.
    pub cached_prompt: u64,
    /// Input tokens written to the prompt cache.
    pub cache_creation: u64,
    /// Cost reported by xAI, in 1e-10 USD.
    pub cost_usd_ticks: Option<i64>,
}

impl Usage {
    /// Overwrite every field that `other` reports.
    pub(crate) fn merge(&mut self, other: Usage) {
        for (mine, theirs) in [
            (&mut self.prompt, other.prompt),
            (&mut self.completion, other.completion),
            (&mut self.total, other.total),
            (&mut self.reasoning, other.reasoning),
            (&mut self.cached_prompt, other.cached_prompt),
            (&mut self.cache_creation, other.cache_creation),
        ] {
            if theirs != 0 {
                *mine = theirs;
            }
        }
        self.cost_usd_ticks = other.cost_usd_ticks.or(self.cost_usd_ticks);
    }
}

/// Everything sent for one sample.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Request {
    /// The conversation so far.
    pub items: Vec<Item>,
    /// Functions the model may call.
    pub tools: Vec<ToolSpec>,
    /// Provider tools passed through as JSON, such as `{"type":"web_search"}`.
    pub hosted_tools: Vec<Value>,
    /// Tool policy.
    pub tool_choice: ToolChoice,
    /// Overrides [`SamplerSettings::model`](crate::SamplerSettings::model).
    pub model: Option<String>,
    /// Sampling temperature.
    pub temperature: Option<f32>,
    /// Nucleus sampling.
    pub top_p: Option<f32>,
    /// Output limit.
    pub max_output_tokens: Option<u64>,
    /// Reasoning effort such as `"low"` or `"high"`.
    pub reasoning_effort: Option<String>,
    /// Force the output to match this JSON schema.
    pub json_schema: Option<Value>,
    /// Routes requests with the same key to the same prompt cache.
    pub prompt_cache_key: Option<String>,
    /// What to do on [`StopReason::Length`].
    pub length_policy: LengthPolicy,
    /// Extra headers for this request only.
    pub headers: Vec<(String, String)>,
}

impl Request {
    /// A request with these items and default settings.
    pub fn new(items: Vec<Item>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    /// Replace every image with [`IMAGE_REMOVED`] and return how many were removed.
    pub fn strip_images(&mut self) -> usize {
        let mut removed = 0;
        for item in &mut self.items {
            match item {
                Item::User(parts) => {
                    for part in parts.iter_mut().filter(|p| matches!(p, Part::Image(_))) {
                        *part = Part::Text(IMAGE_REMOVED.into());
                        removed += 1;
                    }
                }
                Item::ToolResult {
                    content, images, ..
                } if !images.is_empty() => {
                    removed += images.len();
                    images.clear();
                    if !content.is_empty() {
                        content.push('\n');
                    }
                    content.push_str(IMAGE_REMOVED);
                }
                _ => {}
            }
        }
        removed
    }
}

/// A finished sample.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Response {
    /// Provider response id.
    pub id: String,
    /// Model that answered.
    pub model: String,
    /// Visible text.
    pub text: String,
    /// Reasoning items, in order.
    pub reasoning: Vec<Reasoning>,
    /// Tool calls, in order.
    pub tool_calls: Vec<ToolCall>,
    /// Provider-native items; see [`Item::Hosted`].
    pub hosted: Vec<Value>,
    /// Why the model stopped.
    pub stop_reason: StopReason,
    /// Token accounting.
    pub usage: Usage,
    /// Model limits reported in response headers.
    pub metadata: ModelMetadata,
    /// Timing of the whole sample, retries included.
    pub metrics: Metrics,
}

impl Response {
    /// The items to append to the conversation before the next turn.
    pub fn items(&self) -> Vec<Item> {
        let assistant = Item::Assistant {
            text: self.text.clone(),
            tool_calls: self.tool_calls.clone(),
        };
        self.reasoning
            .iter()
            .cloned()
            .map(Item::Reasoning)
            .chain(self.hosted.iter().cloned().map(Item::Hosted))
            .chain([assistant])
            .collect()
    }
}

/// Model limits from the `x-grok-*` and `x-models-etag` response headers.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModelMetadata {
    /// Context window in tokens.
    pub context_window: Option<u64>,
    /// Largest allowed output in tokens.
    pub max_completion_tokens: Option<u64>,
    /// Version of the server's model list.
    pub models_etag: Option<String>,
}

/// Timing of a sample.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Metrics {
    /// HTTP attempts made, including the successful one.
    pub attempts: u32,
    /// From the start of the last attempt to its first output.
    pub time_to_first_token: Option<Duration>,
    /// From the first dispatch to completion.
    pub duration: Duration,
}

/// One streamed piece of output.
#[derive(Clone, Debug, PartialEq)]
pub enum Delta {
    /// Visible text.
    Text(String),
    /// Reasoning text.
    Reasoning(String),
    /// Part of a tool call. `id` and `name` arrive once; `arguments` accumulates.
    ToolCall {
        /// Position of the call in the response.
        index: u32,
        /// Call id.
        id: Option<String>,
        /// Tool name.
        name: Option<String>,
        /// Next piece of the JSON arguments.
        arguments: String,
    },
    /// A server-side tool started or finished.
    HostedTool {
        /// Item id.
        id: String,
        /// Item type or tool name, such as `web_search_call`.
        name: String,
        /// Whether it finished.
        done: bool,
    },
}
