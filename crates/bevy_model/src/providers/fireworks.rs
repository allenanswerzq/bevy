//! The models of Fireworks, pi's provider `fireworks`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices, ThinkingLevel,
};

/// Registers the 27 models pi lists for `fireworks`.
#[derive(Clone, Copy, Debug, Default)]
pub struct FireworksModelsPlugin;

impl Plugin for FireworksModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("fireworks")),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "accounts/fireworks/models/deepseek-v4p1-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.006,
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/models/deepseek-v4p1-flash-1017",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash 1017")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.006,
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/models/ember-1",
        ModelInfo {
            name: Some(Cow::Borrowed("Ember-1")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/models/glm-5p3",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_573),
            max_output_tokens: Some(262_144),
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
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/models/glm-5p3-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_573),
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
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/models/kimi-k3",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
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
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"requiresReasoningContentOnAssistantMessages":true,"thinkingFormat":"openai","supportsMidConvoSystemMessages":true,"supportsMidConvoToolAdditions":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/models/kimi-k3p1-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3.1 Pro")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference/v1")),
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
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"requiresReasoningContentOnAssistantMessages":true,"thinkingFormat":"openai","supportsMidConvoSystemMessages":true,"supportsMidConvoToolAdditions":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/models/minimax-m3",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax-M3")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(512_000),
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/models/minimax-m3p1-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax-M3.1-Flash")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/models/muse-spark-open",
        ModelInfo {
            name: Some(Cow::Borrowed("Muse Spark Open")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/models/qwen3p8-2p4t-a95b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3.8 2.4T A95B")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
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
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, None),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/models/qwen3p8-max",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3.8 Max")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/deepseek-flash-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek Flash Latest")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.006,
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/deepseek-v4p1-flash-1017-spot",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash 1017 Spot")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.21,
                    output: 0.84,
                    cache_read: 0.0042,
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/deepseek-v4p1-flash-1017-ultrafast",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash 1017 Ultra-fast")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.4,
                    output: 9.6,
                    cache_read: 0.048,
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/deepseek-v4p1-flash-spot",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash Spot")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.21,
                    output: 0.84,
                    cache_read: 0.0042,
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/glm-5p3-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3 Fast")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_572),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.1,
                    output: 6.6,
                    cache_read: 0.39,
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
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/glm-5p3-spot",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3 Spot")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_573),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.98,
                    output: 3.08,
                    cache_read: 0.182,
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
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/glm-fast-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3 Fast (Latest)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_572),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.1,
                    output: 6.6,
                    cache_read: 0.39,
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
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/glm-flash-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM Flash Latest (GLM 5.3 Flash)")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_573),
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
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/glm-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM Latest")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_573),
            max_output_tokens: Some(262_144),
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
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/kimi-fast-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi Fast Latest")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.5,
                    output: 22.5,
                    cache_read: 0.45,
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/kimi-k3-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3 Fast")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.5,
                    output: 22.5,
                    cache_read: 0.45,
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
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"requiresReasoningContentOnAssistantMessages":true,"thinkingFormat":"openai","supportsMidConvoSystemMessages":true,"supportsMidConvoToolAdditions":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/kimi-k3-spot",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3 Spot")),
            api: Some(Cow::Borrowed("openai-completions")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference/v1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.1,
                    output: 10.5,
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
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"sendSessionAffinityHeaders":true,"supportsLongCacheRetention":false,"requiresReasoningContentOnAssistantMessages":true,"thinkingFormat":"openai","supportsMidConvoSystemMessages":true,"supportsMidConvoToolAdditions":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/kimi-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi Latest")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/minimax-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax Latest")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(512_000),
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "accounts/fireworks/routers/qwen-max-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen Max Latest (Qwen3.8 Max)")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            base_url: Some(Cow::Borrowed("https://api.fireworks.ai/inference")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
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
                r#"{"allowEmptySignature":true,"sendSessionAffinityHeaders":true,"supportsEagerToolInputStreaming":false,"supportsCacheControlOnTools":false,"supportsLongCacheRetention":false,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
];
