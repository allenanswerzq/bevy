//! The models of Mistral, pi's provider `mistral`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices, ThinkingLevel,
};

/// Registers the 34 models pi lists for `mistral`.
#[derive(Clone, Copy, Debug, Default)]
pub struct MistralModelsPlugin;

impl Plugin for MistralModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("mistral")),
    api: Some(Cow::Borrowed("mistral-conversations")),
    base_url: Some(Cow::Borrowed("https://api.mistral.ai")),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "codestral-2508",
        ModelInfo {
            name: Some(Cow::Borrowed("Codestral 25.08")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 0.9,
                    cache_read: 0.03,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "codestral-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Codestral (latest)")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(256_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3,
                    output: 0.9,
                    cache_read: 0.03,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "deepseek-v4-1-flash-1017",
        ModelInfo {
            name: Some(Cow::Borrowed("DeepSeek V4.1 Flash 1017")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(128_000),
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
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            ..BASE
        },
    ),
    (
        "devstral-2512",
        ModelInfo {
            name: Some(Cow::Borrowed("Devstral 2")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 2.0,
                    cache_read: 0.04,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "devstral-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Devstral 2")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 2.0,
                    cache_read: 0.04,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "devstral-medium-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Devstral 2 (latest)")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.4,
                    output: 2.0,
                    cache_read: 0.04,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "glm-5-2",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.2")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
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
            ..BASE
        },
    ),
    (
        "labs-devstral-small-2512",
        ModelInfo {
            name: Some(Cow::Borrowed("Devstral Small 2")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
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
            input_limits: Some(InputLimits {
                max_request_bytes: None,
                images: Some(ImageLimits {
                    max_per_message: None,
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
        "labs-leanstral-1-5-1",
        ModelInfo {
            name: Some(Cow::Borrowed("Leanstral 1.5")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(128_000),
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
            ..BASE
        },
    ),
    (
        "magistral-medium-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Magistral Medium (latest)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 2.0,
                    output: 5.0,
                    cache_read: 0.2,
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
            ..BASE
        },
    ),
    (
        "ministral-14b-2512",
        ModelInfo {
            name: Some(Cow::Borrowed("Ministral 3 14B")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
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
            ..BASE
        },
    ),
    (
        "ministral-3b-2512",
        ModelInfo {
            name: Some(Cow::Borrowed("Ministral 3 3B")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(262_144),
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
            ..BASE
        },
    ),
    (
        "ministral-3b-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Ministral 3B (latest)")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(128_000),
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
            ..BASE
        },
    ),
    (
        "ministral-8b-2512",
        ModelInfo {
            name: Some(Cow::Borrowed("Ministral 3 8B")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
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
            ..BASE
        },
    ),
    (
        "ministral-8b-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Ministral 8B (latest)")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(128_000),
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
            ..BASE
        },
    ),
    (
        "mistral-large-2512",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Large 3")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
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
            ..BASE
        },
    ),
    (
        "mistral-large-4",
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
        "mistral-large-4-0-8",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Large 4.0.8")),
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
        "mistral-large-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Large (latest)")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
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
            ..BASE
        },
    ),
    (
        "mistral-medium-2505",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Medium 3")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(131_072),
            max_output_tokens: Some(131_072),
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
            ..BASE
        },
    ),
    (
        "mistral-medium-2508",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Medium 3.1")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
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
            ..BASE
        },
    ),
    (
        "mistral-medium-2604",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Medium 3.5")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.5,
                    output: 7.5,
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
            ..BASE
        },
    ),
    (
        "mistral-medium-3.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Medium 3.5")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
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
            ..BASE
        },
    ),
    (
        "mistral-medium-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Medium (latest)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(262_144),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.5,
                    output: 7.5,
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
            ..BASE
        },
    ),
    (
        "mistral-small-2506",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Small 3.2")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
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
            ..BASE
        },
    ),
    (
        "mistral-small-2603",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Small 4")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(256_000),
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
            ..BASE
        },
    ),
    (
        "mistral-small-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Mistral Small (latest)")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
            max_output_tokens: Some(256_000),
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
            ..BASE
        },
    ),
    (
        "open-mistral-nemo",
        ModelInfo {
            name: Some(Cow::Borrowed("Open Mistral Nemo")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(128_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.15,
                    output: 0.15,
                    cache_read: 0.015,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "pixtral-12b",
        ModelInfo {
            name: Some(Cow::Borrowed("Pixtral 12B")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(128_000),
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
            ..BASE
        },
    ),
    (
        "voxtral-small-2507",
        ModelInfo {
            name: Some(Cow::Borrowed("Voxtral Small")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(32_768),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.4,
                    cache_read: 0.01,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "voxtral-small-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Voxtral Small (latest)")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(32_768),
            max_output_tokens: Some(32_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1,
                    output: 0.4,
                    cache_read: 0.01,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "zai-glm-5-2",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.2")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
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
            ..BASE
        },
    ),
    (
        "zai-glm-5-3",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.3")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_048_576),
            max_output_tokens: Some(131_072),
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
            ..BASE
        },
    ),
    (
        "zai-glm-5-3-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.3-Flash")),
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
            ..BASE
        },
    ),
];
