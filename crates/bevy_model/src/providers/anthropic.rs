//! The models of Anthropic, pi's provider `anthropic`.

use alloc::borrow::Cow;
use core::time::Duration;

use bevy_app::{App, Plugin};

use crate::{
    Cost, CostTier, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices,
    PromptCache, ThinkingLevel,
};

/// Registers the 18 models pi lists for `anthropic`.
#[derive(Clone, Copy, Debug, Default)]
pub struct AnthropicModelsPlugin;

impl Plugin for AnthropicModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("anthropic")),
    api: Some(Cow::Borrowed("anthropic-messages")),
    base_url: Some(Cow::Borrowed("https://api.anthropic.com")),
    prompt_cache: Some(PromptCache {
        short: Some(Duration::from_secs(300)),
        long: Some(Duration::from_secs(3_600)),
    }),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "claude-fable-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5")),
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(600),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolChanges":true,"supportsStrictTools":true,"allowedFallbackModels":[{"provider":"anthropic","model":"claude-opus-4-8","cost":{"input":5,"output":25,"cacheRead":0.5,"cacheWrite":6.25}},{"provider":"anthropic","model":"claude-opus-5","cost":{"input":5,"output":25,"cacheRead":0.5,"cacheWrite":6.25}}]}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-fable-5-1",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5.1")),
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(600),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsMidConvoEffort":true,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolChanges":true,"supportsStrictTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-fable-5-2",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5.2")),
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(600),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsMidConvoEffort":true,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolChanges":true,"supportsStrictTools":true,"allowedFallbackModels":[{"provider":"anthropic","model":"claude-opus-5","cost":{"input":5,"output":25,"cacheRead":0.5,"cacheWrite":6.25}}]}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-haiku-4-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 4.5 (latest)")),
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
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(100),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictTools":true}"#)),
            ..BASE
        },
    ),
    (
        "claude-haiku-4-5-20251001",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 4.5")),
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
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(100),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictTools":true}"#)),
            ..BASE
        },
    ),
    (
        "claude-haiku-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 5.5")),
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
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(600),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoEffort":true,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolChanges":true,"supportsStrictTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-opus-4-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.5 (latest)")),
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
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(100),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictTools":true}"#)),
            ..BASE
        },
    ),
    (
        "claude-opus-4-5-20251101",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.5")),
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
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(100),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictTools":true}"#)),
            ..BASE
        },
    ),
    (
        "claude-opus-4-6",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.6")),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(600),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsStrictTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-opus-4-7",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.7")),
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(600),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsStrictTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-opus-4-8",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.8")),
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(600),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolChanges":true,"supportsStrictTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-opus-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5")),
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(600),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoEffort":true,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolChanges":true,"supportsStrictTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-opus-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5.5")),
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
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(600),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoEffort":true,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolChanges":true,"supportsStrictTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-sonnet-4-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.5 (latest)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.0,
                    output: 15.0,
                    cache_read: 0.3,
                    cache_write: 3.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(100),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictTools":true}"#)),
            ..BASE
        },
    ),
    (
        "claude-sonnet-4-5-20250929",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.5")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.0,
                    output: 15.0,
                    cache_read: 0.3,
                    cache_write: 3.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(100),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictTools":true}"#)),
            ..BASE
        },
    ),
    (
        "claude-sonnet-4-6",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.6")),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(600),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsStrictTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-sonnet-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5")),
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(600),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsStrictTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-sonnet-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5.5")),
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
                max_request_bytes: Some(33_554_432),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(600),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoEffort":true,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolChanges":true,"supportsStrictTools":true}"#,
            )),
            ..BASE
        },
    ),
];
