//! The models of `OpenCode` Go, pi's provider `opencode-go`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, CostTier, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices,
    ThinkingLevel,
};

/// Registers the 32 models pi lists for `opencode-go`.
#[derive(Clone, Copy, Debug, Default)]
pub struct OpenCodeGoModelsPlugin;

impl Plugin for OpenCodeGoModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("opencode-go")),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "beam-free",
        ModelInfo {
            name: Some(Cow::Borrowed("Beam Free")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
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
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-haiku-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 5.5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/anthropic")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.5,
                    cache_read: 0.01,
                    cache_write: 0.125,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 100_000,
                    prices: Prices {
                        input: 0.5,
                        output: 2.5,
                        cache_read: 0.05,
                        cache_write: 0.625,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
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
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoEffort":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek-v4-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4 Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.66,
                    output: 1.98,
                    cache_read: 0.022,
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens","requiresReasoningContentOnAssistantMessages":true,"thinkingFormat":"deepseek"}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek-v4.1-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.003,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens","requiresReasoningContentOnAssistantMessages":true,"thinkingFormat":"deepseek"}"#,
            )),
            ..BASE
        },
    ),
    (
        "gemini-4-argon",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 4 Argon")),
            api: Some(Cow::Borrowed("google-generative-ai")),
            base_url: Some(Cow::Borrowed(
                "https://opencode.ai/inference/go/google/v1beta",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.1,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, None),
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
            ..BASE
        },
    ),
    (
        "glm-5.2",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.2")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.4,
                    output: 4.4,
                    cache_read: 0.26,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "glm-5.3",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.4,
                    output: 4.4,
                    cache_read: 0.26,
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "glm-5.3-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.3-Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Luna")),
            api: Some(Cow::Borrowed("openai-responses")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 1.2,
                    cache_read: 0.02,
                    cache_write: 0.25,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 0.4,
                        output: 1.8,
                        cache_read: 0.04,
                        cache_write: 0.5,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
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
                r#"{"sessionAffinityFormat":"openai-nosession","supportsMidConvoSystemMessages":true,"supportsAdditionalTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Luna")),
            api: Some(Cow::Borrowed("openai-responses")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.5,
                    cache_read: 0.01,
                    cache_write: 0.125,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 0.2,
                        output: 0.75,
                        cache_read: 0.02,
                        cache_write: 0.25,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
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
                r#"{"sessionAffinityFormat":"openai-nosession","supportsMidConvoSystemMessages":true,"supportsAdditionalTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "grok-4.7",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.7")),
            api: Some(Cow::Borrowed("openai-responses")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(500_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
                    cache_read: 0.5,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 4.0,
                        output: 12.0,
                        cache_read: 1.0,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, None),
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
                r#"{"sessionAffinityFormat":"openai-nosession"}"#,
            )),
            ..BASE
        },
    ),
    (
        "hy3",
        ModelInfo {
            name: Some(Cow::Borrowed("Hy3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.14,
                    output: 0.58,
                    cache_read: 0.035,
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
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "hy4",
        ModelInfo {
            name: Some(Cow::Borrowed("Hy4")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.834,
                    output: 2.501,
                    cache_read: 0.042,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "hy4-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Hy4 preview")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_024_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.834,
                    output: 2.501,
                    cache_read: 0.042,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "iceman-contributor-free",
        ModelInfo {
            name: Some(Cow::Borrowed("Iceman Contributor Free")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "kimi-k2.7-code",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K2.7 Code")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.95,
                    output: 4.0,
                    cache_read: 0.19,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "kimi-k3",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.0,
                    output: 15.0,
                    cache_read: 0.3,
                    cache_write: 3.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, None),
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens","supportsMidConvoSystemMessages":true,"supportsMidConvoToolAdditions":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "kimi-k3.1-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3.1 Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.2,
                    cache_write: 2.0,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens","supportsMidConvoSystemMessages":true,"supportsMidConvoToolAdditions":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "longcat-2.0",
        ModelInfo {
            name: Some(Cow::Borrowed("LongCat-2.0")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.006,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "longcat-2.5-free",
        ModelInfo {
            name: Some(Cow::Borrowed("LongCat 2.5 Free")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "mimo-v2.5",
        ModelInfo {
            name: Some(Cow::Borrowed("MiMo V2.5")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.14,
                    output: 0.28,
                    cache_read: 0.0028,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "mimo-v2.5-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("MiMo V2.5 Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.435,
                    output: 0.87,
                    cache_read: 0.003625,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "mimo-v2.6-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("MiMo-V2.6-Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.14,
                    output: 0.28,
                    cache_read: 0.0028,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "mimo-v2.6-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("MiMo-V2.6-Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.435,
                    output: 0.87,
                    cache_read: 0.003625,
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "minimax-m3",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax-M3")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/anthropic")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.06,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 512_000,
                    prices: Prices {
                        input: 0.6,
                        output: 2.4,
                        cache_read: 0.12,
                        cache_write: 0.0,
                    },
                }]),
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
            ..BASE
        },
    ),
    (
        "minimax-m3.1-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax M3.1 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(512_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.015,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
    (
        "muse-spark-1.2-contributor",
        ModelInfo {
            name: Some(Cow::Borrowed("Muse Spark 1.2 Contributor")),
            api: Some(Cow::Borrowed("openai-responses")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.2,
                    cache_read: 0.002,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, None),
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
                r#"{"sessionAffinityFormat":"openai-nosession"}"#,
            )),
            ..BASE
        },
    ),
    (
        "muse-spark-1.3-contributor",
        ModelInfo {
            name: Some(Cow::Borrowed("Muse Spark 1.3 Contributor")),
            api: Some(Cow::Borrowed("openai-responses")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.2,
                    cache_read: 0.002,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, None),
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
                r#"{"sessionAffinityFormat":"openai-nosession"}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen3.7-plus",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3.7 Plus")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/anthropic")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 1.6,
                    cache_read: 0.04,
                    cache_write: 0.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 256_000,
                    prices: Prices {
                        input: 1.2,
                        output: 4.8,
                        cache_read: 0.12,
                        cache_write: 1.5,
                    },
                }]),
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
            ..BASE
        },
    ),
    (
        "qwen3.8-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3.8 Flash")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/anthropic")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.47,
                    cache_read: 0.016,
                    cache_write: 0.2,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "qwen3.8-max",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3.8 Max")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/anthropic")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
                    cache_read: 0.25,
                    cache_write: 2.5,
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
            ..BASE
        },
    ),
    (
        "step-5-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Step 5 Preview")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://opencode.ai/inference/go/openai/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 2.7,
                    cache_read: 0.1,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, None),
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsStrictMode":true,"maxTokensField":"max_tokens"}"#,
            )),
            ..BASE
        },
    ),
];
