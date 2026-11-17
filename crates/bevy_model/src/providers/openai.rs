//! The models of `OpenAI`, pi's provider `openai`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, CostTier, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices,
    ThinkingLevel,
};

/// Registers the 44 models pi lists for `openai`.
#[derive(Clone, Copy, Debug, Default)]
pub struct OpenAiModelsPlugin;

impl Plugin for OpenAiModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("openai")),
    api: Some(Cow::Borrowed("openai-responses")),
    base_url: Some(Cow::Borrowed("https://api.openai.com/v1")),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "chat-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Chat Latest")),
            reasoning: Some(true),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "gpt-4",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(8_192),
            max_output_tokens: Some(8_192),
            cost: Some(Cost {
                prices: Prices {
                    input: 30.0,
                    output: 60.0,
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
        "gpt-4-turbo",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4 Turbo")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(4_096),
            cost: Some(Cost {
                prices: Prices {
                    input: 10.0,
                    output: 30.0,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "gpt-4.1",
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "gpt-4.1-mini",
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "gpt-4.1-nano",
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
                    cache_read: 0.025,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "gpt-4o",
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "gpt-4o-2024-05-13",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4o (2024-05-13)")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "gpt-4o-2024-08-06",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4o (2024-08-06)")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "gpt-4o-2024-11-20",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-4o (2024-11-20)")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "gpt-4o-mini",
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "gpt-5",
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5-chat-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 Chat Latest")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(128_000),
            max_output_tokens: Some(16_384),
            cost: Some(Cost {
                prices: Prices {
                    input: 1.25,
                    output: 10.0,
                    cache_read: 0.125,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            input_limits: Some(InputLimits {
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 Mini")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5-nano",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 Nano")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5 Pro")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.1",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.1")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.2",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.2")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.2-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.2 Pro")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.3-codex",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.3 Codex")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.3-codex-spark",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.3 Codex Spark")),
            reasoning: Some(true),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.4",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.4")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(272_000),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.4-mini",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.4 mini")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.4-nano",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.4 nano")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.4-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.4 Pro")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.5",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.5")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(272_000),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.5-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.5 Pro")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.6-cyber",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Cyber")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 12.5,
                    output: 75.0,
                    cache_read: 1.25,
                    cache_write: 15.625,
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsExplicitPromptCacheMode":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Luna")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(272_000),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsAdditionalTools":true,"supportsToolSearch":true,"supportsMidConvoSystemMessages":true,"supportsExplicitPromptCacheMode":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Sol")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(272_000),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsAdditionalTools":true,"supportsToolSearch":true,"supportsMidConvoSystemMessages":true,"supportsExplicitPromptCacheMode":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.6-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Terra")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(272_000),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsAdditionalTools":true,"supportsToolSearch":true,"supportsMidConvoSystemMessages":true,"supportsExplicitPromptCacheMode":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-6-astra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Astra")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(272_000),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsAdditionalTools":true,"supportsToolSearch":true,"supportsMidConvoSystemMessages":true,"supportsExplicitPromptCacheMode":true,"supportsMidConvoEffort":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Luna")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(272_000),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsAdditionalTools":true,"supportsToolSearch":true,"supportsMidConvoSystemMessages":true,"supportsExplicitPromptCacheMode":true,"supportsMidConvoEffort":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Sol")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(272_000),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsAdditionalTools":true,"supportsToolSearch":true,"supportsMidConvoSystemMessages":true,"supportsExplicitPromptCacheMode":true,"supportsMidConvoEffort":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-6.1-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Sol")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(272_000),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsAdditionalTools":true,"supportsToolSearch":true,"supportsMidConvoSystemMessages":true,"supportsExplicitPromptCacheMode":true,"supportsMidConvoEffort":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-6.1-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Terra")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(272_000),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsOpenAIGrammarTools":true,"supportsAdditionalTools":true,"supportsToolSearch":true,"supportsMidConvoSystemMessages":true,"supportsExplicitPromptCacheMode":true,"supportsMidConvoEffort":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-daybreak-blue-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Daybreak Blue")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsExplicitPromptCacheMode":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-daybreak-red-latest",
        ModelInfo {
            name: Some(Cow::Borrowed("Daybreak Red")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(400_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 12.5,
                    output: 75.0,
                    cache_read: 1.25,
                    cache_write: 15.625,
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
                    resize: Some(ImageResize {
                        max_width: Some(2_000),
                        max_height: Some(2_000),
                        max_bytes: Some(4_718_592),
                        jpeg_quality: Some(80),
                    }),
                }),
            }),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsExplicitPromptCacheMode":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "o1",
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "o1-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("o1-pro")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(200_000),
            max_output_tokens: Some(100_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 150.0,
                    output: 600.0,
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
            input_limits: Some(InputLimits {
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "o3",
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "o3-mini",
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
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(r#"{"supportsStrictMode":true}"#)),
            ..BASE
        },
    ),
    (
        "o3-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("o3-pro")),
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
        "o4-mini",
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
                max_request_bytes: Some(536_870_912),
                images: Some(ImageLimits {
                    max_per_message: None,
                    max_per_request: Some(1_500),
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
];
