//! `POST /chat/completions`.

use serde_json::{json, Value};

use super::{count, openai_tool_choice, put, stream_error, text, Chunk};
use crate::{Delta, Item, Part, Request, SampleError, StopReason, Usage};

pub(super) fn encode(model: &str, request: &Request) -> Value {
    let mut messages = Vec::new();
    for item in &request.items {
        match item {
            Item::System(text) => messages.push(json!({"role": "system", "content": text})),
            Item::User(parts) => messages.push(json!({"role": "user", "content": content(parts)})),
            Item::Assistant { text, tool_calls } => {
                let mut message = json!({"role": "assistant", "content": text});
                if !tool_calls.is_empty() {
                    message["tool_calls"] = tool_calls
                        .iter()
                        .map(|c| {
                            json!({
                                "id": c.id,
                                "type": "function",
                                "function": {"name": c.name, "arguments": c.arguments},
                            })
                        })
                        .collect();
                }
                messages.push(message);
            }
            Item::ToolResult {
                call_id,
                content: output,
                images,
            } => {
                messages.push(json!({"role": "tool", "tool_call_id": call_id, "content": output}));
                // Tool messages are text only; send images as a user turn.
                if !images.is_empty() {
                    let parts: Vec<_> = images.iter().cloned().map(Part::Image).collect();
                    messages.push(json!({"role": "user", "content": content(&parts)}));
                }
            }
            Item::Reasoning(_) | Item::Hosted(_) => {}
        }
    }

    let mut body = json!({
        "model": model,
        "messages": messages,
        "stream": true,
        "stream_options": {"include_usage": true},
    });
    if !request.tools.is_empty() {
        body["tools"] = request
            .tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters,
                    },
                })
            })
            .collect();
        body["tool_choice"] = openai_tool_choice(
            &request.tool_choice,
            |name| json!({"type": "function", "function": {"name": name}}),
        );
    }
    put(
        &mut body,
        "max_completion_tokens",
        request.max_output_tokens,
    );
    put(&mut body, "temperature", request.temperature);
    put(&mut body, "top_p", request.top_p);
    put(
        &mut body,
        "reasoning_effort",
        request.reasoning_effort.clone(),
    );
    put(
        &mut body,
        "prompt_cache_key",
        request.prompt_cache_key.clone(),
    );
    if let Some(schema) = &request.json_schema {
        body["response_format"] = json!({
            "type": "json_schema",
            "json_schema": {"name": "structured_output", "schema": schema, "strict": true},
        });
    }
    body
}

fn content(parts: &[Part]) -> Value {
    match parts {
        [Part::Text(text)] => text.as_str().into(),
        parts => parts
            .iter()
            .map(|part| match part {
                Part::Text(text) => json!({"type": "text", "text": text}),
                Part::Image(url) => json!({"type": "image_url", "image_url": {"url": url}}),
            })
            .collect(),
    }
}

pub(super) fn decode(event: &Value, out: &mut Vec<Chunk>) -> Result<(), SampleError> {
    if let Some(error) = event.get("error") {
        return Err(stream_error(error));
    }
    if let (Some(id), Some(model)) = (text(&event["id"]), text(&event["model"])) {
        out.push(Chunk::Started { id, model });
    }
    let choice = &event["choices"][0];
    let delta = &choice["delta"];
    for (field, wrap) in [
        ("content", Delta::Text as fn(String) -> Delta),
        ("reasoning_content", Delta::Reasoning),
    ] {
        if let Some(text) = delta[field].as_str().filter(|t| !t.is_empty()) {
            out.push(Chunk::Delta(wrap(text.into())));
        }
    }
    for call in delta["tool_calls"].as_array().into_iter().flatten() {
        out.push(Chunk::Delta(Delta::ToolCall {
            index: count(&call["index"]) as u32,
            id: text(&call["id"]),
            name: text(&call["function"]["name"]),
            arguments: text(&call["function"]["arguments"]).unwrap_or_default(),
        }));
    }
    if let Some(reason) = choice["finish_reason"].as_str() {
        out.push(Chunk::Stop(match reason {
            "length" => StopReason::Length,
            "tool_calls" | "function_call" => StopReason::ToolCalls,
            "content_filter" => StopReason::ContentFilter,
            _ => StopReason::Stop,
        }));
    }
    let usage = &event["usage"];
    if usage.is_object() {
        out.push(Chunk::Usage(Usage {
            prompt: count(&usage["prompt_tokens"]),
            completion: count(&usage["completion_tokens"]),
            total: count(&usage["total_tokens"]),
            reasoning: count(&usage["completion_tokens_details"]["reasoning_tokens"]),
            cached_prompt: count(&usage["prompt_tokens_details"]["cached_tokens"]),
            cache_creation: 0,
            cost_usd_ticks: usage["cost_in_usd_ticks"].as_i64(),
        }));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::{
        backend::{Builder, Decoder},
        Backend, StopReason,
    };

    #[test]
    fn decodes_text_tool_calls_and_usage() {
        let events = [
            json!({
                "id": "r1",
                "model": "m",
                "choices": [{"delta": {"reasoning_content": "think"}}],
            }),
            json!({"id": "r1", "model": "m", "choices": [{"delta": {"content": "Hi"}}]}),
            json!({
                "id": "r1",
                "model": "m",
                "choices": [{
                    "delta": {
                        "tool_calls": [{
                            "index": 0,
                            "id": "c1",
                            "function": {"name": "ls", "arguments": r#"{"a""#},
                        }],
                    },
                }],
            }),
            json!({
                "id": "r1",
                "model": "m",
                "choices": [{
                    "delta": {"tool_calls": [{"index": 0, "function": {"arguments": ":1}"}}]},
                    "finish_reason": "tool_calls",
                }],
            }),
            json!({
                "id": "r1",
                "model": "m",
                "choices": [],
                "usage": {"prompt_tokens": 5, "completion_tokens": 3, "total_tokens": 8},
            }),
        ];
        let mut decoder = Decoder::new(Backend::ChatCompletions);
        let mut builder = Builder::default();
        let events = events.iter().map(Value::to_string);
        for event in events.chain(["[DONE]".into()]) {
            decoder
                .decode(&event)
                .unwrap()
                .into_iter()
                .for_each(|c| builder.push(c));
        }
        assert!(builder.done);
        let response = builder.finish();
        assert_eq!((response.id.as_str(), response.text.as_str()), ("r1", "Hi"));
        assert_eq!(response.reasoning[0].text, "think");
        assert_eq!(response.tool_calls[0].arguments, r#"{"a":1}"#);
        assert_eq!(response.stop_reason, StopReason::ToolCalls);
        assert_eq!(response.usage.total, 8);
    }
}
