//! The models of Amazon Bedrock, pi's provider `amazon-bedrock`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, CostTier, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices,
    ThinkingLevel,
};

/// Registers the 202 models pi lists for `amazon-bedrock`.
#[derive(Clone, Copy, Debug, Default)]
pub struct AmazonBedrockModelsPlugin;

impl Plugin for AmazonBedrockModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("amazon-bedrock")),
    api: Some(Cow::Borrowed("bedrock-converse-stream")),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "amazon.nova-2-lite-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova 2 Lite")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.33,
                    output: 2.75,
                    cache_read: 0.0825,
                    cache_write: 0.33,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "amazon.nova-lite-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Lite")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.24,
                    cache_read: 0.015,
                    cache_write: 0.06,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "amazon.nova-micro-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Micro")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.035,
                    output: 0.14,
                    cache_read: 0.00875,
                    cache_write: 0.035,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "amazon.nova-pro-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Pro")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.8,
                    output: 3.2,
                    cache_read: 0.2,
                    cache_write: 0.8,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "anthropic.claude-fable-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-fable-5-1",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5.1")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-fable-5-2",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5.2")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-haiku-4-5-20251001-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 4.5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-haiku-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 5.5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-opus-4-1-20250805-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.1")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                    max_per_message: Some(20),
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
        "anthropic.claude-opus-4-5-20251101-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-opus-4-6-v1",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.6")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-opus-4-7",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.7")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-opus-4-8",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.8")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-opus-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5.5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-sonnet-4-5-20250929-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-sonnet-4-6",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.6")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.3,
                    output: 16.5,
                    cache_read: 0.33,
                    cache_write: 4.125,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-sonnet-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic.claude-sonnet-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5.5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "apac.amazon.nova-lite-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Lite (APAC)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.063,
                    output: 0.252,
                    cache_read: 0.01575,
                    cache_write: 0.063,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "apac.amazon.nova-micro-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Micro (APAC)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.037,
                    output: 0.148,
                    cache_read: 0.00925,
                    cache_write: 0.037,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "apac.amazon.nova-pro-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Pro (APAC)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.84,
                    output: 3.36,
                    cache_read: 0.21,
                    cache_write: 0.84,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "apac.anthropic.claude-sonnet-4-20250514-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4 (APAC)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "au.anthropic.claude-haiku-4-5-20251001-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 4.5 (AU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.1,
                    output: 5.5,
                    cache_read: 0.11,
                    cache_write: 1.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "au.anthropic.claude-haiku-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 5.5 (AU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.11,
                    output: 0.55,
                    cache_read: 0.011,
                    cache_write: 0.1375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 100_000,
                    prices: Prices {
                        input: 0.55,
                        output: 2.75,
                        cache_read: 0.055,
                        cache_write: 0.6875,
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
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "au.anthropic.claude-opus-4-6-v1",
        ModelInfo {
            name: Some(Cow::Borrowed("AU Anthropic Claude Opus 4.6")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "au.anthropic.claude-opus-4-7",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.7 (AU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "au.anthropic.claude-opus-4-8",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.8 (AU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "au.anthropic.claude-opus-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5 (AU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "au.anthropic.claude-opus-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5.5 (AU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.4,
                    output: 22.0,
                    cache_read: 0.22,
                    cache_write: 5.5,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "au.anthropic.claude-sonnet-4-5-20250929-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.5 (AU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.3,
                    output: 16.5,
                    cache_read: 0.33,
                    cache_write: 4.125,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "au.anthropic.claude-sonnet-4-6",
        ModelInfo {
            name: Some(Cow::Borrowed("AU Anthropic Claude Sonnet 4.6")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.3,
                    output: 16.5,
                    cache_read: 0.33,
                    cache_write: 4.125,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "au.anthropic.claude-sonnet-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5 (AU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 11.0,
                    cache_read: 0.22,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "au.anthropic.claude-sonnet-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5.5 (AU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 11.0,
                    cache_read: 0.11,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "ca.amazon.nova-lite-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Lite (CA)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.064,
                    output: 0.256,
                    cache_read: 0.016,
                    cache_write: 0.064,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "deepseek.v3-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek-V3.1")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(163_840),
            max_output_tokens: Some(81_920),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.58,
                    output: 1.68,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "deepseek.v3.2",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V3.2")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(163_840),
            max_output_tokens: Some(81_920),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.62,
                    output: 1.85,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.amazon.nova-2-lite-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova 2 Lite (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.374,
                    output: 3.157,
                    cache_read: 0.0935,
                    cache_write: 0.374,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "eu.amazon.nova-lite-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Lite (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.069,
                    output: 0.276,
                    cache_read: 0.01725,
                    cache_write: 0.069,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "eu.amazon.nova-micro-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Micro (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.04,
                    output: 0.16,
                    cache_read: 0.01,
                    cache_write: 0.04,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "eu.amazon.nova-pro-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Pro (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.92,
                    output: 3.68,
                    cache_read: 0.23,
                    cache_write: 0.92,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "eu.anthropic.claude-fable-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 11.0,
                    output: 55.0,
                    cache_read: 1.1,
                    cache_write: 13.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.anthropic.claude-haiku-4-5-20251001-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 4.5 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.1,
                    output: 5.5,
                    cache_read: 0.11,
                    cache_write: 1.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.anthropic.claude-haiku-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 5.5 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.11,
                    output: 0.55,
                    cache_read: 0.011,
                    cache_write: 0.1375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 100_000,
                    prices: Prices {
                        input: 0.55,
                        output: 2.75,
                        cache_read: 0.055,
                        cache_write: 0.6875,
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
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.anthropic.claude-opus-4-5-20251101-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.5 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.anthropic.claude-opus-4-6-v1",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.6 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.anthropic.claude-opus-4-7",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.7 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.anthropic.claude-opus-4-8",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.8 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.anthropic.claude-opus-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.anthropic.claude-opus-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5.5 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.4,
                    output: 22.0,
                    cache_read: 0.22,
                    cache_write: 5.5,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.anthropic.claude-sonnet-4-20250514-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "eu.anthropic.claude-sonnet-4-5-20250929-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.5 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.3,
                    output: 16.5,
                    cache_read: 0.33,
                    cache_write: 4.125,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.anthropic.claude-sonnet-4-6",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.6 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.3,
                    output: 16.5,
                    cache_read: 0.33,
                    cache_write: 4.125,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.anthropic.claude-sonnet-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 11.0,
                    cache_read: 0.22,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.anthropic.claude-sonnet-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5.5 (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 11.0,
                    cache_read: 0.11,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "eu.mistral.pixtral-large-2502-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Pixtral Large (25.02) (EU)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.eu-central-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "global.amazon.nova-2-lite-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova 2 Lite (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 2.5,
                    cache_read: 0.075,
                    cache_write: 0.3,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "global.anthropic.claude-fable-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-fable-5-1",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5.1 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-fable-5-2",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5.2 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-haiku-4-5-20251001-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 4.5 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-haiku-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 5.5 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-opus-4-5-20251101-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.5 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-opus-4-6-v1",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.6 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-opus-4-7",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.7 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-opus-4-8",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.8 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-opus-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-opus-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5.5 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-sonnet-4-20250514-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "global.anthropic.claude-sonnet-4-5-20250929-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.5 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-sonnet-4-6",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.6 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-sonnet-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.anthropic.claude-sonnet-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5.5 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.moonshotai.kimi-k3",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.moonshotai.kimi-k3.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3.1 Pro Highspeed (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 20.0,
                    cache_read: 0.2,
                    cache_write: 5.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.openai.gpt-5.6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Luna (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.openai.gpt-5.6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Sol (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.openai.gpt-5.6-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Terra (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.openai.gpt-6-astra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Astra (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.openai.gpt-6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Luna (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.openai.gpt-6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Sol (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.openai.gpt-6.1-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Sol (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.openai.gpt-6.1-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Terra (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.xai.grok-4.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.6 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "global.xai.grok-4.7",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.7 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "global.zai.glm-5.3",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.3 (Global)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.68,
                    output: 5.28,
                    cache_read: 0.312,
                    cache_write: 2.1,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "google.gemma-4-26b-a4b",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemma 4 26B A4B IT")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.13,
                    output: 0.4,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "google.gemma-4-31b",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemma 4 31B IT")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.14,
                    output: 0.4,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "google.gemma-4-e2b",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemma 4 E2B IT")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.04,
                    output: 0.08,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "in.anthropic.claude-haiku-4-5-20251001-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 4.5 (India)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.1,
                    output: 5.5,
                    cache_read: 0.11,
                    cache_write: 1.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "in.anthropic.claude-opus-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5 (India)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "in.anthropic.claude-sonnet-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5 (India)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 11.0,
                    cache_read: 0.22,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "in.openai.gpt-5.6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Luna (India)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.22,
                    output: 1.32,
                    cache_read: 0.022,
                    cache_write: 0.275,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 0.44,
                        output: 1.98,
                        cache_read: 0.044,
                        cache_write: 0.55,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "in.openai.gpt-5.6-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Terra (India)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 13.2,
                    cache_read: 0.22,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.4,
                        output: 19.8,
                        cache_read: 0.44,
                        cache_write: 5.5,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "jp.amazon.nova-2-lite-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova 2 Lite (JP)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.396,
                    output: 3.311,
                    cache_read: 0.099,
                    cache_write: 0.396,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "jp.anthropic.claude-haiku-4-5-20251001-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 4.5 (JP)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.1,
                    output: 5.5,
                    cache_read: 0.11,
                    cache_write: 1.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "jp.anthropic.claude-haiku-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 5.5 (JP)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.11,
                    output: 0.55,
                    cache_read: 0.011,
                    cache_write: 0.1375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 100_000,
                    prices: Prices {
                        input: 0.55,
                        output: 2.75,
                        cache_read: 0.055,
                        cache_write: 0.6875,
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
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "jp.anthropic.claude-opus-4-7",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.7 (JP)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "jp.anthropic.claude-opus-4-8",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.8 (JP)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "jp.anthropic.claude-opus-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5 (JP)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "jp.anthropic.claude-opus-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5.5 (JP)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.4,
                    output: 22.0,
                    cache_read: 0.22,
                    cache_write: 5.5,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "jp.anthropic.claude-sonnet-4-5-20250929-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.5 (JP)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.3,
                    output: 16.5,
                    cache_read: 0.33,
                    cache_write: 4.125,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "jp.anthropic.claude-sonnet-4-6",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.6 (JP)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.3,
                    output: 16.5,
                    cache_read: 0.33,
                    cache_write: 4.125,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "jp.anthropic.claude-sonnet-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5 (JP)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 11.0,
                    cache_read: 0.22,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "meta.llama3-1-70b-instruct-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 3.1 70B Instruct")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.72,
                    output: 0.72,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "meta.llama3-1-8b-instruct-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 3.1 8B Instruct")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.22,
                    output: 0.22,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "meta.llama3-3-70b-instruct-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 3.3 70B Instruct")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.72,
                    output: 0.72,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "meta.llama4-maverick-17b-instruct-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 4 Maverick 17B Instruct")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.24,
                    output: 0.97,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "meta.llama4-scout-17b-instruct-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 4 Scout 17B Instruct")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(10_000_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.17,
                    output: 0.66,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "minimax.minimax-m2",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax-M2")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(204_608),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "minimax.minimax-m2.1",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax-M2.1")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(196_608),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "minimax.minimax-m2.5",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax-M2.5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(196_608),
            max_output_tokens: Some(98_304),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral.devstral-2-123b",
        ModelInfo {
            name: Some(Cow::Borrowed("Devstral 2 123B")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 2.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral.magistral-small-2509",
        ModelInfo {
            name: Some(Cow::Borrowed("Magistral Small 1.2")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(40_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 1.5,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral.ministral-3-14b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Ministral 14B 3.0")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 0.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral.ministral-3-3b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Ministral 3 3B")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(8_192),
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
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral.ministral-3-8b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Ministral 3 8B")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.15,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral.mistral-large-3-675b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Large 3")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 1.5,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral.pixtral-large-2502-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Pixtral Large (25.02)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "mistral.voxtral-mini-3b-2507",
        ModelInfo {
            name: Some(Cow::Borrowed("Voxtral Mini 3B 2507")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(32_768),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.04,
                    output: 0.04,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral.voxtral-small-24b-2507",
        ModelInfo {
            name: Some(Cow::Borrowed("Voxtral Small 24B 2507")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(32_768),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.3,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "moonshot.kimi-k2-thinking",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K2 Thinking")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(16_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.5,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "moonshotai.kimi-k2.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K2.5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 3.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "nvidia.nemotron-nano-12b-v2",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA Nemotron Nano 12B v2 VL BF16")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 0.6,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "nvidia.nemotron-nano-3-30b",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA Nemotron Nano 3 30B")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.24,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "nvidia.nemotron-nano-9b-v2",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA Nemotron Nano 9B v2")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.23,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "nvidia.nemotron-super-3-120b",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA Nemotron 3 Super 120B A12B")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.65,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-5.4",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.4")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.75,
                    output: 16.5,
                    cache_read: 0.275,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-5.5",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 33.0,
                    cache_read: 0.55,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-5.6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Luna")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.22,
                    output: 1.32,
                    cache_read: 0.022,
                    cache_write: 0.275,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 0.44,
                        output: 1.98,
                        cache_read: 0.044,
                        cache_write: 0.55,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-5.6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Sol")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.4,
                    output: 22.0,
                    cache_read: 0.44,
                    cache_write: 5.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 8.8,
                        output: 33.0,
                        cache_read: 0.88,
                        cache_write: 11.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-5.6-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Terra")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 13.2,
                    cache_read: 0.22,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.4,
                        output: 19.8,
                        cache_read: 0.44,
                        cache_write: 5.5,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-6-astra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Astra")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 11.0,
                    output: 55.0,
                    cache_read: 1.1,
                    cache_write: 13.75,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 22.0,
                        output: 82.5,
                        cache_read: 2.2,
                        cache_write: 27.5,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Luna")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.11,
                    output: 0.55,
                    cache_read: 0.011,
                    cache_write: 0.1375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 0.22,
                        output: 0.825,
                        cache_read: 0.022,
                        cache_write: 0.275,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Sol")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 11.0,
                    cache_read: 0.22,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.4,
                        output: 16.5,
                        cache_read: 0.44,
                        cache_write: 5.5,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-6.1-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Sol")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 11.0,
                    cache_read: 0.11,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.4,
                        output: 16.5,
                        cache_read: 0.22,
                        cache_write: 5.5,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-6.1-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Terra")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.1,
                    output: 5.5,
                    cache_read: 0.055,
                    cache_write: 1.375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 2.2,
                        output: 8.25,
                        cache_read: 0.11,
                        cache_write: 2.75,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-oss-120b",
        ModelInfo {
            name: Some(Cow::Borrowed("gpt-oss-120b")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-oss-120b-1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("gpt-oss-120b")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-oss-20b",
        ModelInfo {
            name: Some(Cow::Borrowed("gpt-oss-20b")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.07,
                    output: 0.3,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-oss-20b-1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("gpt-oss-20b")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.07,
                    output: 0.3,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-oss-safeguard-120b",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT OSS Safeguard 120B")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
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
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "openai.gpt-oss-safeguard-20b",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT OSS Safeguard 20B")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.07,
                    output: 0.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "qwen.qwen3-235b-a22b-2507-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3 235B-A22B Instruct 2507")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.22,
                    output: 0.88,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "qwen.qwen3-32b-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3 32B")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(32_768),
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
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "qwen.qwen3-coder-30b-a3b-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3-Coder 30B-A3B Instruct")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "qwen.qwen3-coder-480b-a35b-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3-Coder 480B-A35B Instruct")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.45,
                    output: 1.8,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "qwen.qwen3-coder-next",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3 Coder Next")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 1.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "qwen.qwen3-next-80b-a3b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3-Next 80B-A3B Instruct")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 1.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "qwen.qwen3-vl-235b-a22b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3 VL 235B A22B Instruct")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.53,
                    output: 2.66,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us-gov.openai.gpt-oss-120b-1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("gpt-oss-120b (GovCloud)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.18,
                    output: 0.72,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us-gov.openai.gpt-oss-20b-1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("gpt-oss-20b (GovCloud)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.084,
                    output: 0.36,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.amazon.nova-2-lite-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova 2 Lite (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.33,
                    output: 2.75,
                    cache_read: 0.0825,
                    cache_write: 0.33,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "us.amazon.nova-lite-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Lite (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.24,
                    cache_read: 0.015,
                    cache_write: 0.06,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "us.amazon.nova-micro-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Micro (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.035,
                    output: 0.14,
                    cache_read: 0.00875,
                    cache_write: 0.035,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "us.amazon.nova-premier-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Premier (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 12.5,
                    cache_read: 0.625,
                    cache_write: 2.5,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "us.amazon.nova-pro-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Pro (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(10_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.8,
                    output: 3.2,
                    cache_read: 0.2,
                    cache_write: 0.8,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "us.anthropic.claude-fable-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 11.0,
                    output: 55.0,
                    cache_read: 1.1,
                    cache_write: 13.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-fable-5-1",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5.1 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 11.0,
                    output: 55.0,
                    cache_read: 0.275,
                    cache_write: 13.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-fable-5-2",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5.2 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 8.8,
                    output: 44.0,
                    cache_read: 0.22,
                    cache_write: 11.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-haiku-4-5-20251001-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 4.5 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.1,
                    output: 5.5,
                    cache_read: 0.11,
                    cache_write: 1.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-haiku-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 5.5 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.11,
                    output: 0.55,
                    cache_read: 0.011,
                    cache_write: 0.1375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 100_000,
                    prices: Prices {
                        input: 0.55,
                        output: 2.75,
                        cache_read: 0.055,
                        cache_write: 0.6875,
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
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-opus-4-1-20250805-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.1 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                    max_per_message: Some(20),
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
        "us.anthropic.claude-opus-4-5-20251101-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.5 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-opus-4-6-v1",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.6 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-opus-4-7",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.7 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-opus-4-8",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.8 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-opus-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 5.5,
                    output: 27.5,
                    cache_read: 0.55,
                    cache_write: 6.875,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-opus-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5.5 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.4,
                    output: 22.0,
                    cache_read: 0.22,
                    cache_write: 5.5,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-sonnet-4-20250514-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "us.anthropic.claude-sonnet-4-5-20250929-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.5 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.3,
                    output: 16.5,
                    cache_read: 0.33,
                    cache_write: 4.125,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-sonnet-4-6",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.6 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.3,
                    output: 16.5,
                    cache_read: 0.33,
                    cache_write: 4.125,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-sonnet-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 11.0,
                    cache_read: 0.22,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.anthropic.claude-sonnet-5-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5.5 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 11.0,
                    cache_read: 0.11,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.meta.llama3-1-70b-instruct-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 3.1 70B Instruct (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.72,
                    output: 0.72,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "us.meta.llama3-1-8b-instruct-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 3.1 8B Instruct (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.22,
                    output: 0.22,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "us.meta.llama3-3-70b-instruct-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 3.3 70B Instruct (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.72,
                    output: 0.72,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "us.meta.llama4-maverick-17b-instruct-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 4 Maverick 17B Instruct (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.24,
                    output: 0.97,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "us.meta.llama4-scout-17b-instruct-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 4 Scout 17B Instruct (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(10_000_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.17,
                    output: 0.66,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "us.mistral.pixtral-large-2502-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Pixtral Large (25.02) (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "us.moonshotai.kimi-k3",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.3,
                    output: 16.5,
                    cache_read: 0.33,
                    cache_write: 4.125,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.moonshotai.kimi-k3.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3.1 Pro Highspeed (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.4,
                    output: 22.0,
                    cache_read: 0.22,
                    cache_write: 5.5,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.openai.gpt-5.6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Luna (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.22,
                    output: 1.32,
                    cache_read: 0.022,
                    cache_write: 0.275,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 0.44,
                        output: 1.98,
                        cache_read: 0.044,
                        cache_write: 0.55,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.openai.gpt-5.6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Sol (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.4,
                    output: 22.0,
                    cache_read: 0.44,
                    cache_write: 5.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 8.8,
                        output: 33.0,
                        cache_read: 0.88,
                        cache_write: 11.0,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.openai.gpt-5.6-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Terra (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 13.2,
                    cache_read: 0.22,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.4,
                        output: 19.8,
                        cache_read: 0.44,
                        cache_write: 5.5,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.openai.gpt-6-astra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Astra (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 11.0,
                    output: 55.0,
                    cache_read: 1.1,
                    cache_write: 13.75,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 22.0,
                        output: 82.5,
                        cache_read: 2.2,
                        cache_write: 27.5,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.openai.gpt-6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Luna (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.11,
                    output: 0.55,
                    cache_read: 0.011,
                    cache_write: 0.1375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 0.22,
                        output: 0.825,
                        cache_read: 0.022,
                        cache_write: 0.275,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.openai.gpt-6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Sol (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 11.0,
                    cache_read: 0.22,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.4,
                        output: 16.5,
                        cache_read: 0.44,
                        cache_write: 5.5,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.openai.gpt-6.1-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Sol (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 11.0,
                    cache_read: 0.11,
                    cache_write: 2.75,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 4.4,
                        output: 16.5,
                        cache_read: 0.22,
                        cache_write: 5.5,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.openai.gpt-6.1-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Terra (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.1,
                    output: 5.5,
                    cache_read: 0.055,
                    cache_write: 1.375,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 2.2,
                        output: 8.25,
                        cache_read: 0.11,
                        cache_write: 2.75,
                    },
                }]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.writer.palmyra-x4-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Palmyra X4 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(122_880),
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
            ..BASE
        },
    ),
    (
        "us.writer.palmyra-x5-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Palmyra X5 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_040_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 6.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "us.xai.grok-4.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.6 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(500_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 6.6,
                    cache_read: 0.55,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
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
        "us.xai.grok-4.7",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.7 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(500_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 6.6,
                    cache_read: 0.55,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "us.zai.glm-5.3",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.3 (US)")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.848,
                    output: 5.808,
                    cache_read: 0.3432,
                    cache_write: 2.31,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "writer.palmyra-x4-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Palmyra X4")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(122_880),
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
            ..BASE
        },
    ),
    (
        "writer.palmyra-x5-v1:0",
        ModelInfo {
            name: Some(Cow::Borrowed("Palmyra X5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_040_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 6.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "xai.grok-4.3",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.3")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 2.5,
                    cache_read: 0.2,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "xai.grok-4.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.6")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(500_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.2,
                    output: 6.6,
                    cache_read: 0.55,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: Some(20),
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "zai.glm-4.7",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-4.7")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(202_752),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "zai.glm-4.7-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-4.7-Flash")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(202_752),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.07,
                    output: 0.4,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "zai.glm-5",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5")),
            base_url: Some(Cow::Borrowed(
                "https://bedrock-runtime.us-east-1.amazonaws.com",
            )),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(202_752),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 3.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
];
