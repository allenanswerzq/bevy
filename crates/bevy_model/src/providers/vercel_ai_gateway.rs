//! The models of Vercel AI Gateway, pi's provider `vercel-ai-gateway`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, CostTier, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices,
    ThinkingLevel,
};

/// Registers the 307 models pi lists for `vercel-ai-gateway`.
#[derive(Clone, Copy, Debug, Default)]
pub struct VercelAiGatewayModelsPlugin;

impl Plugin for VercelAiGatewayModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("vercel-ai-gateway")),
    api: Some(Cow::Borrowed("anthropic-messages")),
    base_url: Some(Cow::Borrowed("https://ai-gateway.vercel.sh")),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "alibaba/qwen-3-14b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3-14B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(40_960),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.12,
                    output: 0.24,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen-3-235b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3 235B A22B")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.22,
                    output: 0.88,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen-3-30b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3-30B-A3B")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen-3-32b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3 32B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3-coder",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3 Coder 480B A35B Instruct")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.22,
                    output: 1.8,
                    cache_read: 0.022,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3-coder-30b-a3b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3 Coder 30B A3B Instruct")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3-coder-next",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3 Coder Next")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(256_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 1.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3-max-thinking",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3 Max Thinking")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.2,
                    output: 6.0,
                    cache_read: 0.24,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[
                    CostTier {
                        input_tokens_above: 32_000,
                        prices: Prices {
                            input: 2.4,
                            output: 12.0,
                            cache_read: 0.48,
                            cache_write: 0.0,
                        },
                    },
                    CostTier {
                        input_tokens_above: 128_000,
                        prices: Prices {
                            input: 3.0,
                            output: 15.0,
                            cache_read: 0.6,
                            cache_write: 0.0,
                        },
                    },
                ]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3-next-80b-a3b-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3 Next 80B A3B Instruct")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_114),
            max_output_tokens: Some(262_114),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 1.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3-next-80b-a3b-thinking",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3 Next 80B A3B Thinking")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 1.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3-vl-instruct",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3 VL 235B A22B Instruct")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 0.88,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3.5-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3.5 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.4,
                    cache_read: 0.01,
                    cache_write: 0.125,
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
        "alibaba/qwen3.5-plus",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3.5 Plus")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 2.4,
                    cache_read: 0.04,
                    cache_write: 0.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 256_000,
                    prices: Prices {
                        input: 0.5,
                        output: 3.0,
                        cache_read: 0.05,
                        cache_write: 0.625,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3.6-27b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3.6 27B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(256_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 3.6,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3.6-plus",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3.6 Plus")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 3.0,
                    cache_read: 0.05,
                    cache_write: 0.625,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 255_999,
                    prices: Prices {
                        input: 2.0,
                        output: 6.0,
                        cache_read: 0.2,
                        cache_write: 2.5,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3.7-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3.7 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(991_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.03,
                    output: 0.13,
                    cache_read: 0.006,
                    cache_write: 0.038,
                },
                tiers: Cow::Borrowed(&[
                    CostTier {
                        input_tokens_above: 31_999,
                        prices: Prices {
                            input: 0.1,
                            output: 0.4,
                            cache_read: 0.02,
                            cache_write: 0.125,
                        },
                    },
                    CostTier {
                        input_tokens_above: 255_999,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3.7-max",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3.7 Max")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(991_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 7.5,
                    cache_read: 0.5,
                    cache_write: 3.125,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3.7-plus",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3.7 Plus")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 1.6,
                    cache_read: 0.08,
                    cache_write: 0.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 255_999,
                    prices: Prices {
                        input: 1.2,
                        output: 4.8,
                        cache_read: 0.24,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3.8-2.4t-a95b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3.8 2.4T A95B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "alibaba/qwen3.8-27b",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3.8 27B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 2.3,
                    cache_read: 0.059,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3.8-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3.8 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(991_000),
            max_output_tokens: Some(128_000),
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
        "alibaba/qwen3.8-max",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3.8 Max")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "alibaba/qwen3.8-max-0902",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen3.8 Max 0902")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(991_000),
            max_output_tokens: Some(128_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "alibaba/qwen3.8-max-prime",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3.8 Max Prime")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 12.0,
                    cache_read: 0.5,
                    cache_write: 5.0,
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
        "alibaba/qwen3.8-omni-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Qwen 3.8 Omni Flash")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "amazon/nova-2-lite",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova 2 Lite")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(1_000_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.33,
                    output: 2.75,
                    cache_read: 0.0825,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "amazon/nova-lite",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Lite")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.24,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "amazon/nova-micro",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Micro")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.035,
                    output: 0.14,
                    cache_read: 0.0087,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "amazon/nova-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("Nova Pro")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(300_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.8,
                    output: 3.2,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5",
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5 (Batch)")),
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5.1",
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5.1-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5.1 (Batch)")),
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5.2",
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5.2-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5.2 (Batch)")),
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-fable-5.2-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5.2 (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 16.0,
                    output: 80.0,
                    cache_read: 0.4,
                    cache_write: 20.0,
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
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-haiku-4.5",
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "anthropic/claude-haiku-4.5-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 4.5 (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic/claude-haiku-5.5",
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-haiku-5.5-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 5.5 (Batch)")),
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.5",
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "anthropic/claude-opus-4.5-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.5 (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.6",
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.6-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.6 (Batch)")),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.7",
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.7-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.7 (Batch)")),
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.8",
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.8-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.8 (Batch)")),
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-4.8-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.8 (Fast)")),
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-5",
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-5-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5 (Batch)")),
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-5-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5 (Fast)")),
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-5.5",
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-5.5-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5.5 (Batch)")),
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
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-opus-5.5-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5.5 (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 8.0,
                    output: 40.0,
                    cache_read: 0.4,
                    cache_write: 10.0,
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
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-4.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.5")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-4.5-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.5 (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-4.6",
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-4.6-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.6 (Batch)")),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Max, Some(Cow::Borrowed("max")))]),
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-5",
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
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: None,
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-5-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5 (Batch)")),
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-5.5",
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "anthropic/claude-sonnet-5.5-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5.5 (Batch)")),
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
                r#"{"allowEmptySignature":true,"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "bytedance/seed-1.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Seed 1.6")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 2.0,
                    cache_read: 0.05,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 128_000,
                    prices: Prices {
                        input: 0.5,
                        output: 4.0,
                        cache_read: 0.05,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "bytedance/seed-1.8",
        ModelInfo {
            name: Some(Cow::Borrowed("Bytedance Seed 1.8")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 2.0,
                    cache_read: 0.05,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 128_000,
                    prices: Prices {
                        input: 0.5,
                        output: 4.0,
                        cache_read: 0.05,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "bytedance/seed-2.1-turbo",
        ModelInfo {
            name: Some(Cow::Borrowed("Seed 2.1 Turbo")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 2.5,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "callstack/apex",
        ModelInfo {
            name: Some(Cow::Borrowed("Apex")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 3.0,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "cohere/command-a",
        ModelInfo {
            name: Some(Cow::Borrowed("Command A")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(8_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 10.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-r1",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek-R1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.35,
                    output: 5.4,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v3.1",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V3.1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(163_840),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 0.95,
                    cache_read: 0.13,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v3.2",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V3.2")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.62,
                    output: 1.85,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v3.2-thinking",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V3.2 Thinking")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.62,
                    output: 1.85,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4-flash-0731",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4 Flash 0731")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4-flash-vision-exp",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4 Flash Vision Exp")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2156,
                    output: 0.6468,
                    cache_read: 0.0068,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4 Pro")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_600),
            max_output_tokens: Some(1_048_600),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.32,
                    output: 3.96,
                    cache_read: 0.132,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4-pro-0813",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4 Pro 0813")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(384_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.66,
                    output: 1.98,
                    cache_read: 0.066,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4.1-flash",
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "deepseek/deepseek-v4.1-flash-1017",
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "deepseek/deepseek-v4.1-flash-1017-fast",
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "deepseek/deepseek-v4.1-flash-1017-ultrafast",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash 1017 Ultrafast")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.4,
                    output: 9.6,
                    cache_read: 0.048,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "deepseek/deepseek-v4.1-flash-fast",
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "fireworks/ember-1",
        ModelInfo {
            name: Some(Cow::Borrowed("Ember-1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.0,
                    output: 15.0,
                    cache_read: 0.3,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-2.5-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 2.5 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 2.5,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-2.5-flash-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 2.5 Flash (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 1.25,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-2.5-flash-lite",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 2.5 Flash Lite")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.4,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-2.5-flash-lite-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 2.5 Flash Lite (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.05,
                    output: 0.2,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-2.5-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 2.5 Pro")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 10.0,
                    cache_read: 0.125,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 2.5,
                        output: 15.0,
                        cache_read: 0.25,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-2.5-pro-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 2.5 Pro (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.625,
                    output: 5.0,
                    cache_read: 0.125,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 1.25,
                        output: 7.5,
                        cache_read: 0.25,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.5,
                    output: 3.0,
                    cache_read: 0.05,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 0.5,
                        output: 3.0,
                        cache_read: 0.05,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3-flash-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3 Flash (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 1.5,
                    cache_read: 0.05,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 0.25,
                        output: 1.5,
                        cache_read: 0.05,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3.1-flash-lite",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3.1 Flash Lite")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 1.5,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3.1-flash-lite-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3.1 Flash Lite (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.125,
                    output: 0.75,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3.1-pro-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3.1 Pro Preview")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 12.0,
                    cache_read: 0.2,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 4.0,
                        output: 18.0,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3.1-pro-preview-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3.1 Pro Preview (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 6.0,
                    cache_read: 0.2,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 200_000,
                    prices: Prices {
                        input: 2.0,
                        output: 9.0,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3.5-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3.5 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.5,
                    output: 9.0,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3.5-flash-lite",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3.5 Flash Lite")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 2.5,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3.5-flash-lite-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3.5 Flash Lite (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 1.25,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3.6-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3.6 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.75,
                    output: 3.75,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3.6-flash-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3.6 Flash (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.375,
                    output: 1.875,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3.7-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3.7 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.75,
                    output: 3.75,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-3.8-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3.8 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_535),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.75,
                    output: 3.75,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemini-4-argon",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 4 Argon")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(1_000_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 10.0,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemma-4-26b-a4b-it",
        ModelInfo {
            name: Some(Cow::Borrowed("Google Gemma 4 26B A4B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "google/gemma-4-31b-it",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemma 4 31B IT")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
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
                    max_per_message: None,
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
        "humansand/ampersand-v0.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Ampersand v0.1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "inception/mercury-2",
        ModelInfo {
            name: Some(Cow::Borrowed("Mercury 2")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 0.75,
                    cache_read: 0.025,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "inception/mercury-2.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Mercury 2.5")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(260_000),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.04,
                    output: 0.15,
                    cache_read: 0.004,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "inception/mercury-3",
        ModelInfo {
            name: Some(Cow::Borrowed("Mercury 3")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.35,
                    cache_read: 0.01,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "inception/mercury-coder-small",
        ModelInfo {
            name: Some(Cow::Borrowed("Mercury Coder Small Beta")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(32_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 1.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "inclusionai/ling-3.0-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Ling 3.0 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.021,
                    output: 0.063,
                    cache_read: 0.0042,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "inclusionai/ling-3.0-flash-fin",
        ModelInfo {
            name: Some(Cow::Borrowed("Ling 3.0 Flash Fin")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.075,
                    output: 0.22,
                    cache_read: 0.015,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "inclusionai/ling-3.0-flash-sante",
        ModelInfo {
            name: Some(Cow::Borrowed("Ling 3.0 Flash Sante")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.075,
                    output: 0.22,
                    cache_read: 0.015,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "inclusionai/ling-3.0-flash-vl",
        ModelInfo {
            name: Some(Cow::Borrowed("Ling 3.0 Flash VL")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.075,
                    output: 0.22,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "inclusionai/ling-3.1-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Ling 3.1 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 0.9,
                    cache_read: 0.06,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "inclusionai/ling-3.1-flash-1019",
        ModelInfo {
            name: Some(Cow::Borrowed("Ling 3.1 Flash 1019")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 0.9,
                    cache_read: 0.06,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "interfaze/interfaze-beta",
        ModelInfo {
            name: Some(Cow::Borrowed("Interfaze Beta")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.5,
                    output: 3.5,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "liquid/lfm2.5-24b-a2b",
        ModelInfo {
            name: Some(Cow::Borrowed("LFM2.5-24B-A2B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_200),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.05,
                    output: 0.7,
                    cache_read: 0.025,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "meituan/longcat-2.5",
        ModelInfo {
            name: Some(Cow::Borrowed("LongCat 2.5")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "meta/llama-3.1-70b",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 3.1 70B Instruct")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.72,
                    output: 0.72,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "meta/llama-3.1-8b",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 3.1 8B Instruct")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.22,
                    output: 0.22,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "meta/llama-3.3-70b",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 3.3 70B Instruct")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.72,
                    output: 0.72,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "meta/llama-4-maverick",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 4 Maverick 17B Instruct")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
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
                    max_per_message: None,
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
        "meta/llama-4-scout",
        ModelInfo {
            name: Some(Cow::Borrowed("Llama 4 Scout 17B Instruct")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
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
                    max_per_message: None,
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
        "meta/muse-ember",
        ModelInfo {
            name: Some(Cow::Borrowed("Muse Ember")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.5,
                    cache_read: 0.004,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
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
                    input: 0.35,
                    output: 1.5,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "meta/muse-spark-1.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Muse Spark 1.1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 4.25,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "meta/muse-spark-1.2",
        ModelInfo {
            name: Some(Cow::Borrowed("Muse Spark 1.2")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 4.25,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "meta/muse-spark-1.2-contributor",
        ModelInfo {
            name: Some(Cow::Borrowed("Muse Spark 1.2 Contributor")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.2,
                    cache_read: 0.002,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "meta/muse-spark-1.3",
        ModelInfo {
            name: Some(Cow::Borrowed("Muse Spark 1.3")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 4.25,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "meta/muse-spark-1.3-contributor",
        ModelInfo {
            name: Some(Cow::Borrowed("Muse Spark 1.3 Contributor")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.2,
                    cache_read: 0.002,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "meta/muse-spark-open",
        ModelInfo {
            name: Some(Cow::Borrowed("Muse Spark Open")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 4.25,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "meta/muse-wonder-1.0",
        ModelInfo {
            name: Some(Cow::Borrowed("Muse Wonder")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 20.0,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m2",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax M2")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(205_000),
            max_output_tokens: Some(205_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.03,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m2.1",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax M2.1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(204_800),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.03,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m2.1-lightning",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax M2.1 Lightning")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(204_800),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 2.4,
                    cache_read: 0.03,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m2.5",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax M2.5")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(204_800),
            max_output_tokens: Some(131_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.03,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m2.5-highspeed",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax M2.5 High Speed")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(204_800),
            max_output_tokens: Some(131_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.4,
                    cache_read: 0.03,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m2.7",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax M2.7")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(204_800),
            max_output_tokens: Some(131_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 1.2,
                    cache_read: 0.06,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m2.7-highspeed",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax M2.7 High Speed")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(204_800),
            max_output_tokens: Some(131_100),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.4,
                    cache_read: 0.06,
                    cache_write: 0.375,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "minimax/minimax-m3",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax M3")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "minimax/minimax-m3.1-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax-M3.1-Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral/codestral",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Codestral")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 0.9,
                    cache_read: 0.03,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral/ministral-14b",
        ModelInfo {
            name: Some(Cow::Borrowed("Ministral 14B")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(256_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral/ministral-3b",
        ModelInfo {
            name: Some(Cow::Borrowed("Ministral 3B")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(4_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral/ministral-8b",
        ModelInfo {
            name: Some(Cow::Borrowed("Ministral 8B")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(4_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral/mistral-large-3",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Large 3")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(256_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral/mistral-large-4",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Large 4")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "mistral/mistral-medium-3.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Medium Latest")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(256_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.5,
                    output: 7.5,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral/mistral-nemo",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Nemo 12B")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.029,
                    output: 0.03,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "mistral/mistral-small",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Small")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(4_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "mixedbread/toast-1",
        ModelInfo {
            name: Some(Cow::Borrowed("Toast 1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_000),
            max_output_tokens: Some(4_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 0.72,
                    cache_read: 0.036,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k2",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K2 Instruct")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.57,
                    output: 2.3,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k2.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K2.5")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(256_000),
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
                    max_per_message: None,
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
        "moonshotai/kimi-k2.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K2.6")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_000),
            max_output_tokens: Some(262_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.95,
                    output: 4.0,
                    cache_read: 0.16,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k2.7-code",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K2.7 Code")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(32_768),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k2.7-code-highspeed",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K2.7 Code High Speed")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.9,
                    output: 8.0,
                    cache_read: 0.38,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "moonshotai/kimi-k3",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "moonshotai/kimi-k3-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3 Fast")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "moonshotai/kimi-k3.1-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3.1 Pro")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "moonshotai/kimi-k3.1-pro-highspeed",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3.1 Pro Highspeed")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 20.0,
                    cache_read: 0.2,
                    cache_write: 4.0,
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
        "nvidia/nemotron-3-nano-30b-a3b",
        ModelInfo {
            name: Some(Cow::Borrowed("Nemotron 3 Nano 30B A3B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.05,
                    output: 0.2,
                    cache_read: 0.025,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3-super-120b-a12b",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA Nemotron 3 Super 120B A12B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.65,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3-ultra-550b-a55b",
        ModelInfo {
            name: Some(Cow::Borrowed("Nemotron 3 Ultra")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.4,
                    cache_read: 0.12,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3.5-lightning",
        ModelInfo {
            name: Some(Cow::Borrowed("Nemotron 3.5 Lightning 30B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.16,
                    cache_read: 0.03,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3.5-lightning-free",
        ModelInfo {
            name: Some(Cow::Borrowed("Nemotron 3.5 Lightning Free")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(65_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-3.5-super-vl",
        ModelInfo {
            name: Some(Cow::Borrowed("NVIDIA Nemotron 3.5 Super VL")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 0.5,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "nvidia/nemotron-nano-12b-v2-vl",
        ModelInfo {
            name: Some(Cow::Borrowed("Nvidia Nemotron Nano 12B V2 VL")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(131_072),
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
                    max_per_message: None,
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
        "nvidia/nemotron-nano-9b-v2",
        ModelInfo {
            name: Some(Cow::Borrowed("Nvidia Nemotron Nano 9B V2")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.23,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4.1")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4.1 (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4.1 (Fast)")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_047_576),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.5,
                    output: 14.0,
                    cache_read: 0.875,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4.1 mini")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1-mini-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4.1 mini (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1-mini-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4.1 mini (Fast)")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_047_576),
            max_output_tokens: Some(32_768),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.7,
                    output: 2.8,
                    cache_read: 0.175,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4.1-nano",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4.1 nano")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_047_576),
            max_output_tokens: Some(32_768),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4o")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4o (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4o (Fast)")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.25,
                    output: 17.0,
                    cache_read: 2.125,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4o mini")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o-mini-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4o mini (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-4o-mini-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4o mini (Fast)")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.25,
                    output: 1.0,
                    cache_read: 0.125,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/gpt-5-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 (Batch)")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/gpt-5-codex",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5-Codex")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/gpt-5-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 20.0,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 mini")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/gpt-5-mini-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 mini (Batch)")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/gpt-5-mini-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 mini (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.45,
                    output: 3.6,
                    cache_read: 0.045,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5-nano",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 nano")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/gpt-5-nano-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 nano (Batch)")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/gpt-5-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 pro")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(272_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 15.0,
                    output: 120.0,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5-pro-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 pro (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(272_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 7.5,
                    output: 60.0,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.1-codex",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.1-Codex")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/gpt-5.1-codex-max",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.1 Codex Max")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/gpt-5.1-codex-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.1 Codex Mini")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/gpt-5.1-thinking",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.1 Thinking")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/gpt-5.1-thinking-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.1 Thinking (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 20.0,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.2",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.2")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.2-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.2 (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.2-codex",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.2 Codex")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.2-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.2 (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.5,
                    output: 28.0,
                    cache_read: 0.35,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.2-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.2 ")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.3-codex",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.3 Codex")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.3-codex-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.3 Codex (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.5,
                    output: 28.0,
                    cache_read: 0.35,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.4")),
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
                    input_tokens_above: 271_999,
                    prices: Prices {
                        input: 5.0,
                        output: 22.5,
                        cache_read: 0.5,
                        cache_write: 0.0,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.4 (Batch)")),
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
                    input_tokens_above: 271_999,
                    prices: Prices {
                        input: 2.5,
                        output: 11.25,
                        cache_read: 0.25,
                        cache_write: 0.0,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.4 (Fast)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.4 Mini")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-mini-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.4 Mini (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-mini-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.4 Mini (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.5,
                    output: 9.0,
                    cache_read: 0.15,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-nano",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.4 Nano")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-nano-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.4 Nano (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.4-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.4 Pro")),
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
                    input_tokens_above: 271_999,
                    prices: Prices {
                        input: 60.0,
                        output: 270.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.5",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.5")),
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
                    input_tokens_above: 271_999,
                    prices: Prices {
                        input: 10.0,
                        output: 45.0,
                        cache_read: 1.0,
                        cache_write: 0.0,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.5-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.5 (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.5,
                    output: 15.0,
                    cache_read: 0.25,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 271_999,
                    prices: Prices {
                        input: 5.0,
                        output: 22.5,
                        cache_read: 0.5,
                        cache_write: 0.0,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.5-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.5 (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 12.5,
                    output: 75.0,
                    cache_read: 1.25,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.5-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.5 Pro")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 30.0,
                    output: 180.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 271_999,
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.5-pro-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.5 Pro (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 15.0,
                    output: 90.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 271_999,
                    prices: Prices {
                        input: 30.0,
                        output: 135.0,
                        cache_read: 0.0,
                        cache_write: 0.0,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.6 Luna")),
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
                    input_tokens_above: 271_999,
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
                    max_per_message: None,
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
        "openai/gpt-5.6-luna-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.6 Luna (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.6,
                    cache_read: 0.01,
                    cache_write: 0.125,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 271_999,
                    prices: Prices {
                        input: 0.2,
                        output: 0.9,
                        cache_read: 0.02,
                        cache_write: 0.25,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-luna-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.6 Luna (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 2.4,
                    cache_read: 0.04,
                    cache_write: 0.5,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 271_999,
                    prices: Prices {
                        input: 0.8,
                        output: 3.6,
                        cache_read: 0.08,
                        cache_write: 1.0,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.6 Sol")),
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
                    input_tokens_above: 271_999,
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
                    max_per_message: None,
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
        "openai/gpt-5.6-sol-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.6 Sol (Batch)")),
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
                    input_tokens_above: 271_999,
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
                    max_per_message: None,
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
        "openai/gpt-5.6-sol-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.6 Sol (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 8.0,
                    output: 40.0,
                    cache_read: 0.8,
                    cache_write: 10.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 271_999,
                    prices: Prices {
                        input: 16.0,
                        output: 60.0,
                        cache_read: 1.6,
                        cache_write: 20.0,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.6 Terra")),
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
                    input_tokens_above: 271_999,
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
                    max_per_message: None,
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
        "openai/gpt-5.6-terra-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.6 Terra (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 6.0,
                    cache_read: 0.1,
                    cache_write: 1.25,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 271_999,
                    prices: Prices {
                        input: 2.0,
                        output: 9.0,
                        cache_read: 0.2,
                        cache_write: 2.5,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-5.6-terra-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT 5.6 Terra (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 24.0,
                    cache_read: 0.4,
                    cache_write: 5.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 271_999,
                    prices: Prices {
                        input: 8.0,
                        output: 36.0,
                        cache_read: 0.8,
                        cache_write: 10.0,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-astra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Astra")),
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
                    max_per_message: None,
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
        "openai/gpt-6-astra-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Astra (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-astra-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Astra (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 20.0,
                    output: 100.0,
                    cache_read: 2.0,
                    cache_write: 25.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 40.0,
                        output: 150.0,
                        cache_read: 4.0,
                        cache_write: 50.0,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Luna")),
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
                    max_per_message: None,
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
        "openai/gpt-6-luna-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Luna (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-luna-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Luna (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 1.0,
                    cache_read: 0.02,
                    cache_write: 0.25,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 0.4,
                        output: 1.5,
                        cache_read: 0.04,
                        cache_write: 0.5,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Sol")),
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
                    max_per_message: None,
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
        "openai/gpt-6-sol-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Sol (Batch)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-6-sol-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Sol (Fast)")),
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
                    max_per_message: None,
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
        "openai/gpt-6.1-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Sol")),
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
                    max_per_message: None,
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
        "openai/gpt-6.1-sol-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Sol (Batch)")),
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
                    max_per_message: None,
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
        "openai/gpt-6.1-sol-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Sol (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_050_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 20.0,
                    cache_read: 0.2,
                    cache_write: 5.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 272_000,
                    prices: Prices {
                        input: 8.0,
                        output: 30.0,
                        cache_read: 0.4,
                        cache_write: 10.0,
                    },
                }]),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-6.1-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Terra")),
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
                    max_per_message: None,
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
        "openai/gpt-6.1-terra-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Terra (Fast)")),
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
                    max_per_message: None,
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
        "openai/gpt-oss-120b",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT OSS 120B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(131_072),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.5,
                    cache_read: 0.1,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
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
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.03,
                    output: 0.14,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-oss-safeguard-120b",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT OSS Safeguard 120B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.6,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/gpt-oss-safeguard-20b",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT OSS Safeguard 20B")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.07,
                    output: 0.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/o1",
        ModelInfo {
            name: Some(Cow::Borrowed("o1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(100_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 15.0,
                    output: 60.0,
                    cache_read: 7.5,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/o3",
        ModelInfo {
            name: Some(Cow::Borrowed("o3")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/o3-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("o3 (Fast)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(100_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 3.5,
                    output: 14.0,
                    cache_read: 0.875,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/o3-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("o3-mini")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(200_000),
            max_output_tokens: Some(100_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.1,
                    output: 4.4,
                    cache_read: 0.55,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "openai/o3-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("o3 Pro")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "openai/o4-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("o4-mini")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(100_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.1,
                    output: 4.4,
                    cache_read: 0.275,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "particle/particle-1",
        ModelInfo {
            name: Some(Cow::Borrowed("Particle-1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.125,
                    output: 0.5,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "poolside/laguna-s-2.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Laguna S 2.1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.09,
                    output: 0.18,
                    cache_read: 0.009,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "quiverai/arrow-2",
        ModelInfo {
            name: Some(Cow::Borrowed("Arrow 2")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 4.0,
                    output: 20.0,
                    cache_read: 0.4,
                    cache_write: 5.0,
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
        "quiverai/arrow-2-telos",
        ModelInfo {
            name: Some(Cow::Borrowed("Arrow 2 Telos")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 6.0,
                    output: 30.0,
                    cache_read: 0.6,
                    cache_write: 7.5,
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
        "reflection/beam",
        ModelInfo {
            name: Some(Cow::Borrowed("Beam 1.0")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "reflection/beam-free",
        ModelInfo {
            name: Some(Cow::Borrowed("Beam 1.0 (Free)")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "sakana/fugu-max",
        ModelInfo {
            name: Some(Cow::Borrowed("Fugu Max")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(1_000_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 6.0,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "sakana/fugu-ultra",
        ModelInfo {
            name: Some(Cow::Borrowed("Fugu Ultra")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(1_000_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "sakana/fugu-ultra-v2",
        ModelInfo {
            name: Some(Cow::Borrowed("Fugu Ultra v2")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(1_000_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "sakana/namazu",
        ModelInfo {
            name: Some(Cow::Borrowed("Sakana Namazu")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(256_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.95,
                    output: 4.0,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "spacexai/grok-4.20-multi-agent",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.20 Multi-Agent")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(2_000_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "spacexai/grok-4.20-multi-agent-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.20 Multi-Agent (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(2_000_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "spacexai/grok-4.20-multi-agent-beta",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.20 Multi Agent Beta")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(2_000_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "spacexai/grok-4.20-non-reasoning",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.20 Non-Reasoning")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(2_000_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "spacexai/grok-4.20-non-reasoning-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.20 Non-Reasoning (Batch)")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(2_000_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "spacexai/grok-4.20-non-reasoning-beta",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.20 Beta Non-Reasoning")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(2_000_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "spacexai/grok-4.20-reasoning",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.20 Reasoning")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(2_000_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "spacexai/grok-4.20-reasoning-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.20 Reasoning (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(2_000_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "spacexai/grok-4.20-reasoning-beta",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.20 Beta Reasoning")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(2_000_000),
            max_output_tokens: Some(2_000_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "spacexai/grok-4.3",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.3")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(1_000_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "spacexai/grok-4.3-batch",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.3 (Batch)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(1_000_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "spacexai/grok-4.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.5")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(500_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "spacexai/grok-4.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.6")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "spacexai/grok-4.7",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.7")),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "spacexai/grok-build-0.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok Build 0.1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(256_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "spacexai/grok-voice-3.0",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok Voice 3.0")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(4_096),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "stealth/glyph-cluster",
        ModelInfo {
            name: Some(Cow::Borrowed("Glyph Cluster")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(256_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.0,
                    output: 0.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "stepfun/step-3.7-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Step 3.7 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(256_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 1.15,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "stepfun/step-5-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Step 5 Preview")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(1_048_576),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 2.7,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "tencent/hy3",
        ModelInfo {
            name: Some(Cow::Borrowed("Tencent Hy3")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.14,
                    output: 0.58,
                    cache_read: 0.035,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "tencent/hy4",
        ModelInfo {
            name: Some(Cow::Borrowed("Tencent Hy4")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_024_000),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.417,
                    output: 1.2505,
                    cache_read: 0.021,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "tencent/hy4-preview",
        ModelInfo {
            name: Some(Cow::Borrowed("Tencent Hy4 Preview")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "thinkingmachines/inkling",
        ModelInfo {
            name: Some(Cow::Borrowed("Inkling")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(256_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 4.05,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "thinkingmachines/inkling-small",
        ModelInfo {
            name: Some(Cow::Borrowed("Inkling Small")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(1_000_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.45,
                    output: 1.2,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "xiaomi/mimo-v2.6-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("MiMo V2.6 Flash")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "xiaomi/mimo-v2.6-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("MiMo V2.6 Pro")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "xiaomi/mimo-v2.6-pro-ultraspeed",
        ModelInfo {
            name: Some(Cow::Borrowed("MiMo V2.6 Pro UltraSpeed")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-4.5",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 4.5")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(96_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.2,
                    cache_read: 0.11,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-4.5-air",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 4.5 Air")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(96_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 1.1,
                    cache_read: 0.03,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-4.5v",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 4.5V")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(66_000),
            max_output_tokens: Some(16_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-4.6",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 4.6")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(200_000),
            max_output_tokens: Some(96_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.2,
                    cache_read: 0.11,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-4.7",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 4.7")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(200_000),
            max_output_tokens: Some(120_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6,
                    output: 2.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-4.7-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 4.7 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(200_000),
            max_output_tokens: Some(131_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.07,
                    output: 0.4,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-4.7-flashx",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 4.7 FlashX")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(200_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.4,
                    cache_read: 0.01,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-5",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(202_800),
            max_output_tokens: Some(131_100),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.0,
                    output: 3.2,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-5-turbo",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5 Turbo")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(202_800),
            max_output_tokens: Some(131_100),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.2,
                    output: 4.0,
                    cache_read: 0.24,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-5.1",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.1")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(202_800),
            max_output_tokens: Some(64_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.4,
                    output: 4.4,
                    cache_read: 0.26,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-5.2",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.2")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.4,
                    output: 4.4,
                    cache_read: 0.14,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-5.2-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.2 Fast")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.8,
                    output: 8.8,
                    cache_read: 0.56,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-5.3",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(1_000_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.4,
                    output: 4.4,
                    cache_read: 0.14,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-5.3-fast",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3 Fast")),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-5.3-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3 Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(131_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.5,
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
    (
        "zai/glm-5.3-flashx",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3 FlashX")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "zai/glm-5.3-flashx-legal",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5.3 FlashX Legal")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "zai/glm-5v-turbo",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM 5V Turbo")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(128_000),
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
            compat: Some(Cow::Borrowed(r#"{"allowEmptySignature":true}"#)),
            ..BASE
        },
    ),
];
