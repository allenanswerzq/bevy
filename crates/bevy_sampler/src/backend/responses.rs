//! `POST /responses`.

use bevy_tool::ToolCall;
use serde_json::{json, Value};

use super::{count, openai_tool_choice, put, stream_error, text, Chunk};
use crate::{Delta, Item, Part, Reasoning, Request, SampleError, StopReason, Usage};

pub(super) fn encode(model: &str, request: &Request) -> Value {
    let message =
        |role: &str, content: Value| json!({"type": "message", "role": role, "content": content});
    let mut input = Vec::new();
    for item in &request.items {
        match item {
            Item::System(text) => input.push(message("system", text.as_str().into())),
            Item::User(parts) => input.push(message("user", content(parts))),
            Item::Assistant { text, tool_calls } => {
                if !text.is_empty() {
                    input.push(message("assistant", text.as_str().into()));
                }
                input.extend(tool_calls.iter().map(|c| {
                    json!({
                        "type": "function_call",
                        "call_id": c.id,
                        "name": c.name,
                        "arguments": c.arguments,
                    })
                }));
            }
            Item::ToolResult {
                call_id,
                content: output,
                images,
            } => {
                let output = if images.is_empty() {
                    output.as_str().into()
                } else {
                    let parts: Vec<_> = [Part::Text(output.clone())]
                        .into_iter()
                        .chain(images.iter().cloned().map(Part::Image))
                        .collect();
                    content(&parts)
                };
                input.push(json!({
                    "type": "function_call_output",
                    "call_id": call_id,
                    "output": output,
                }));
            }
            Item::Reasoning(reasoning) => {
                let mut item = json!({
                    "type": "reasoning",
                    "summary": [],
                    "content": [{"type": "reasoning_text", "text": reasoning.text}],
                });
                put(&mut item, "id", reasoning.id.clone());
                put(&mut item, "encrypted_content", reasoning.signature.clone());
                input.push(item);
            }
            Item::Hosted(item) => input.push(item.clone()),
        }
    }

    let mut body = json!({
        "model": model,
        "input": input,
        "stream": true,
        "store": false,
        "include": ["reasoning.encrypted_content"],
    });
    let tools: Vec<Value> = request
        .tools
        .iter()
        .map(|t| {
            json!({
                "type": "function",
                "name": t.name,
                "description": t.description,
                "parameters": t.parameters,
            })
        })
        .chain(request.hosted_tools.iter().cloned())
        .collect();
    if !tools.is_empty() {
        body["tools"] = tools.into();
        body["tool_choice"] = openai_tool_choice(
            &request.tool_choice,
            |name| json!({"type": "function", "name": name}),
        );
    }
    if let Some(effort) = &request.reasoning_effort {
        body["reasoning"] = json!({"effort": effort, "summary": "concise"});
    }
    if let Some(schema) = &request.json_schema {
        body["text"] = json!({
            "format": {
                "type": "json_schema",
                "name": "structured_output",
                "schema": schema,
                "strict": true,
            },
        });
    }
    put(&mut body, "max_output_tokens", request.max_output_tokens);
    put(&mut body, "temperature", request.temperature);
    put(&mut body, "top_p", request.top_p);
    put(
        &mut body,
        "prompt_cache_key",
        request.prompt_cache_key.clone(),
    );
    body
}

fn content(parts: &[Part]) -> Value {
    match parts {
        [Part::Text(text)] => text.as_str().into(),
        parts => parts
            .iter()
            .map(|part| match part {
                Part::Text(text) => json!({"type": "input_text", "text": text}),
                Part::Image(url) => {
                    json!({"type": "input_image", "image_url": url, "detail": "auto"})
                }
            })
            .collect(),
    }
}

pub(super) fn decode(event: &Value, out: &mut Vec<Chunk>) -> Result<(), SampleError> {
    let item = &event["item"];
    let index = count(&event["output_index"]) as u32;
    let delta = || text(&event["delta"]).unwrap_or_default();
    let hosted = !matches!(
        item["type"].as_str(),
        Some("message" | "reasoning" | "function_call") | None
    );
    let hosted_delta = |done| {
        Chunk::Delta(Delta::HostedTool {
            id: text(&item["id"]).unwrap_or_default(),
            name: text(&item["type"]).unwrap_or_default(),
            done,
        })
    };

    match event["type"].as_str().unwrap_or_default() {
        "response.created" => out.push(Chunk::Started {
            id: text(&event["response"]["id"]).unwrap_or_default(),
            model: text(&event["response"]["model"]).unwrap_or_default(),
        }),
        "response.output_text.delta" => out.push(Chunk::Delta(Delta::Text(delta()))),
        "response.reasoning_text.delta" | "response.reasoning_summary_text.delta" => {
            out.push(Chunk::Delta(Delta::Reasoning(delta())));
        }
        "response.function_call_arguments.delta" => out.push(Chunk::Delta(Delta::ToolCall {
            index,
            id: None,
            name: None,
            arguments: delta(),
        })),
        "response.output_item.added" if item["type"] == "function_call" => {
            out.push(Chunk::Delta(Delta::ToolCall {
                index,
                id: text(&item["call_id"]),
                name: text(&item["name"]),
                arguments: String::new(),
            }));
        }
        "response.output_item.added" if hosted => out.push(hosted_delta(false)),
        "response.output_item.done" => match item["type"].as_str() {
            Some("reasoning") => out.push(Chunk::Reasoning(reasoning(item))),
            Some("function_call") => out.push(Chunk::ToolCallDone(
                index,
                ToolCall {
                    id: text(&item["call_id"]).unwrap_or_default(),
                    name: text(&item["name"]).unwrap_or_default(),
                    arguments: text(&item["arguments"]).unwrap_or_default(),
                },
            )),
            _ if hosted => out.extend([Chunk::Hosted(item.clone()), hosted_delta(true)]),
            _ => {}
        },
        "response.completed" | "response.incomplete" => {
            let response = &event["response"];
            let usage = &response["usage"];
            out.push(Chunk::Usage(Usage {
                prompt: count(&usage["input_tokens"]),
                completion: count(&usage["output_tokens"]),
                total: count(&usage["total_tokens"]),
                reasoning: count(&usage["output_tokens_details"]["reasoning_tokens"]),
                cached_prompt: count(&usage["input_tokens_details"]["cached_tokens"]),
                cache_creation: 0,
                cost_usd_ticks: usage["cost_in_usd_ticks"].as_i64(),
            }));
            out.push(Chunk::Stop(
                match response["incomplete_details"]["reason"].as_str() {
                    Some("max_output_tokens") => StopReason::Length,
                    Some("content_filter") => StopReason::ContentFilter,
                    _ => StopReason::Stop,
                },
            ));
            doom_loop(&response["doom_loop_check"], out);
            out.push(Chunk::Done);
        }
        "response.doom_loop_check" => doom_loop(&event["doom_loop_check"], out),
        "response.failed" => return Err(stream_error(&event["response"]["error"])),
        "error" => return Err(stream_error(event.get("error").unwrap_or(event))),
        _ => {}
    }
    Ok(())
}

fn reasoning(item: &Value) -> Reasoning {
    let joined = |field: &str| -> String {
        item[field]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|p| p["text"].as_str())
            .collect()
    };
    let content = joined("content");
    Reasoning {
        id: text(&item["id"]),
        text: if content.is_empty() {
            joined("summary")
        } else {
            content
        },
        signature: text(&item["encrypted_content"]),
    }
}

fn doom_loop(check: &Value, out: &mut Vec<Chunk>) {
    if let Some(triggers) = check["triggers"].as_array().filter(|t| !t.is_empty()) {
        out.push(Chunk::DoomLoop(triggers.iter().filter_map(text).collect()));
    }
}

#[cfg(test)]
mod tests {
    use bevy_tool::ToolCall;
    use serde_json::json;

    use crate::{
        backend::{encode, Builder, Chunk, Decoder},
        Backend, Item, Reasoning, Request, SamplerSettings, StopReason,
    };

    #[test]
    fn decodes_reasoning_calls_hosted_and_doom_loop() {
        let events = [
            json!({"type": "response.created", "response": {"id": "r1", "model": "grok"}}),
            json!({"type": "response.reasoning_summary_text.delta", "delta": "hm"}),
            json!({
                "type": "response.output_item.done",
                "output_index": 0,
                "item": {
                    "type": "reasoning",
                    "id": "rs",
                    "summary": [{"text": "hm"}],
                    "encrypted_content": "enc",
                },
            }),
            json!({
                "type": "response.output_item.added",
                "output_index": 1,
                "item": {"type": "web_search_call", "id": "ws"},
            }),
            json!({
                "type": "response.output_item.done",
                "output_index": 1,
                "item": {"type": "web_search_call", "id": "ws"},
            }),
            json!({
                "type": "response.output_item.added",
                "output_index": 2,
                "item": {"type": "function_call", "call_id": "c1", "name": "ls"},
            }),
            json!({
                "type": "response.function_call_arguments.delta",
                "output_index": 2,
                "delta": "{}",
            }),
            json!({"type": "response.output_text.delta", "delta": "ok"}),
            json!({
                "type": "response.completed",
                "response": {
                    "usage": {"input_tokens": 4, "output_tokens": 2},
                    "doom_loop_check": {"triggers": ["tail_repetition:3@thinking"]},
                },
            }),
        ];
        let mut decoder = Decoder::new(Backend::Responses);
        let mut builder = Builder::default();
        let mut doom = Vec::new();
        for event in events {
            for chunk in decoder.decode(&event.to_string()).unwrap() {
                if let Chunk::DoomLoop(t) = &chunk {
                    doom.clone_from(t);
                }
                builder.push(chunk);
            }
        }
        assert_eq!(doom, ["tail_repetition:3@thinking"]);
        let response = builder.finish();
        assert_eq!(
            response.reasoning,
            [Reasoning {
                id: Some("rs".into()),
                text: "hm".into(),
                signature: Some("enc".into())
            }]
        );
        assert_eq!(response.hosted.len(), 1);
        assert_eq!(
            response.tool_calls,
            [ToolCall {
                id: "c1".into(),
                name: "ls".into(),
                arguments: "{}".into()
            }]
        );
        assert_eq!(
            (response.text.as_str(), response.stop_reason),
            ("ok", StopReason::ToolCalls)
        );
        assert_eq!(response.usage.total, 6);
        assert_eq!(response.items().len(), 3);
    }

    #[test]
    fn encodes_tool_round_trip() {
        let settings = SamplerSettings {
            backend: Backend::Responses,
            ..Default::default()
        };
        let call = ToolCall {
            id: "c1".into(),
            name: "ls".into(),
            arguments: "{}".into(),
        };
        let request = Request::new(vec![
            Item::user("hi"),
            Item::Assistant {
                text: String::new(),
                tool_calls: vec![call],
            },
            Item::ToolResult {
                call_id: "c1".into(),
                content: "a.txt".into(),
                images: vec![],
            },
        ]);
        let body = encode(&settings, &request);
        assert_eq!(
            body["input"][1],
            json!({"type": "function_call", "call_id": "c1", "name": "ls", "arguments": "{}"})
        );
        assert_eq!(
            body["input"][2],
            json!({"type": "function_call_output", "call_id": "c1", "output": "a.txt"})
        );
        assert_eq!(body["store"], false);
    }
}
