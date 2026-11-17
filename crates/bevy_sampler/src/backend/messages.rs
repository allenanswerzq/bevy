//! `POST /messages` (Anthropic).

use bevy_tool::ToolChoice;
use serde_json::{json, Value};

use super::{count, stream_error, text, Chunk};
use crate::{Delta, Item, Part, Reasoning, Request, SampleError, StopReason, Usage};

const DEFAULT_MAX_TOKENS: u64 = 128_000;

pub(super) fn encode(model: &str, request: &Request) -> Value {
    let mut system = Vec::new();
    let mut messages: Vec<Value> = Vec::new();
    // Consecutive blocks from the same role share one message, so thinking,
    // text, and tool calls form one assistant turn and tool results one user turn.
    let mut push = |role: &str, block: Value| match messages.last_mut() {
        Some(last) if last["role"] == role => last["content"].as_array_mut().unwrap().push(block),
        _ => messages.push(json!({"role": role, "content": [block]})),
    };
    for item in &request.items {
        match item {
            Item::System(text) => system.push(json!({"type": "text", "text": text})),
            Item::User(parts) => parts.iter().filter_map(block).for_each(|b| push("user", b)),
            Item::Assistant { text, tool_calls } => {
                if !text.is_empty() {
                    push("assistant", json!({"type": "text", "text": text}));
                }
                for call in tool_calls {
                    let input: Value =
                        serde_json::from_str(&call.arguments).unwrap_or_else(|_| json!({}));
                    push(
                        "assistant",
                        json!({
                            "type": "tool_use",
                            "id": tool_id(&call.id),
                            "name": call.name,
                            "input": input,
                        }),
                    );
                }
            }
            Item::ToolResult {
                call_id,
                content,
                images,
            } => {
                let parts = [Part::Text(content.clone())]
                    .into_iter()
                    .chain(images.iter().cloned().map(Part::Image));
                let content: Vec<_> = parts.filter_map(|p| block(&p)).collect();
                push(
                    "user",
                    json!({
                        "type": "tool_result",
                        "tool_use_id": tool_id(call_id),
                        "content": content,
                    }),
                );
            }
            // Anthropic rejects thinking without its signature.
            Item::Reasoning(Reasoning {
                text,
                signature: Some(signature),
                ..
            }) => push(
                "assistant",
                json!({"type": "thinking", "thinking": text, "signature": signature}),
            ),
            Item::Hosted(block) => push("assistant", block.clone()),
            Item::Reasoning(_) => {}
        }
    }

    // Cache the system prompt, the whole conversation, and the conversation
    // as of the previous turn, so the next request can reuse either prefix.
    let mark = |message: Option<&mut Value>| {
        if let Some(block) = message.and_then(|m| m["content"].as_array_mut()?.last_mut()) {
            block["cache_control"] = json!({"type": "ephemeral"});
        }
    };
    if let Some(last) = system.last_mut() {
        last["cache_control"] = json!({"type": "ephemeral"});
    }
    if let Some(tip) = messages.len().checked_sub(1) {
        let previous_user = messages[..tip]
            .iter()
            .rposition(|m| m["role"] == "assistant")
            .and_then(|a| messages[..a].iter().rposition(|m| m["role"] == "user"));
        if let Some(user) = previous_user {
            mark(messages.get_mut(user));
        }
        mark(messages.get_mut(tip));
    }

    let mut body = json!({
        "model": model,
        "messages": messages,
        "max_tokens": request.max_output_tokens.unwrap_or(DEFAULT_MAX_TOKENS),
        "stream": true,
    });
    if !system.is_empty() {
        body["system"] = system.into();
    }
    let tools: Vec<Value> = request
        .tools
        .iter()
        .map(|t| {
            json!({
                "name": t.name,
                "description": t.description,
                "input_schema": t.parameters,
            })
        })
        .chain(request.hosted_tools.iter().cloned())
        .collect();
    if !tools.is_empty() {
        body["tools"] = tools.into();
        body["tool_choice"] = match &request.tool_choice {
            ToolChoice::Auto => json!({"type": "auto"}),
            ToolChoice::None => json!({"type": "none"}),
            ToolChoice::Required => json!({"type": "any"}),
            ToolChoice::Function(name) => json!({"type": "tool", "name": name}),
        };
    }
    if let Some(effort) = &request.reasoning_effort {
        body["thinking"] = json!({"type": "adaptive", "display": "summarized"});
        body["output_config"]["effort"] = effort.as_str().into();
    }
    if let Some(schema) = &request.json_schema {
        body["output_config"]["format"] = json!({"type": "json_schema", "schema": schema});
    }
    super::put(&mut body, "temperature", request.temperature);
    super::put(&mut body, "top_p", request.top_p);
    body
}

fn block(part: &Part) -> Option<Value> {
    match part {
        Part::Text(text) if text.is_empty() => None,
        Part::Text(text) => Some(json!({"type": "text", "text": text})),
        Part::Image(url) => Some(
            match url
                .strip_prefix("data:")
                .and_then(|d| d.split_once(";base64,"))
            {
                Some((media_type, data)) => json!({
                    "type": "image",
                    "source": {"type": "base64", "media_type": media_type, "data": data},
                }),
                None => json!({"type": "image", "source": {"type": "url", "url": url}}),
            },
        ),
    }
}

/// Anthropic ids allow only `[a-zA-Z0-9_-]`.
fn tool_id(id: &str) -> String {
    id.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// The content block being streamed.
pub(crate) enum Block {
    None,
    Tool,
    Thinking(Reasoning),
    /// A block replayed verbatim, such as `server_tool_use`,
    /// `web_search_tool_result`, or `redacted_thinking`. `input` collects its
    /// streamed JSON input.
    Hosted {
        block: Value,
        input: String,
    },
}

pub(super) fn decode(
    block: &mut Block,
    event: &Value,
    out: &mut Vec<Chunk>,
) -> Result<(), SampleError> {
    let index = count(&event["index"]) as u32;
    let delta = &event["delta"];
    match event["type"].as_str().unwrap_or_default() {
        "message_start" => {
            let message = &event["message"];
            out.push(Chunk::Started {
                id: text(&message["id"]).unwrap_or_default(),
                model: text(&message["model"]).unwrap_or_default(),
            });
            out.push(Chunk::Usage(usage(&message["usage"])));
        }
        "content_block_start" => {
            let start = &event["content_block"];
            *block = match start["type"].as_str().unwrap_or_default() {
                "text" => Block::None,
                "thinking" => Block::Thinking(Reasoning::default()),
                "tool_use" => {
                    out.push(Chunk::Delta(Delta::ToolCall {
                        index,
                        id: text(&start["id"]),
                        name: text(&start["name"]),
                        arguments: String::new(),
                    }));
                    Block::Tool
                }
                _ => {
                    out.extend(hosted_tool(start, false));
                    Block::Hosted {
                        block: start.clone(),
                        input: String::new(),
                    }
                }
            };
        }
        "content_block_delta" => match (delta["type"].as_str().unwrap_or_default(), &mut *block) {
            ("text_delta", _) => out.push(Chunk::Delta(Delta::Text(
                text(&delta["text"]).unwrap_or_default(),
            ))),
            ("thinking_delta", Block::Thinking(thinking)) => {
                let piece = text(&delta["thinking"]).unwrap_or_default();
                thinking.text.push_str(&piece);
                out.push(Chunk::Delta(Delta::Reasoning(piece)));
            }
            ("signature_delta", Block::Thinking(thinking)) => {
                thinking
                    .signature
                    .get_or_insert_default()
                    .push_str(delta["signature"].as_str().unwrap_or_default());
            }
            ("input_json_delta", Block::Tool) => out.push(Chunk::Delta(Delta::ToolCall {
                index,
                id: None,
                name: None,
                arguments: text(&delta["partial_json"]).unwrap_or_default(),
            })),
            ("input_json_delta", Block::Hosted { input, .. }) => {
                input.push_str(delta["partial_json"].as_str().unwrap_or_default());
            }
            _ => {}
        },
        "content_block_stop" => match core::mem::replace(block, Block::None) {
            Block::Thinking(reasoning) => out.push(Chunk::Reasoning(reasoning)),
            Block::Hosted { mut block, input } => {
                if let Ok(input) = serde_json::from_str(&input) {
                    block["input"] = input;
                }
                out.extend(hosted_tool(&block, true));
                out.push(Chunk::Hosted(block));
            }
            Block::None | Block::Tool => {}
        },
        "message_delta" => {
            if let Some(reason) = delta["stop_reason"].as_str() {
                out.push(Chunk::Stop(match reason {
                    "max_tokens" => StopReason::Length,
                    "tool_use" => StopReason::ToolCalls,
                    "refusal" => StopReason::ContentFilter,
                    _ => StopReason::Stop,
                }));
            }
            out.push(Chunk::Usage(usage(&event["usage"])));
        }
        "message_stop" => out.push(Chunk::Done),
        "error" => return Err(stream_error(&event["error"])),
        _ => {} // ping
    }
    Ok(())
}

/// Progress for server tool calls such as `server_tool_use` or `mcp_tool_use`.
fn hosted_tool(block: &Value, done: bool) -> Option<Chunk> {
    let kind = block["type"].as_str()?;
    kind.ends_with("tool_use").then(|| {
        Chunk::Delta(Delta::HostedTool {
            id: text(&block["id"]).unwrap_or_default(),
            name: text(&block["name"]).unwrap_or_else(|| kind.into()),
            done,
        })
    })
}

fn usage(usage: &Value) -> Usage {
    let (input, read, created) = (
        count(&usage["input_tokens"]),
        count(&usage["cache_read_input_tokens"]),
        count(&usage["cache_creation_input_tokens"]),
    );
    Usage {
        prompt: input + read + created,
        completion: count(&usage["output_tokens"]),
        cached_prompt: read,
        cache_creation: created,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use bevy_tool::ToolCall;
    use serde_json::json;

    use crate::{
        backend::{encode, Builder, Decoder},
        Backend, Item, Reasoning, Request, SamplerSettings, StopReason,
    };

    #[test]
    fn decodes_thinking_text_and_tools() {
        let events = [
            json!({
                "type": "message_start",
                "message": {
                    "id": "m1",
                    "model": "claude",
                    "usage": {"input_tokens": 3, "cache_read_input_tokens": 7},
                },
            }),
            json!({
                "type": "content_block_start",
                "index": 0,
                "content_block": {"type": "thinking"},
            }),
            json!({
                "type": "content_block_delta",
                "index": 0,
                "delta": {"type": "thinking_delta", "thinking": "hmm"},
            }),
            json!({
                "type": "content_block_delta",
                "index": 0,
                "delta": {"type": "signature_delta", "signature": "sig"},
            }),
            json!({"type": "content_block_stop", "index": 0}),
            json!({"type": "ping"}),
            json!({"type": "content_block_start", "index": 1, "content_block": {"type": "text"}}),
            json!({
                "type": "content_block_delta",
                "index": 1,
                "delta": {"type": "text_delta", "text": "ok"},
            }),
            json!({
                "type": "content_block_start",
                "index": 2,
                "content_block": {"type": "tool_use", "id": "t1", "name": "ls"},
            }),
            json!({
                "type": "content_block_delta",
                "index": 2,
                "delta": {"type": "input_json_delta", "partial_json": "{}"},
            }),
            json!({"type": "content_block_stop", "index": 2}),
            json!({
                "type": "content_block_start",
                "index": 3,
                "content_block": {
                    "type": "server_tool_use",
                    "id": "s1",
                    "name": "web_search",
                    "input": {},
                },
            }),
            json!({
                "type": "content_block_delta",
                "index": 3,
                "delta": {"type": "input_json_delta", "partial_json": r#"{"query":"x"}"#},
            }),
            json!({"type": "content_block_stop", "index": 3}),
            json!({
                "type": "content_block_start",
                "index": 4,
                "content_block": {
                    "type": "web_search_tool_result",
                    "tool_use_id": "s1",
                    "content": [],
                },
            }),
            json!({"type": "content_block_stop", "index": 4}),
            json!({
                "type": "message_delta",
                "delta": {"stop_reason": "tool_use"},
                "usage": {"output_tokens": 5},
            }),
            json!({"type": "message_stop"}),
        ];
        let mut decoder = Decoder::new(Backend::Messages);
        let mut builder = Builder::default();
        for event in events {
            decoder
                .decode(&event.to_string())
                .unwrap()
                .into_iter()
                .for_each(|c| builder.push(c));
        }
        assert!(builder.done);
        let response = builder.finish();
        assert_eq!(
            response.reasoning,
            [Reasoning {
                id: None,
                text: "hmm".into(),
                signature: Some("sig".into())
            }]
        );
        assert_eq!(response.text, "ok");
        assert_eq!(response.tool_calls[0].id, "t1");
        assert_eq!(response.stop_reason, StopReason::ToolCalls);
        assert_eq!(
            (
                response.usage.prompt,
                response.usage.completion,
                response.usage.cached_prompt
            ),
            (10, 5, 7)
        );
        assert_eq!(response.hosted[0]["input"]["query"], "x");

        // The whole turn replays as one assistant message, hosted blocks included.
        let settings = SamplerSettings {
            backend: Backend::Messages,
            ..Default::default()
        };
        let body = encode(&settings, &Request::new(response.items()));
        let types: Vec<_> = body["messages"][0]["content"]
            .as_array()
            .unwrap()
            .iter()
            .map(|b| b["type"].as_str().unwrap())
            .collect();
        assert_eq!(
            types,
            [
                "thinking",
                "server_tool_use",
                "web_search_tool_result",
                "text",
                "tool_use"
            ]
        );
    }

    #[test]
    fn encodes_turns_and_cache_breakpoints() {
        let settings = SamplerSettings {
            backend: Backend::Messages,
            ..Default::default()
        };
        let call = ToolCall {
            id: "call.1".into(),
            name: "ls".into(),
            arguments: "{\"a\":1}".into(),
        };
        let request = Request::new(vec![
            Item::system("be brief"),
            Item::user("list files"),
            Item::Reasoning(Reasoning {
                id: None,
                text: "hm".into(),
                signature: Some("s".into()),
            }),
            Item::Assistant {
                text: String::new(),
                tool_calls: vec![call],
            },
            Item::ToolResult {
                call_id: "call.1".into(),
                content: "a.txt".into(),
                images: vec![],
            },
            Item::ToolResult {
                call_id: "call.1".into(),
                content: "b.txt".into(),
                images: vec![],
            },
        ]);
        let body = encode(&settings, &request);
        let ephemeral = json!({"type": "ephemeral"});
        assert_eq!(body["system"][0]["cache_control"], ephemeral);
        let messages = body["messages"].as_array().unwrap();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0]["content"][0]["cache_control"], ephemeral);
        assert_eq!(messages[1]["content"][0]["type"], "thinking");
        assert_eq!(messages[1]["content"][1]["input"], json!({"a": 1}));
        assert_eq!(messages[1]["content"][1]["id"], "call_1");
        assert_eq!(messages[2]["content"].as_array().unwrap().len(), 2);
        assert_eq!(messages[2]["content"][1]["cache_control"], ephemeral);
        assert!(messages[2]["content"][0].get("cache_control").is_none());
    }
}
