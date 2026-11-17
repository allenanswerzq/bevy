//! The models of Baseten, pi's provider `baseten`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices, ThinkingLevel,
};

/// Registers the 15 models pi lists for `baseten`.
#[derive(Clone, Copy, Debug, Default)]
pub struct BasetenModelsPlugin;

impl Plugin for BasetenModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("baseten")),
    api: Some(Cow::Borrowed("openai-completions")),
    base_url: Some(Cow::Borrowed("https://inference.baseten.co/v1")),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "deepseek-ai/DeepSeek-V4-Flash-0731",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4 Flash 0731")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.13,
                    output: 0.26,
                    cache_read: 0.028,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"openai"}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek-ai/DeepSeek-V4-Pro-0813",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4 Pro 0813")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.32,
                    output: 3.96,
                    cache_read: 0.132,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"openai"}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek-ai/DeepSeek-V4.1-Flash",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.007,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"openai"}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek-ai/DeepSeek-V4.1-Flash-1017",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash 1017")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.007,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"openai"}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek-ai/DeepSeek-V4.1-Flash-1017-Fast",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash 1017 Fast")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.4,
                    cache_read: 0.014,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"openai"}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek-ai/DeepSeek-V4.1-Flash-Fast",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash Fast")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.4,
                    cache_read: 0.014,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"openai"}"#,
            )),
            ..BASE
        },
    ),
    (
        "moonshotai/Kimi-K3",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.0,
                    output: 15.0,
                    cache_read: 0.3,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"openai"}"#,
            )),
            ..BASE
        },
    ),
    (
        "moonshotai/Kimi-K3.1-Pro",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3.1 Pro")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.2,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/NVIDIA-Nemotron-3-Ultra-550B-A55B",
        ModelInfo {
            name: Some(Cow::Borrowed("Nemotron 3 Ultra")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(202_800),
            max_output_tokens: Some(202_800),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.4,
                    cache_read: 0.12,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"openai"}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-oss-120b",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI GPT 120B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_072),
            max_output_tokens: Some(128_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.5,
                    cache_read: 0.1,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"openai"}"#,
            )),
            ..BASE
        },
    ),
    (
        "zai-org/GLM-5.2",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.2")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.4,
                    output: 4.4,
                    cache_read: 0.14,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"baseten","chatTemplateArgs":{"enable_thinking":{"$var":"thinking.enabled"}}}"#,
            )),
            ..BASE
        },
    ),
    (
        "zai-org/GLM-5.2-Fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.2 Fast")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.1,
                    output: 6.6,
                    cache_read: 0.21,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"baseten","chatTemplateArgs":{"enable_thinking":{"$var":"thinking.enabled"}}}"#,
            )),
            ..BASE
        },
    ),
    (
        "zai-org/GLM-5.3",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.4,
                    output: 4.4,
                    cache_read: 0.14,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"openai"}"#,
            )),
            ..BASE
        },
    ),
    (
        "zai-org/GLM-5.3-Fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3 Fast")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.1,
                    output: 6.6,
                    cache_read: 0.21,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"openai"}"#,
            )),
            ..BASE
        },
    ),
    (
        "zai-org/GLM-5.3-Flash",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.5,
                    cache_read: 0.03,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"supportsUsageInStreaming":true,"maxTokensField":"max_tokens","sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"thinkingFormat":"openai"}"#,
            )),
            ..BASE
        },
    ),
];
