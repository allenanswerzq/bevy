//! The models of NVIDIA, pi's provider `nvidia`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices, ThinkingLevel,
};

/// Registers the 20 models pi lists for `nvidia`.
#[derive(Clone, Copy, Debug, Default)]
pub struct NvidiaModelsPlugin;

impl Plugin for NvidiaModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("nvidia")),
    api: Some(Cow::Borrowed("openai-completions")),
    base_url: Some(Cow::Borrowed("https://integrate.api.nvidia.com/v1")),
    headers: Cow::Borrowed(&[(Cow::Borrowed("NVCF-POLL-SECONDS"), Cow::Borrowed("3600"))]),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "deepseek-ai/deepseek-v4.1-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false,"requiresReasoningContentOnAssistantMessages":true,"thinkingFormat":"deepseek"}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/diffusiongemma-26b-a4b-it",
        ModelInfo {
            name: Some(Cow::Borrowed("DiffusionGemma 26B A4B IT")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(250_000),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemma-3-12b-it",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemma 3 12B IT")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemma-3-4b-it",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemma 3 4B IT")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta/llama-3.2-90b-vision-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama-3.2-90B-Vision-Instruct")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta/muse-glimmer-30b",
        ModelInfo {
            name: Some(Cow::Borrowed("Muse Glimmer 30B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "minimaxai/minimax-m3.1-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax-M3.1-Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-7b-instruct-v0.3",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral-7B-Instruct-v0.3")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(65_536),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k2.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K2.6")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/cosmos-reason2-8b",
        ModelInfo {
            name: Some(Cow::Borrowed("Cosmos Reason2 8B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/llama-3.1-nemotron-70b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 3.1 Nemotron 70B Instruct")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/llama-3.1-nemotron-ultra-253b-v1",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 3.1 Nemotron Ultra 253B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning",
        ModelInfo {
            name: Some(Cow::Borrowed("Nemotron 3 Nano Omni")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3-ultra-550b-a55b",
        ModelInfo {
            name: Some(Cow::Borrowed("Nemotron 3 Ultra 550B A55B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3.5-lightning-30b-a3b",
        ModelInfo {
            name: Some(Cow::Borrowed("Nemotron 3.5 Lightning 30B A3B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3.5-super-vl-120b-a12b",
        ModelInfo {
            name: Some(Cow::Borrowed("Nemotron 3.5 Super VL 120B A12B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-oss-20b",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT OSS 20B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "poolside/laguna-xs-2.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Laguna XS 2.1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "stepfun-ai/step-5-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Step 5 Preview")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5.3-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.3-Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","supportsLongCacheRetention":false,"supportsStrictMode":false}"#,
            )),
            ..BASE
        },
    ),
];
