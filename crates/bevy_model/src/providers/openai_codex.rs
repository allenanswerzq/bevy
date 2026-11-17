//! The models of `OpenAI` Codex, pi's provider `openai-codex`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, CostTier, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices,
    ThinkingLevel,
};

/// Registers the 8 models pi lists for `openai-codex`.
#[derive(Clone, Copy, Debug, Default)]
pub struct OpenAiCodexModelsPlugin;

impl Plugin for OpenAiCodexModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("openai-codex")),
    api: Some(Cow::Borrowed("openai-codex-responses")),
    base_url: Some(Cow::Borrowed("https://chatgpt.com/backend-api")),
    input_limits: Some(InputLimits {
        max_request_bytes: None,
        images: Some(ImageLimits {
            max_per_message: None,
            max_per_request: None,
            resize: Some(ImageResize {
                max_width: Some(2_000),
                max_height: Some(2_000),
                max_bytes: Some(4_718_592),
                jpeg_quality: Some(80),
            }),
        }),
    }),
    compat: Some(Cow::Borrowed(
        r#"{"supportsOpenAIGrammarTools":true,"supportsAdditionalTools":true,"supportsToolSearch":true,"supportsMidConvoSystemMessages":true}"#,
    )),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
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
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
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
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
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
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
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
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
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
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
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
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            ..BASE
        },
    ),
    (
        "gpt-6.1-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Sol")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
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
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            ..BASE
        },
    ),
    (
        "gpt-6.1-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Terra")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
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
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            ..BASE
        },
    ),
];
