//! The models of `GitHub` Copilot, pi's provider `github-copilot`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, CostTier, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices,
    ThinkingLevel,
};

/// Registers the 27 models pi lists for `github-copilot`.
#[derive(Clone, Copy, Debug, Default)]
pub struct GithubCopilotModelsPlugin;

impl Plugin for GithubCopilotModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("github-copilot")),
    base_url: Some(Cow::Borrowed("https://api.individual.githubcopilot.com")),
    input_limits: Some(InputLimits {
        max_request_bytes: None,
        images: Some(ImageLimits {
            max_per_message: None,
            max_per_request: None,
            resize: Some(ImageResize {
                max_width: Some(2_000),
                max_height: Some(2_000),
                max_bytes: Some(4_718_592),
                jpeg_quality: Some(80),
            }),
        }),
    }),
    headers: Cow::Borrowed(&[
        (
            Cow::Borrowed("User-Agent"),
            Cow::Borrowed("GitHubCopilotChat/0.35.0"),
        ),
        (
            Cow::Borrowed("Editor-Version"),
            Cow::Borrowed("vscode/1.107.0"),
        ),
        (
            Cow::Borrowed("Editor-Plugin-Version"),
            Cow::Borrowed("copilot-chat/0.35.0"),
        ),
        (
            Cow::Borrowed("Copilot-Integration-Id"),
            Cow::Borrowed("vscode-chat"),
        ),
    ]),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "claude-fable-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
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
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-fable-5.1",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Fable 5.1")),
            api: Some(Cow::Borrowed("anthropic-messages")),
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
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-haiku-4.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 4.5 (latest)")),
            api: Some(Cow::Borrowed("anthropic-messages")),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsEagerToolInputStreaming":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-haiku-5.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Haiku 5.5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
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
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-opus-4.8",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 4.8")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
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
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-opus-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
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
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-opus-5.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Opus 5.5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
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
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-sonnet-4.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 4.6")),
            api: Some(Cow::Borrowed("anthropic-messages")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(32_000),
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
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(r#"{"forceAdaptiveThinking":true}"#)),
            ..BASE
        },
    ),
    (
        "claude-sonnet-5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
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
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "claude-sonnet-5.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Claude Sonnet 5.5")),
            api: Some(Cow::Borrowed("anthropic-messages")),
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
            compat: Some(Cow::Borrowed(
                r#"{"forceAdaptiveThinking":true,"supportsTemperature":false,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gemini-3.8-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 3.8 Flash")),
            api: Some(Cow::Borrowed("openai-completions")),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "gemini-4-argon",
        ModelInfo {
            name: Some(Cow::Borrowed("Gemini 4 Argon")),
            api: Some(Cow::Borrowed("openai-completions")),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.3-codex",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.3 Codex")),
            api: Some(Cow::Borrowed("openai-responses")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
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
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(r#"{"supportsOpenAIGrammarTools":true}"#)),
            ..BASE
        },
    ),
    (
        "gpt-5.4-nano",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.4 nano")),
            api: Some(Cow::Borrowed("openai-responses")),
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
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, None),
            ]),
            compat: Some(Cow::Borrowed(r#"{"supportsOpenAIGrammarTools":true}"#)),
            ..BASE
        },
    ),
    (
        "gpt-5.6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Luna")),
            api: Some(Cow::Borrowed("openai-responses")),
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
                    input_tokens_above: 200_000,
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
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsMidConvoSystemMessages":true,"supportsAdditionalTools":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Sol")),
            api: Some(Cow::Borrowed("openai-responses")),
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
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsMidConvoSystemMessages":true,"supportsAdditionalTools":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-5.6-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-5.6 Terra")),
            api: Some(Cow::Borrowed("openai-responses")),
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
                (ThinkingLevel::Off, None),
                (ThinkingLevel::Minimal, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Low, Some(Cow::Borrowed("low"))),
                (ThinkingLevel::Medium, Some(Cow::Borrowed("medium"))),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsMidConvoSystemMessages":true,"supportsAdditionalTools":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-6-astra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Astra")),
            api: Some(Cow::Borrowed("openai-responses")),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsMidConvoSystemMessages":true,"supportsAdditionalTools":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-6-luna",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Luna")),
            api: Some(Cow::Borrowed("openai-responses")),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsMidConvoSystemMessages":true,"supportsAdditionalTools":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-6-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6 Sol")),
            api: Some(Cow::Borrowed("openai-responses")),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsMidConvoSystemMessages":true,"supportsAdditionalTools":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-6.1-sol",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Sol")),
            api: Some(Cow::Borrowed("openai-responses")),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsMidConvoSystemMessages":true,"supportsAdditionalTools":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "gpt-6.1-terra",
        ModelInfo {
            name: Some(Cow::Borrowed("GPT-6.1 Terra")),
            api: Some(Cow::Borrowed("openai-responses")),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsMidConvoSystemMessages":true,"supportsAdditionalTools":true,"supportsOpenAIGrammarTools":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "grok-4.5",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.5")),
            api: Some(Cow::Borrowed("openai-responses")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(128_000),
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
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, None),
            ]),
            ..BASE
        },
    ),
    (
        "grok-4.6",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.6")),
            api: Some(Cow::Borrowed("openai-responses")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(128_000),
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
            ..BASE
        },
    ),
    (
        "grok-4.7",
        ModelInfo {
            name: Some(Cow::Borrowed("Grok 4.7")),
            api: Some(Cow::Borrowed("openai-responses")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(500_000),
            max_output_tokens: Some(128_000),
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
            ..BASE
        },
    ),
    (
        "kimi-k3",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3")),
            api: Some(Cow::Borrowed("openai-completions")),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":true,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "mai-code-1.1-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("MAI-Code-1.1-Flash")),
            api: Some(Cow::Borrowed("openai-responses")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(256_000),
            max_output_tokens: Some(128_000),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.2,
                    output: 1.2,
                    cache_read: 0.02,
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
            ..BASE
        },
    ),
];
