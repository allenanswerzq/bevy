//! The models of `OpenRouter`, pi's provider `openrouter`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, CostTier, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices,
    ThinkingLevel,
};

/// Registers the 403 models pi lists for `openrouter`.
#[derive(Clone, Copy, Debug, Default)]
pub struct OpenRouterModelsPlugin;

impl Plugin for OpenRouterModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("openrouter")),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "aion-labs/aion-2.0",
        ModelInfo {
            name: Some(Cow::Borrowed("AionLabs: Aion-2.0")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.8,
                    output: 1.6,
                    cache_read: 0.2,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "aion-labs/aion-3.0",
        ModelInfo {
            name: Some(Cow::Borrowed("AionLabs: Aion-3.0")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.0,
                    output: 6.0,
                    cache_read: 0.75,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "aion-labs/aion-3.0-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("AionLabs: Aion-3.0-Mini")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.7,
                    output: 1.4,
                    cache_read: 0.18,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "aion-labs/aion-3.5",
        ModelInfo {
            name: Some(Cow::Borrowed("AionLabs: Aion 3.5")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.0,
                    output: 6.0,
                    cache_read: 0.75,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "aion-labs/aion-3.5-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("AionLabs: Aion 3.5 Mini")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.7,
                    output: 1.4,
                    cache_read: 0.18,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "amazon/nova-2-lite-v1",
        ModelInfo {
            name: Some(Cow::Borrowed("Amazon: Nova 2 Lite")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 2.5,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "amazon/nova-lite-v1",
        ModelInfo {
            name: Some(Cow::Borrowed("Amazon: Nova Lite 1.0")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(5_120),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.24,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "amazon/nova-micro-v1",
        ModelInfo {
            name: Some(Cow::Borrowed("Amazon: Nova Micro 1.0")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(5_120),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.035,
                    output: 0.14,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "amazon/nova-premier-v1",
        ModelInfo {
            name: Some(Cow::Borrowed("Amazon: Nova Premier 1.0")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 12.5,
                    cache_read: 0.625,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "amazon/nova-pro-v1",
        ModelInfo {
            name: Some(Cow::Borrowed("Amazon: Nova Pro 1.0")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(5_120),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.8,
                    output: 3.2,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Fable 5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 10.0,
                    output: 50.0,
                    cache_read: 1.0,
                    cache_write: 12.5,
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
            compat: Some(Cow::Borrowed(r#"{"forceAdaptiveThinking":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Fable 5.1")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 10.0,
                    output: 50.0,
                    cache_read: 0.25,
                    cache_write: 12.5,
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
                r#"{"forceAdaptiveThinking":true,"supportsMidConvoEffort":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5.1:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Fable 5.1 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 25.0,
                    cache_read: 0.125,
                    cache_write: 6.25,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5.2",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Fable 5.2")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 8.0,
                    output: 40.0,
                    cache_read: 0.2,
                    cache_write: 10.0,
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
                r#"{"forceAdaptiveThinking":true,"supportsMidConvoEffort":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5.2:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Fable 5.2 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 20.0,
                    cache_read: 0.1,
                    cache_write: 5.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Fable 5 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 25.0,
                    cache_read: 0.5,
                    cache_write: 6.25,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-haiku-4.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Haiku 4.5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 5.0,
                    cache_read: 0.1,
                    cache_write: 1.25,
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
        "anthropic/claude-haiku-4.5:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Haiku 4.5 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 2.5,
                    cache_read: 0.05,
                    cache_write: 0.625,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-haiku-5.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Haiku 5.5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
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
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoEffort":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-haiku-5.5:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Haiku 5.5 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.05,
                    output: 0.25,
                    cache_read: 0.005,
                    cache_write: 0.0625,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 100_000,
                    prices: Prices {
                        input: 0.25,
                        output: 1.25,
                        cache_read: 0.025,
                        cache_write: 0.3125,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 4.1")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 15.0,
                    output: 75.0,
                    cache_read: 1.5,
                    cache_write: 18.75,
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
        "anthropic/claude-opus-4.1:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 4.1 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 7.5,
                    output: 37.5,
                    cache_read: 0.75,
                    cache_write: 9.375,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 4.5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 25.0,
                    cache_read: 0.5,
                    cache_write: 6.25,
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
        "anthropic/claude-opus-4.5:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 4.5 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 12.5,
                    cache_read: 0.25,
                    cache_write: 3.125,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 4.6")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 25.0,
                    cache_read: 0.5,
                    cache_write: 6.25,
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
            compat: Some(Cow::Borrowed(r#"{"forceAdaptiveThinking":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.6:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 4.6 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 12.5,
                    cache_read: 0.25,
                    cache_write: 3.125,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.7",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 4.7")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 25.0,
                    cache_read: 0.5,
                    cache_write: 6.25,
                },
                tiers: Cow::Borrowed(&[]),
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
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.7:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 4.7 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 12.5,
                    cache_read: 0.25,
                    cache_write: 3.125,
                },
                tiers: Cow::Borrowed(&[]),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.8",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 4.8")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 25.0,
                    cache_read: 0.5,
                    cache_write: 6.25,
                },
                tiers: Cow::Borrowed(&[]),
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
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.8:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 4.8 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 12.5,
                    cache_read: 0.25,
                    cache_write: 3.125,
                },
                tiers: Cow::Borrowed(&[]),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 25.0,
                    cache_read: 0.5,
                    cache_write: 6.25,
                },
                tiers: Cow::Borrowed(&[]),
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
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-5.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 5.5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 20.0,
                    cache_read: 0.2,
                    cache_write: 5.0,
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
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoEffort":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-5.5:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 5.5 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.1,
                    cache_write: 2.5,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-5:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus 5 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 12.5,
                    cache_read: 0.25,
                    cache_write: 3.125,
                },
                tiers: Cow::Borrowed(&[]),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-4.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Sonnet 4.5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.0,
                    output: 15.0,
                    cache_read: 0.3,
                    cache_write: 3.75,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 6.0,
                        output: 22.5,
                        cache_read: 0.6,
                        cache_write: 7.5,
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
        "anthropic/claude-sonnet-4.5:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Sonnet 4.5 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.5,
                    output: 7.5,
                    cache_read: 0.15,
                    cache_write: 1.875,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 3.0,
                        output: 11.25,
                        cache_read: 0.3,
                        cache_write: 3.75,
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
            compat: Some(Cow::Borrowed(
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-4.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Sonnet 4.6")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.0,
                    output: 15.0,
                    cache_read: 0.3,
                    cache_write: 3.75,
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
            compat: Some(Cow::Borrowed(r#"{"forceAdaptiveThinking":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-4.6:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Sonnet 4.6 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.5,
                    output: 7.5,
                    cache_read: 0.15,
                    cache_write: 1.875,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Sonnet 5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.2,
                    cache_write: 2.5,
                },
                tiers: Cow::Borrowed(&[]),
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
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-5.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Sonnet 5.5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.1,
                    cache_write: 2.5,
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
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoEffort":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-5.5:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Sonnet 5.5 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 5.0,
                    cache_read: 0.05,
                    cache_write: 1.25,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-5:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Sonnet 5 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 5.0,
                    cache_read: 0.1,
                    cache_write: 1.25,
                },
                tiers: Cow::Borrowed(&[]),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"cacheControlFormat":"anthropic","sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "auto",
        ModelInfo {
            name: Some(Cow::Borrowed("Auto")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(30_000),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "bytedance-seed/seed-1.6",
        ModelInfo {
            name: Some(Cow::Borrowed("ByteDance Seed: Seed 1.6")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 2.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 128_000,
                    prices: Prices {
                        input: 0.5,
                        output: 4.0,
                        cache_read: 0.0,
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "bytedance-seed/seed-1.6-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("ByteDance Seed: Seed 1.6 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.075,
                    output: 0.3,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 128_000,
                    prices: Prices {
                        input: 0.1,
                        output: 0.8,
                        cache_read: 0.0,
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "bytedance-seed/seed-2-1-turbo",
        ModelInfo {
            name: Some(Cow::Borrowed("ByteDance Seed: Seed 2.1 Turbo")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 2.5,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "bytedance-seed/seed-2.0-code",
        ModelInfo {
            name: Some(Cow::Borrowed("ByteDance Seed: Seed-2.0-Code")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 3.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 128_000,
                    prices: Prices {
                        input: 1.0,
                        output: 6.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "bytedance-seed/seed-2.0-lite",
        ModelInfo {
            name: Some(Cow::Borrowed("ByteDance Seed: Seed-2.0-Lite")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 2.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 128_000,
                    prices: Prices {
                        input: 0.5,
                        output: 4.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "bytedance-seed/seed-2.0-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("ByteDance Seed: Seed-2.0-Mini")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.4,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 128_000,
                    prices: Prices {
                        input: 0.2,
                        output: 0.8,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "cohere/command-a",
        ModelInfo {
            name: Some(Cow::Borrowed("Cohere: Command A")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 10.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "cohere/command-a-plus",
        ModelInfo {
            name: Some(Cow::Borrowed("Cohere: Command A+")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(192_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.5,
                    cache_read: 0.15,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "cohere/command-r-08-2024",
        ModelInfo {
            name: Some(Cow::Borrowed("Cohere: Command R (08-2024)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "cohere/command-r-plus-08-2024",
        ModelInfo {
            name: Some(Cow::Borrowed("Cohere: Command R+ (08-2024)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 10.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "cohere/command-r7b-12-2024",
        ModelInfo {
            name: Some(Cow::Borrowed("Cohere: Command R7B (12-2024)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0375,
                    output: 0.15,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "cohere/north-mini-code:free",
        ModelInfo {
            name: Some(Cow::Borrowed("Cohere: North Mini Code (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(64_000),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-chat",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(163_840),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.32,
                    output: 0.89,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-chat-v3-0324",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V3 0324")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(115_200),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.29,
                    output: 1.14,
                    cache_read: 0.11,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-chat-v3.1",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V3.1")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(163_840),
            max_output_tokens: Some(147_456),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 0.95,
                    cache_read: 0.13,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-r1-0528",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: R1 0528")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(163_840),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 2.15,
                    cache_read: 0.35,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v3.1-terminus",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V3.1 Terminus")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(163_840),
            max_output_tokens: Some(147_456),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.27,
                    output: 1.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v3.2",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V3.2")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(163_840),
            max_output_tokens: Some(147_456),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.26,
                    output: 0.38,
                    cache_read: 0.13,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v3.2-exp",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V3.2 Exp")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(163_840),
            max_output_tokens: Some(147_456),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.27,
                    output: 0.41,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V4 Flash 0423")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_575),
            max_output_tokens: Some(943_717),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.091,
                    output: 0.182,
                    cache_read: 0.0182,
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"requiresReasoningContentOnAssistantMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4-flash-0731",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V4 Flash 0731")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0434,
                    output: 0.3,
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
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"requiresReasoningContentOnAssistantMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4-flash-0731:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V4 Flash 0731 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.048,
                    output: 0.144,
                    cache_read: 0.012,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"requiresReasoningContentOnAssistantMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4-flash-vision-exp",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V4 Flash Vision Exp")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_575),
            max_output_tokens: Some(943_717),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.44,
                    output: 1.32,
                    cache_read: 0.014,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"requiresReasoningContentOnAssistantMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V4 Pro 0423")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_024_000),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.95526,
                    output: 1.91052,
                    cache_read: 0.079605,
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"requiresReasoningContentOnAssistantMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4-pro-0813",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V4 Pro 0813")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.111,
                    output: 4.44,
                    cache_read: 0.097,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"requiresReasoningContentOnAssistantMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4.1-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V4.1 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.006,
                    output: 0.5,
                    cache_read: 0.005,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"requiresReasoningContentOnAssistantMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4.1-flash-1017",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V4.1 Flash 1017")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.006,
                    output: 0.5,
                    cache_read: 0.005,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"requiresReasoningContentOnAssistantMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4.1-flash-1017:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V4.1 Flash 1017 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.011,
                    output: 0.2,
                    cache_read: 0.002,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"requiresReasoningContentOnAssistantMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4.1-flash:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V4.1 Flash (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.112,
                    output: 0.336,
                    cache_read: 0.00336,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"requiresReasoningContentOnAssistantMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "dots-studio/dots-3-note-preview:free",
        ModelInfo {
            name: Some(Cow::Borrowed("Dots Studio: Dots3-Note Preview (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(512_000),
            max_output_tokens: Some(460_800),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "elorian/glimpse",
        ModelInfo {
            name: Some(Cow::Borrowed("Elorian: Glimpse")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(256_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 3.0,
                    cache_read: 0.06,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "fireworks/ember-1",
        ModelInfo {
            name: Some(Cow::Borrowed("Fireworks: Ember-1")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-2.5-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 2.5 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 2.5,
                    cache_read: 0.03,
                    cache_write: 0.083333,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-2.5-flash-lite",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 2.5 Flash Lite")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.4,
                    cache_read: 0.01,
                    cache_write: 0.083333,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-2.5-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 2.5 Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 10.0,
                    cache_read: 0.125,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 2.5,
                        output: 15.0,
                        cache_read: 0.25,
                        cache_write: 0.375,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-2.5-pro-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 2.5 Pro Preview 06-05")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 10.0,
                    cache_read: 0.125,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 2.5,
                        output: 15.0,
                        cache_read: 0.25,
                        cache_write: 0.375,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3-flash-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3 Flash Preview")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 3.0,
                    cache_read: 0.05,
                    cache_write: 0.083333,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3-flash-preview:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3 Flash Preview (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 1.5,
                    cache_read: 0.0,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3-pro-image",
        ModelInfo {
            name: Some(Cow::Borrowed(
                "Google: Nano Banana Pro (Gemini 3 Pro Image)",
            )),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(65_536),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 12.0,
                    cache_read: 0.2,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.1-flash-lite",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.1 Flash Lite")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 1.5,
                    cache_read: 0.025,
                    cache_write: 0.083333,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.1-flash-lite-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.1 Flash Lite Preview")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 1.5,
                    cache_read: 0.025,
                    cache_write: 0.083333,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.1-flash-lite:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.1 Flash Lite (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.125,
                    output: 0.75,
                    cache_read: 0.0125,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.1-pro-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.1 Pro Preview")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 12.0,
                    cache_read: 0.2,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 4.0,
                        output: 18.0,
                        cache_read: 0.4,
                        cache_write: 0.375,
                    },
                }]),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.1-pro-preview-customtools",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.1 Pro Preview Custom Tools")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 12.0,
                    cache_read: 0.2,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 4.0,
                        output: 18.0,
                        cache_read: 0.4,
                        cache_write: 0.375,
                    },
                }]),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.1-pro-preview:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.1 Pro Preview (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 6.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 2.0,
                        output: 9.0,
                        cache_read: 0.0,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.5-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.5 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.5,
                    output: 9.0,
                    cache_read: 0.15,
                    cache_write: 0.083333,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.5-flash-lite",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.5 Flash Lite")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 2.5,
                    cache_read: 0.03,
                    cache_write: 0.083333,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.5-flash-lite:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.5 Flash Lite (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 1.25,
                    cache_read: 0.015,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.5-flash:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.5 Flash (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.75,
                    output: 4.5,
                    cache_read: 0.075,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.6-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.6 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.75,
                    output: 3.75,
                    cache_read: 0.075,
                    cache_write: 0.041667,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.6-flash:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.6 Flash (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.375,
                    output: 1.875,
                    cache_read: 0.0375,
                    cache_write: 0.041667,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.7-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.7 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.75,
                    output: 3.75,
                    cache_read: 0.075,
                    cache_write: 0.041667,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.7-flash:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.7 Flash (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.375,
                    output: 1.875,
                    cache_read: 0.0375,
                    cache_write: 0.041667,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.8-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.8 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.825,
                    output: 4.125,
                    cache_read: 0.0825,
                    cache_write: 0.041667,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-3.8-flash:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 3.8 Flash (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.375,
                    output: 1.875,
                    cache_read: 0.0375,
                    cache_write: 0.041667,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-4-argon",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 4 Argon")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.1,
                    cache_write: 0.1875,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-4-argon:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini 4 Argon (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 5.0,
                    cache_read: 0.05,
                    cache_write: 0.1875,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemini-nano-banana-2.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Nano Banana 2.1")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(65_536),
            max_output_tokens: Some(58_982),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.5,
                    output: 7.5,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemma-3-12b-it",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemma 3 12B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.05,
                    output: 0.15,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemma-3-27b-it",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemma 3 27B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(117_964),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.08,
                    output: 0.45,
                    cache_read: 0.04,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemma-4-26b-a4b-it",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemma 4 26B A4B ")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.08,
                    output: 0.1,
                    cache_read: 0.046,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemma-4-26b-a4b-it:free",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemma 4 26B A4B  (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemma-4-31b-it",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemma 4 31B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 0.4,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "google/gemma-4-31b-it:free",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemma 4 31B (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "ibm-granite/granite-4.2-8b",
        ModelInfo {
            name: Some(Cow::Borrowed("IBM: Granite 4.2 8B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(117_964),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.25,
                    cache_read: 0.015,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "inclusionai/ling-3.0-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("inclusionAI: Ling 3.0 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.021,
                    output: 0.063,
                    cache_read: 0.0042,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "inclusionai/ling-3.0-flash-vl",
        ModelInfo {
            name: Some(Cow::Borrowed("inclusionAI: Ling 3.0 Flash VL")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.18,
                    cache_read: 0.012,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "inclusionai/ling-3.0-tiny",
        ModelInfo {
            name: Some(Cow::Borrowed("inclusionAI: Ling 3.0 Tiny")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(250_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.015,
                    output: 0.045,
                    cache_read: 0.003,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "inclusionai/ling-3.1-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("inclusionAI: Ling 3.1 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.045,
                    output: 0.135,
                    cache_read: 0.009,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "liquid/lfm-2.5-2.6b",
        ModelInfo {
            name: Some(Cow::Borrowed("LiquidAI: LFM2.5-2.6B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.35,
                    output: 0.5,
                    cache_read: 0.04,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "liquid/lfm-2.5-2.6b:free",
        ModelInfo {
            name: Some(Cow::Borrowed("LiquidAI: LFM2.5-2.6B (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(65_536),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "liquid/lfm-2.5-24b-a2b",
        ModelInfo {
            name: Some(Cow::Borrowed("LiquidAI: LFM2.5-24B-A2B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.05,
                    output: 0.7,
                    cache_read: 0.025,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "liquid/lfm-3-1.7b",
        ModelInfo {
            name: Some(Cow::Borrowed("LiquidAI: LFM3-1.7B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.08,
                    output: 0.2,
                    cache_read: 0.02,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "liquid/lfm-3-1.7b:free",
        ModelInfo {
            name: Some(Cow::Borrowed("LiquidAI: LFM3-1.7B (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meituan/longcat-2.0",
        ModelInfo {
            name: Some(Cow::Borrowed("Meituan: LongCat 2.0")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_756),
            max_output_tokens: Some(262_144),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta-llama/llama-3.1-8b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Llama 3.1 8B Instruct")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.02,
                    output: 0.04,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta-llama/llama-3.3-70b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Llama 3.3 70B Instruct")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(115_200),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.71,
                    output: 0.71,
                    cache_read: 0.71,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta-llama/llama-4-maverick",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Llama 4 Maverick")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(524_288),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.35,
                    output: 1.0,
                    cache_read: 0.17,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta-llama/llama-4-scout",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Llama 4 Scout")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(327_680),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.3,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta/muse-ember",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Muse Ember")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.5,
                    cache_read: 0.004,
                    cache_write: 0.125,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta/muse-glimmer-30b",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Muse Glimmer 30B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.04,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta/muse-glimmer-30b:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Muse Glimmer 30B (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.24,
                    output: 0.96,
                    cache_read: 0.032,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta/muse-spark-1.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Muse Spark 1.1")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 4.25,
                    cache_read: 0.15,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta/muse-spark-1.2",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Muse Spark 1.2")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 4.25,
                    cache_read: 0.15,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta/muse-spark-1.2-contributor",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Muse Spark 1.2 Contributor")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta/muse-spark-1.3",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Muse Spark 1.3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 4.25,
                    cache_read: 0.15,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta/muse-spark-1.3-contributor",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Muse Spark 1.3 Contributor")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta/muse-spark-open",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Muse Spark Open")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 4.25,
                    cache_read: 0.15,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "meta/muse-wonder",
        ModelInfo {
            name: Some(Cow::Borrowed("Meta: Muse Wonder")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 20.0,
                    cache_read: 0.2,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m2.5",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax: MiniMax M2.5")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(65_536),
            max_output_tokens: Some(58_982),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.06,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m2.7",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax: MiniMax M2.7")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(196_608),
            max_output_tokens: Some(176_947),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.21,
                    output: 0.84,
                    cache_read: 0.042,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m3",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax: MiniMax M3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(524_288),
            max_output_tokens: Some(512_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.06,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m3.1-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax: MiniMax M3.1 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(524_288),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m3:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax: MiniMax M3 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(524_288),
            max_output_tokens: Some(512_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.224,
                    output: 0.88,
                    cache_read: 0.0448,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/codestral-2508",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Codestral 2508")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(204_800),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 0.9,
                    cache_read: 0.03,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/devstral-2512",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Devstral 2 2512")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(209_715),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 2.0,
                    cache_read: 0.04,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/ministral-14b-2512",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Ministral 3 14B 2512")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(209_715),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 0.2,
                    cache_read: 0.02,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/ministral-3b-2512",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Ministral 3 3B 2512")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(104_857),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.1,
                    cache_read: 0.01,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/ministral-8b-2512",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Ministral 3 8B 2512")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(209_715),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.15,
                    cache_read: 0.015,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-large",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Large")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(102_400),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
                    cache_read: 0.2,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-large-2407",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Large 2407")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(104_857),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
                    cache_read: 0.2,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-large-2512",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Mistral Large 3 2512")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(209_715),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 1.5,
                    cache_read: 0.05,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-large-4-0",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Mistral Large 4")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.7,
                    output: 2.1,
                    cache_read: 0.07,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-medium-3",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Mistral Medium 3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(104_857),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 2.0,
                    cache_read: 0.04,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-medium-3-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Mistral Medium 3.5")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(209_715),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.5,
                    output: 7.5,
                    cache_read: 0.0,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-medium-3-5:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Mistral Medium 3.5 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(209_715),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.75,
                    output: 3.75,
                    cache_read: 0.0,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-medium-3.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Mistral Medium 3.1")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(104_857),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 2.0,
                    cache_read: 0.04,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-nemo",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Mistral Nemo")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.029,
                    output: 0.03,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-saba",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Saba")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(32_768),
            max_output_tokens: Some(26_214),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 0.6,
                    cache_read: 0.02,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-small-2603",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Mistral Small 4")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(209_715),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-small-3.1-24b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Mistral Small 3.1 24B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(102_400),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.351,
                    output: 0.555,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mistral-small-3.2-24b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Mistral Small 3.2 24B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(32_768),
            max_output_tokens: Some(26_214),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.3,
                    cache_read: 0.01,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/mixtral-8x22b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Mixtral 8x22B Instruct")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(65_536),
            max_output_tokens: Some(52_428),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
                    cache_read: 0.2,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mistralai/voxtral-small-24b-2507",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral: Voxtral Small 24B 2507")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(32_768),
            max_output_tokens: Some(26_214),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.3,
                    cache_read: 0.01,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k2",
        ModelInfo {
            name: Some(Cow::Borrowed("MoonshotAI: Kimi K2 0711")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(98_304),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.57,
                    output: 2.3,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k2-0905",
        ModelInfo {
            name: Some(Cow::Borrowed("MoonshotAI: Kimi K2 0905")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(98_304),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.5,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k2-thinking",
        ModelInfo {
            name: Some(Cow::Borrowed("MoonshotAI: Kimi K2 Thinking")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(98_304),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.5,
                    cache_read: 0.15,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k2.5",
        ModelInfo {
            name: Some(Cow::Borrowed("MoonshotAI: Kimi K2.5")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.41,
                    output: 2.06,
                    cache_read: 0.07,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k2.6",
        ModelInfo {
            name: Some(Cow::Borrowed("MoonshotAI: Kimi K2.6")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.65,
                    output: 3.41,
                    cache_read: 0.15,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"requiresReasoningContentOnAssistantMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k2.7-code",
        ModelInfo {
            name: Some(Cow::Borrowed("MoonshotAI: Kimi K2.7 Code")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 3.5,
                    cache_read: 0.16,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k3",
        ModelInfo {
            name: Some(Cow::Borrowed("MoonshotAI: Kimi K3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.192,
                    output: 16.0,
                    cache_read: 0.182,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k3.1-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("MoonshotAI: Kimi K3.1 Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.2,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k3:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("MoonshotAI: Kimi K3 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.28,
                    output: 11.4,
                    cache_read: 0.228,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "nex-agi/nex-n2.5-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("Nex AGI: Nex-N2.5-Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.075,
                    output: 0.25,
                    cache_read: 0.015,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "nex-agi/nex-n2.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Nex AGI: Nex-N2.6")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.12,
                    output: 0.4,
                    cache_read: 0.024,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3-nano-30b-a3b",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA: Nemotron 3 Nano 30B A3B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.24,
                    cache_read: 0.012,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA: Nemotron 3 Nano Omni (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3-super-120b-a12b",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA: Nemotron 3 Super")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.075,
                    output: 0.4,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, None),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3-ultra-550b-a55b",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA: Nemotron 3 Ultra")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(202_800),
            max_output_tokens: Some(182_520),
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
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3-ultra-550b-a55b:free",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA: Nemotron 3 Ultra (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
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
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3.5-lightning",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA: Nemotron 3.5 Lightning")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.07,
                    output: 0.2,
                    cache_read: 0.04,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3.5-lightning:free",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA: Nemotron 3.5 Lightning (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3.5-super-vl",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA: Nemotron 3.5 Super VL")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.47,
                    cache_read: 0.02,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3.5-super-vl:free",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA: Nemotron 3.5 Super VL (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-3.5-turbo-0613",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-3.5 Turbo (older v0613)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(4_095),
            max_output_tokens: Some(3_685),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 2.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-3.5-turbo-16k",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-3.5 Turbo 16k")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(16_385),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.0,
                    output: 4.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(8_191),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 30.0,
                    output: 60.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4-turbo:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4 Turbo (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 15.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4.1")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_047_576),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 8.0,
                    cache_read: 0.5,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4.1 Mini")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_047_576),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 1.6,
                    cache_read: 0.1,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1-mini:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4.1 Mini (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_047_576),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 0.8,
                    cache_read: 0.05,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1-nano",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4.1 Nano")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_047_576),
            max_output_tokens: Some(942_818),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.4,
                    cache_read: 0.03,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1-nano:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4.1 Nano (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_047_576),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.05,
                    output: 0.2,
                    cache_read: 0.0125,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4.1 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_047_576),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 4.0,
                    cache_read: 0.25,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4o")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 10.0,
                    cache_read: 1.25,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o-2024-05-13",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4o (2024-05-13)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 15.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o-2024-08-06",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4o (2024-08-06)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 10.0,
                    cache_read: 1.25,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o-2024-11-20",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4o (2024-11-20)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 10.0,
                    cache_read: 1.25,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4o-mini")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.075,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o-mini-2024-07-18",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4o-mini (2024-07-18)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.075,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o-mini:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4o-mini (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.075,
                    output: 0.3,
                    cache_read: 0.0375,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-4o (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 5.0,
                    cache_read: 0.625,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 10.0,
                    cache_read: 0.125,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5 Mini")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 2.0,
                    cache_read: 0.025,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5-mini:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5 Mini (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.125,
                    output: 1.0,
                    cache_read: 0.0125,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5-nano",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5 Nano")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.05,
                    output: 0.4,
                    cache_read: 0.005,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5-nano:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5 Nano (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.025,
                    output: 0.2,
                    cache_read: 0.0025,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5 Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 15.0,
                    output: 120.0,
                    cache_read: 0.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5-pro:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5 Pro (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 7.5,
                    output: 60.0,
                    cache_read: 0.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.1",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.1")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 10.0,
                    cache_read: 0.125,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.1-codex",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.1-Codex")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 10.0,
                    cache_read: 0.13,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.1-codex-max",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.1-Codex-Max")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 10.0,
                    cache_read: 0.125,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.1-codex-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.1-Codex-Mini")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 2.0,
                    cache_read: 0.03,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.1:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.1 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.625,
                    output: 5.0,
                    cache_read: 0.0625,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.2",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.2")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.75,
                    output: 14.0,
                    cache_read: 0.175,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.2-chat",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.2 Chat")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.75,
                    output: 14.0,
                    cache_read: 0.175,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.2-codex",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.2-Codex")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.75,
                    output: 14.0,
                    cache_read: 0.175,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.2-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.2 Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 21.0,
                    output: 168.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.2-pro:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.2 Pro (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 10.5,
                    output: 84.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.2:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.2 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.875,
                    output: 7.0,
                    cache_read: 0.0875,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.3-codex",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.3-Codex")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.75,
                    output: 14.0,
                    cache_read: 0.175,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.4")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 15.0,
                    cache_read: 0.25,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 5.0,
                        output: 22.5,
                        cache_read: 0.5,
                        cache_write: 0.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.4 Mini")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.75,
                    output: 4.5,
                    cache_read: 0.075,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-mini:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.4 Mini (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.375,
                    output: 2.25,
                    cache_read: 0.0375,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-nano",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.4 Nano")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 1.25,
                    cache_read: 0.02,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-nano:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.4 Nano (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.625,
                    cache_read: 0.01,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.4 Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 30.0,
                    output: 180.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 60.0,
                        output: 270.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-pro:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.4 Pro (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 15.0,
                    output: 90.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 30.0,
                        output: 135.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.4 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 7.5,
                    cache_read: 0.125,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 2.5,
                        output: 11.25,
                        cache_read: 0.25,
                        cache_write: 0.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.5",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.5")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 30.0,
                    cache_read: 0.5,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 10.0,
                        output: 45.0,
                        cache_read: 1.0,
                        cache_write: 0.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.5-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.5 Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 30.0,
                    output: 180.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 60.0,
                        output: 270.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.5-pro:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.5 Pro (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 15.0,
                    output: 90.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 30.0,
                        output: 135.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.5:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.5 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 15.0,
                    cache_read: 0.25,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 5.0,
                        output: 22.5,
                        cache_read: 0.5,
                        cache_write: 0.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.6 Luna")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-luna-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.6 Luna Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-luna-pro:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.6 Luna Pro (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.6,
                    cache_read: 0.01,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 0.2,
                        output: 0.9,
                        cache_read: 0.02,
                        cache_write: 0.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-luna:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.6 Luna (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.6,
                    cache_read: 0.01,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 0.2,
                        output: 0.9,
                        cache_read: 0.02,
                        cache_write: 0.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.6 Sol")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 20.0,
                    cache_read: 0.4,
                    cache_write: 5.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 8.0,
                        output: 30.0,
                        cache_read: 0.8,
                        cache_write: 10.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-sol-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.6 Sol Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 20.0,
                    cache_read: 0.4,
                    cache_write: 5.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 8.0,
                        output: 30.0,
                        cache_read: 0.8,
                        cache_write: 10.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-sol-pro:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.6 Sol Pro (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.2,
                    cache_write: 2.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.0,
                        output: 15.0,
                        cache_read: 0.4,
                        cache_write: 5.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-sol:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.6 Sol (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.2,
                    cache_write: 2.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.0,
                        output: 15.0,
                        cache_read: 0.4,
                        cache_write: 5.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.6 Terra")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 12.0,
                    cache_read: 0.2,
                    cache_write: 2.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.0,
                        output: 18.0,
                        cache_read: 0.4,
                        cache_write: 5.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-terra-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.6 Terra Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 12.0,
                    cache_read: 0.2,
                    cache_write: 2.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.0,
                        output: 18.0,
                        cache_read: 0.4,
                        cache_write: 5.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-terra-pro:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.6 Terra Pro (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 6.0,
                    cache_read: 0.1,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 2.0,
                        output: 9.0,
                        cache_read: 0.2,
                        cache_write: 0.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-terra:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5.6 Terra (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 6.0,
                    cache_read: 0.1,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 2.0,
                        output: 9.0,
                        cache_read: 0.2,
                        cache_write: 0.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-5:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-5 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.625,
                    output: 5.0,
                    cache_read: 0.0625,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-astra",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6 Astra")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 10.0,
                    output: 50.0,
                    cache_read: 1.0,
                    cache_write: 12.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 20.0,
                        output: 75.0,
                        cache_read: 2.0,
                        cache_write: 25.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-astra-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6 Astra Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 10.0,
                    output: 50.0,
                    cache_read: 1.0,
                    cache_write: 12.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 20.0,
                        output: 75.0,
                        cache_read: 2.0,
                        cache_write: 25.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-astra-pro:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6 Astra Pro (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 25.0,
                    cache_read: 0.5,
                    cache_write: 6.25,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 10.0,
                        output: 37.5,
                        cache_read: 1.0,
                        cache_write: 12.5,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-astra:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6 Astra (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 25.0,
                    cache_read: 0.5,
                    cache_write: 6.25,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 10.0,
                        output: 37.5,
                        cache_read: 1.0,
                        cache_write: 12.5,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6 Luna")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-luna-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6 Luna Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-luna-pro:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6 Luna Pro (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.05,
                    output: 0.25,
                    cache_read: 0.005,
                    cache_write: 0.0625,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 0.1,
                        output: 0.375,
                        cache_read: 0.01,
                        cache_write: 0.125,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-luna:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6 Luna (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.05,
                    output: 0.25,
                    cache_read: 0.005,
                    cache_write: 0.0625,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 0.1,
                        output: 0.375,
                        cache_read: 0.01,
                        cache_write: 0.125,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6 Sol")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.2,
                    cache_write: 2.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.0,
                        output: 15.0,
                        cache_read: 0.4,
                        cache_write: 5.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-sol-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6 Sol Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.2,
                    cache_write: 2.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.0,
                        output: 15.0,
                        cache_read: 0.4,
                        cache_write: 5.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-sol-pro:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6 Sol Pro (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 5.0,
                    cache_read: 0.1,
                    cache_write: 1.25,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 2.0,
                        output: 7.5,
                        cache_read: 0.2,
                        cache_write: 2.5,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-sol:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6 Sol (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 5.0,
                    cache_read: 0.1,
                    cache_write: 1.25,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 2.0,
                        output: 7.5,
                        cache_read: 0.2,
                        cache_write: 2.5,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6.1-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6.1 Sol")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.1,
                    cache_write: 2.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.0,
                        output: 15.0,
                        cache_read: 0.2,
                        cache_write: 5.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6.1-sol-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6.1 Sol Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.1,
                    cache_write: 2.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.0,
                        output: 15.0,
                        cache_read: 0.2,
                        cache_write: 5.0,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6.1-sol-pro:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6.1 Sol Pro (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 5.0,
                    cache_read: 0.05,
                    cache_write: 1.25,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 2.0,
                        output: 7.5,
                        cache_read: 0.1,
                        cache_write: 2.5,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6.1-sol:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6.1 Sol (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 5.0,
                    cache_read: 0.05,
                    cache_write: 1.25,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 2.0,
                        output: 7.5,
                        cache_read: 0.1,
                        cache_write: 2.5,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6.1-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6.1 Terra")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 5.0,
                    cache_read: 0.05,
                    cache_write: 1.25,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 2.0,
                        output: 7.5,
                        cache_read: 0.1,
                        cache_write: 2.5,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6.1-terra-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6.1 Terra Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 5.0,
                    cache_read: 0.05,
                    cache_write: 1.25,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 2.0,
                        output: 7.5,
                        cache_read: 0.1,
                        cache_write: 2.5,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6.1-terra-pro:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6.1 Terra Pro (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 2.5,
                    cache_read: 0.025,
                    cache_write: 0.625,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 1.0,
                        output: 3.75,
                        cache_read: 0.05,
                        cache_write: 1.25,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-6.1-terra:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT-6.1 Terra (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 2.5,
                    cache_read: 0.025,
                    cache_write: 0.625,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 1.0,
                        output: 3.75,
                        cache_read: 0.05,
                        cache_write: 1.25,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-audio",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT Audio")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 10.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-audio-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT Audio Mini")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.4,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-chat-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT Chat Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 30.0,
                    cache_read: 0.5,
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-oss-120b",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: gpt-oss-120b")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.075,
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
            compat: Some(Cow::Borrowed(
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-oss-120b:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: gpt-oss-120b (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(117_964),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0296,
                    output: 0.136,
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
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-oss-20b",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: gpt-oss-20b")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.018,
                    output: 0.09,
                    cache_read: 0.009,
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
            compat: Some(Cow::Borrowed(
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-oss-20b:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: gpt-oss-20b (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(117_964),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.024,
                    output: 0.112,
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
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/gpt-oss-safeguard-20b",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: gpt-oss-safeguard-20b")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.075,
                    output: 0.3,
                    cache_read: 0.0375,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/o3",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: o3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(100_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 8.0,
                    cache_read: 0.5,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/o3-mini:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: o3 Mini (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(200_000),
            max_output_tokens: Some(100_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.55,
                    output: 2.2,
                    cache_read: 0.275,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/o3-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: o3 Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(100_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 20.0,
                    output: 80.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/o3:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: o3 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(100_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 4.0,
                    cache_read: 0.25,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openai/o4-mini:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: o4 Mini (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(100_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.55,
                    output: 2.2,
                    cache_read: 0.1375,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
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
                r#"{"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openrouter/auto",
        ModelInfo {
            name: Some(Cow::Borrowed("Auto Router")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(4_096),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openrouter/auto-beta",
        ModelInfo {
            name: Some(Cow::Borrowed("Auto Router (Beta)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(64_000),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openrouter/auto-code",
        ModelInfo {
            name: Some(Cow::Borrowed("Auto Code")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(64_000),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openrouter/free",
        ModelInfo {
            name: Some(Cow::Borrowed("Free Models Router")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(4_096),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openrouter/fusion",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenRouter: Fusion")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(30_000),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "openrouter/pareto-code",
        ModelInfo {
            name: Some(Cow::Borrowed("Pareto Code Router")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(4_096),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "particle-ai/particle-1",
        ModelInfo {
            name: Some(Cow::Borrowed("Particle AI: Particle-1")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.05,
                    output: 0.5,
                    cache_read: 0.003,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "perceptron/perceptron-mk1.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Perceptron: Perceptron Mk1.5")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(36_864),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 1.5,
                    cache_read: 0.0,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "prism-ml/ternary-bonsai-2-27b",
        ModelInfo {
            name: Some(Cow::Borrowed("PrismML: Ternary Bonsai 2 27B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.075,
                    output: 0.5,
                    cache_read: 0.0375,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, None),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "prism-ml/ternary-bonsai-2-9b",
        ModelInfo {
            name: Some(Cow::Borrowed("PrismML: Ternary Bonsai 2 9B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.08,
                    output: 0.13,
                    cache_read: 0.04,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen-2.5-72b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen2.5 72B Instruct")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(32_768),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.36,
                    output: 0.4,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen-2.5-7b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen2.5 7B Instruct")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(32_768),
            max_output_tokens: Some(29_491),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen-plus",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen-Plus")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.26,
                    output: 0.78,
                    cache_read: 0.052,
                    cache_write: 0.325,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 256_000,
                    prices: Prices {
                        input: 0.78,
                        output: 2.34,
                        cache_read: 0.156,
                        cache_write: 0.975,
                    },
                }]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-14b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 14B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(40_960),
            max_output_tokens: Some(36_864),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.22,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-235b-a22b-2507",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 235B A22B Instruct 2507")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(117_964),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.14,
                    output: 0.8,
                    cache_read: 0.05,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-235b-a22b-thinking-2507",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 235B A22B Thinking 2507")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.45,
                    output: 3.5,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-30b-a3b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 30B A3B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(40_960),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.12,
                    output: 0.5,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-30b-a3b-instruct-2507",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 30B A3B Instruct 2507")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.3,
                    cache_read: 0.1,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-32b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 32B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(40_960),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.08,
                    output: 0.28,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-coder",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 Coder 480B A35B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.0,
                    cache_read: 0.1,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-coder-30b-a3b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 Coder 30B A3B Instruct")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-coder-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 Coder Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.195,
                    output: 0.975,
                    cache_read: 0.039,
                    cache_write: 0.24375,
                },
                tiers: Cow::Borrowed(&[
                    CostTier {
                        input_tokens_above: 32_000,
                        prices: Prices {
                            input: 0.325,
                            output: 1.625,
                            cache_read: 0.065,
                            cache_write: 0.40625,
                        },
                    },
                    CostTier {
                        input_tokens_above: 128_000,
                        prices: Prices {
                            input: 0.52,
                            output: 2.6,
                            cache_read: 0.104,
                            cache_write: 0.65,
                        },
                    },
                ]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-coder-next",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 Coder Next")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.12,
                    output: 0.8,
                    cache_read: 0.07,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-next-80b-a3b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 Next 80B A3B Instruct")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.09,
                    output: 1.1,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-vl-235b-a22b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 VL 235B A22B Instruct")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.21,
                    output: 1.9,
                    cache_read: 0.1,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-vl-30b-a3b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 VL 30B A3B Instruct")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-vl-30b-a3b-thinking",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 VL 30B A3B Thinking")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.29,
                    output: 1.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3-vl-8b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3 VL 8B Instruct")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 0.75,
                    cache_read: 0.12,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.5-122b-a10b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.5-122B-A10B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.26,
                    output: 2.08,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.5-27b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.5-27B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(81_920),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.26,
                    output: 2.6,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.5-35b-a3b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.5-35B-A3B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(81_920),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.14,
                    output: 1.0,
                    cache_read: 0.05,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.5-397b-a17b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.5 397B A17B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 3.6,
                    cache_read: 0.12,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.5-9b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.5-9B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.17,
                    output: 0.25,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.5-flash-02-23",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.5-Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.065,
                    output: 0.26,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.5-plus-02-15",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.5 Plus 2026-02-15")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.26,
                    output: 1.56,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 256_000,
                    prices: Prices {
                        input: 0.325,
                        output: 1.95,
                        cache_read: 0.0,
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.5-plus-20260420",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.5 Plus 2026-04-20")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.8,
                    cache_read: 0.0,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 256_000,
                    prices: Prices {
                        input: 0.375,
                        output: 2.25,
                        cache_read: 0.0,
                        cache_write: 0.46875,
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.6-27b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.6 27B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(81_920),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.32,
                    output: 3.2,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.6-35b-a3b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.6 35B A3B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.9,
                    cache_read: 0.025,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.6-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.6 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1875,
                    output: 1.125,
                    cache_read: 0.0,
                    cache_write: 0.234375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 256_000,
                    prices: Prices {
                        input: 0.75,
                        output: 3.0,
                        cache_read: 0.0,
                        cache_write: 0.9375,
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.6-plus",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.6 Plus")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.325,
                    output: 1.95,
                    cache_read: 0.0,
                    cache_write: 0.40625,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 256_000,
                    prices: Prices {
                        input: 1.3,
                        output: 3.9,
                        cache_read: 0.0,
                        cache_write: 1.625,
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.7-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.7 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.03,
                    output: 0.13,
                    cache_read: 0.006,
                    cache_write: 0.038,
                },
                tiers: Cow::Borrowed(&[
                    CostTier {
                        input_tokens_above: 32_000,
                        prices: Prices {
                            input: 0.1,
                            output: 0.4,
                            cache_read: 0.02,
                            cache_write: 0.125,
                        },
                    },
                    CostTier {
                        input_tokens_above: 256_000,
                        prices: Prices {
                            input: 0.2,
                            output: 0.8,
                            cache_read: 0.04,
                            cache_write: 0.25,
                        },
                    },
                ]),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.7-max",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.7 Max")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.475,
                    output: 4.425,
                    cache_read: 0.295,
                    cache_write: 1.84375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.7-plus",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.7 Plus")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.32,
                    output: 1.28,
                    cache_read: 0.064,
                    cache_write: 0.4,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 256_000,
                    prices: Prices {
                        input: 0.96,
                        output: 3.84,
                        cache_read: 0.192,
                        cache_write: 1.2,
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.7-plus:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.7 Plus (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.23,
                    output: 0.92,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.8-2.4t-a95b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.8 2.4T A95B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
                    cache_read: 0.25,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, None),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.8-27b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.8 27B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 1.875,
                    cache_read: 0.0375,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, None),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.8-27b:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.8 27B (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.16,
                    output: 2.0,
                    cache_read: 0.04,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, None),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.8-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.8 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.47,
                    cache_read: 0.016,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.8-max-0902",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.8 Max (0902)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
                    cache_read: 0.25,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.8-max-0902:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.8 Max (0902) (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.15,
                    output: 3.45,
                    cache_read: 0.0,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.8-max-prime",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.8 Max Prime")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 12.0,
                    cache_read: 0.5,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "qwen/qwen3.8-omni-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen: Qwen3.8 Omni Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.47,
                    cache_read: 0.016,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "recursive/flint-v1.0:free",
        ModelInfo {
            name: Some(Cow::Borrowed("Recursive: Flint (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "reflection/beam-1.0-501b-a23b",
        ModelInfo {
            name: Some(Cow::Borrowed("Reflection: Beam 1.0 501B A23B")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.0,
                    cache_read: 0.04,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "rekaai/reka-edge",
        ModelInfo {
            name: Some(Cow::Borrowed("Reka Edge")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(16_384),
            max_output_tokens: Some(14_745),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.1,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "relace/relace-search",
        ModelInfo {
            name: Some(Cow::Borrowed("Relace: Relace Search")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 3.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "ricursiveai/ripple-65b-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Ricursive: Ripple 65B Preview")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 0.6,
                    cache_read: 0.05,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "sakana/fugu-max",
        ModelInfo {
            name: Some(Cow::Borrowed("Sakana: Fugu Max")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
                    cache_read: 0.25,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "sakana/fugu-ultra",
        ModelInfo {
            name: Some(Cow::Borrowed("Sakana: Fugu Ultra")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 30.0,
                    cache_read: 0.5,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 10.0,
                        output: 45.0,
                        cache_read: 1.0,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, None),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "sakana/fugu-ultra-v2",
        ModelInfo {
            name: Some(Cow::Borrowed("Sakana: Fugu Ultra v2")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.0,
                    output: 30.0,
                    cache_read: 0.5,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 10.0,
                        output: 45.0,
                        cache_read: 1.0,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, None),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "sakana/sakana-namazu",
        ModelInfo {
            name: Some(Cow::Borrowed("Sakana: Sakana Namazu")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.95,
                    output: 4.0,
                    cache_read: 0.15,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "stepfun/step-3.5-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("StepFun: Step 3.5 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.3,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "stepfun/step-3.7-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("StepFun: Step 3.7 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(230_400),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 1.15,
                    cache_read: 0.04,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "stepfun/step-5-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("StepFun: Step 5 Preview")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "tencent/hy3",
        ModelInfo {
            name: Some(Cow::Borrowed("Tencent: Hy3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.132,
                    output: 0.528,
                    cache_read: 0.033,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "tencent/hy3-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Tencent: Hy3 preview")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.18,
                    output: 0.6,
                    cache_read: 0.06,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "tencent/hy4",
        ModelInfo {
            name: Some(Cow::Borrowed("Tencent: Hy4")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.417,
                    output: 1.2505,
                    cache_read: 0.021,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "tencent/hy4-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Tencent: Hy4 preview")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
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
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "thinkingmachines/inkling",
        ModelInfo {
            name: Some(Cow::Borrowed("Thinking Machines: Inkling")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(524_288),
            max_output_tokens: Some(471_859),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 4.05,
                    cache_read: 0.17,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "thinkingmachines/inkling-2-small:free",
        ModelInfo {
            name: Some(Cow::Borrowed("Thinking Machines: Inkling 2 Small (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
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
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("minimal"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "thinkingmachines/inkling-small",
        ModelInfo {
            name: Some(Cow::Borrowed("Thinking Machines: Inkling Small")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(524_288),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.45,
                    output: 1.2,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "thinkingmachines/inkling-small:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Thinking Machines: Inkling Small (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(524_288),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.36,
                    output: 0.96,
                    cache_read: 0.08,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "typesafe/jev-router",
        ModelInfo {
            name: Some(Cow::Borrowed("TypeSafe: Jev Router")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "unbiased/pareto-26.10",
        ModelInfo {
            name: Some(Cow::Borrowed("Pareto 26.10")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.8,
                    output: 3.2,
                    cache_read: 0.03,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "unbiased/pareto-26.9",
        ModelInfo {
            name: Some(Cow::Borrowed("Pareto 26.9")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 7.5,
                    cache_read: 0.25,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "upstage/solar-mini4",
        ModelInfo {
            name: Some(Cow::Borrowed("Upstage: Solar Mini 4")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(524_288),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.05,
                    output: 0.2,
                    cache_read: 0.005,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "upstage/solar-pro4",
        ModelInfo {
            name: Some(Cow::Borrowed("Upstage: Solar Pro 4")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(524_288),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.03,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "x-ai/grok-4.20",
        ModelInfo {
            name: Some(Cow::Borrowed("SpaceXAI: Grok 4.20")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(1_800_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 2.5,
                    cache_read: 0.2,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 2.5,
                        output: 5.0,
                        cache_read: 0.4,
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "x-ai/grok-4.3",
        ModelInfo {
            name: Some(Cow::Borrowed("SpaceXAI: Grok 4.3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(900_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 2.5,
                    cache_read: 0.2,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 2.5,
                        output: 5.0,
                        cache_read: 0.4,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "x-ai/grok-4.3:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("SpaceXAI: Grok 4.3 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(900_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 2.0,
                    cache_read: 0.16,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 2.0,
                        output: 4.0,
                        cache_read: 0.32,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "x-ai/grok-4.5",
        ModelInfo {
            name: Some(Cow::Borrowed("SpaceXAI: Grok 4.5")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(450_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
                    cache_read: 0.3,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 4.0,
                        output: 12.0,
                        cache_read: 0.6,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "x-ai/grok-4.6",
        ModelInfo {
            name: Some(Cow::Borrowed("SpaceXAI: Grok 4.6")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(450_000),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "x-ai/grok-4.6:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("SpaceXAI: Grok 4.6 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(450_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.6,
                    output: 4.8,
                    cache_read: 0.4,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 3.2,
                        output: 9.6,
                        cache_read: 0.8,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "x-ai/grok-4.7",
        ModelInfo {
            name: Some(Cow::Borrowed("SpaceXAI: Grok 4.7")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(450_000),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "x-ai/grok-4.7:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("SpaceXAI: Grok 4.7 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(450_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.6,
                    output: 4.8,
                    cache_read: 0.4,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 3.2,
                        output: 9.6,
                        cache_read: 0.8,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "x-ai/grok-build-0.1",
        ModelInfo {
            name: Some(Cow::Borrowed("SpaceXAI: Grok Build 0.1")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(230_400),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 2.0,
                    cache_read: 0.2,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 2.0,
                        output: 4.0,
                        cache_read: 0.4,
                        cache_write: 0.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "xiaomi/mimo-v2.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Xiaomi: MiMo-V2.5")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 2.0,
                    cache_read: 0.08,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "xiaomi/mimo-v2.5-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("Xiaomi: MiMo-V2.5-Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.48,
                    output: 1.8,
                    cache_read: 0.096,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "xiaomi/mimo-v2.6-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Xiaomi: MiMo-V2.6-Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "xiaomi/mimo-v2.6-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("Xiaomi: MiMo-V2.6-Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.435,
                    output: 0.87,
                    cache_read: 0.0036,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "xiaomi/mimo-v2.6-pro-ultraspeed",
        ModelInfo {
            name: Some(Cow::Borrowed("Xiaomi: MiMo-V2.6-Pro-UltraSpeed")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.35,
                    output: 8.7,
                    cache_read: 0.036,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "xingchen-agi/xing4.0-29b-a4b:free",
        ModelInfo {
            name: Some(Cow::Borrowed("XingChen: Xing4.0 29B A4B (free)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(235_929),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-4.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 4.5")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(98_304),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.2,
                    cache_read: 0.11,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-4.5-air",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 4.5 Air")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(98_304),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 1.1,
                    cache_read: 0.03,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-4.5v",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 4.5V")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(65_536),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 1.8,
                    cache_read: 0.11,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-4.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 4.6")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(202_752),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 2.0,
                    cache_read: 0.1,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-4.6v",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 4.6V")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 0.9,
                    cache_read: 0.055,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-4.7",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 4.7")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(204_800),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.2,
                    cache_read: 0.11,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-4.7-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 4.7 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(117_964),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0605,
                    output: 0.4,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(202_752),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 1.9,
                    cache_read: 0.119,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5-turbo",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5 Turbo")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(202_752),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.2,
                    output: 4.0,
                    cache_read: 0.24,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5.1")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(202_752),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5.2",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5.2")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0071,
                    output: 6.8,
                    cache_read: 0.0061,
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5.2:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5.2 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.527,
                    output: 1.075,
                    cache_read: 0.178,
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5.3",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5.3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0699,
                    output: 1.4,
                    cache_read: 0.0689,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5.3-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5.3 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_575),
            max_output_tokens: Some(943_717),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5.3-flash:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5.3 Flash (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.03,
                    output: 0.061,
                    cache_read: 0.013,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5.3-flashx",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5.3 FlashX")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.37,
                    output: 1.25,
                    cache_read: 0.075,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5.3-flashx-legal",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5.3 FlashX Legal")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.37,
                    output: 1.25,
                    cache_read: 0.075,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5.3-prime",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5.3 Prime")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.8,
                    output: 8.8,
                    cache_read: 0.56,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5.3:batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5.3 (batch)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.45,
                    output: 2.0,
                    cache_read: 0.1,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "z-ai/glm-5v-turbo",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM 5V Turbo")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(202_752),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.2,
                    output: 4.0,
                    cache_read: 0.24,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~anthropic/claude-fable-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Fable Latest")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 8.0,
                    output: 40.0,
                    cache_read: 0.2,
                    cache_write: 10.0,
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
            compat: Some(Cow::Borrowed(r#"{"forceAdaptiveThinking":true}"#)),
            ..BASE
        },
    ),
    (
        "~anthropic/claude-haiku-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Haiku Latest")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
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
            compat: Some(Cow::Borrowed(r#"{"forceAdaptiveThinking":true}"#)),
            ..BASE
        },
    ),
    (
        "~anthropic/claude-opus-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Opus Latest")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 20.0,
                    cache_read: 0.2,
                    cache_write: 5.0,
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
            compat: Some(Cow::Borrowed(r#"{"forceAdaptiveThinking":true}"#)),
            ..BASE
        },
    ),
    (
        "~anthropic/claude-sonnet-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Anthropic: Claude Sonnet Latest")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.1,
                    cache_write: 2.5,
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
            compat: Some(Cow::Borrowed(r#"{"forceAdaptiveThinking":true}"#)),
            ..BASE
        },
    ),
    (
        "~deepseek/deepseek-flash-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek Flash Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.006,
                    output: 0.5,
                    cache_read: 0.005,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~deepseek/deepseek-pro-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek Pro Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(393_216),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0999,
                    output: 8.0,
                    cache_read: 0.0999,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~deepseek/deepseek-v4-flash-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek: DeepSeek V4 Flash Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0434,
                    output: 0.3,
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
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true,"requiresReasoningContentOnAssistantMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~google/gemini-flash-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini Flash Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.75,
                    output: 3.75,
                    cache_read: 0.075,
                    cache_write: 0.041667,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~google/gemini-pro-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Google: Gemini Pro Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.1,
                    cache_write: 0.1875,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~moonshotai/kimi-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("MoonshotAI: Kimi Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1824,
                    output: 10.5601,
                    cache_read: 0.18,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~openai/gpt-astra-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT Astra Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 10.0,
                    output: 50.0,
                    cache_read: 1.0,
                    cache_write: 12.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 20.0,
                        output: 75.0,
                        cache_read: 2.0,
                        cache_write: 25.0,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~openai/gpt-luna-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT Luna Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~openai/gpt-mini-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT Mini Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.75,
                    output: 4.5,
                    cache_read: 0.075,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~openai/gpt-sol-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT Sol Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
                    cache_read: 0.1,
                    cache_write: 2.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.0,
                        output: 15.0,
                        cache_read: 0.2,
                        cache_write: 5.0,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~openai/gpt-terra-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("OpenAI: GPT Terra Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 5.0,
                    cache_read: 0.05,
                    cache_write: 1.25,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 2.0,
                        output: 7.5,
                        cache_read: 0.1,
                        cache_write: 2.5,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~unbiased/pareto-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Pareto Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.8,
                    output: 3.2,
                    cache_read: 0.03,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~x-ai/grok-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("xAI: Grok Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(450_000),
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~z-ai/glm-flash-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM Flash Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0335,
                    output: 0.083,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "~z-ai/glm-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Z.ai: GLM Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://openrouter.ai/api/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(943_718),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0699,
                    output: 1.4,
                    cache_read: 0.0689,
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
                r#"{"supportsDeveloperRole":false,"thinkingFormat":"openrouter","supportsStrictMode":true,"sendSessionAffinityHeaders":true}"#,
            )),
            ..BASE
        },
    ),
];
