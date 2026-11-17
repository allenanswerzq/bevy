//! The models of Kimi For Coding, pi's provider `kimi-coding`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices, ThinkingLevel,
};

/// Registers the 6 models pi lists for `kimi-coding`.
#[derive(Clone, Copy, Debug, Default)]
pub struct KimiCodingModelsPlugin;

impl Plugin for KimiCodingModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("kimi-coding")),
    api: Some(Cow::Borrowed("openai-completions")),
    base_url: Some(Cow::Borrowed("https://api.kimi.com/coding/v1")),
    input_limits: Some(InputLimits {
        max_request_bytes: None,
        images: Some(ImageLimits {
            max_per_message: None,
            max_per_request: None,
            resize: Some(ImageResize {
                max_width: Some(2_000),
                max_height: Some(2_000),
                max_bytes: Some(4_718_592),
                jpeg_quality: Some(80),
            }),
        }),
    }),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "k3",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3")),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":false,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"maxTokensField":"max_tokens","thinkingFormat":"openai","requiresReasoningContentOnAssistantMessages":true,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolAdditions":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "k3-256k",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3-256K")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(262_144),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":false,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"maxTokensField":"max_tokens","thinkingFormat":"openai","requiresReasoningContentOnAssistantMessages":true,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolAdditions":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "k3.1-pro",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi K3.1 Pro")),
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
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":false,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"maxTokensField":"max_tokens","thinkingFormat":"openai","requiresReasoningContentOnAssistantMessages":true,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolAdditions":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "k3.1-pro-highspeed",
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
                r#"{"supportsStrictMode":false,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"maxTokensField":"max_tokens","thinkingFormat":"openai","requiresReasoningContentOnAssistantMessages":true,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolAdditions":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "kimi-for-coding",
        ModelInfo {
            name: Some(Cow::Borrowed("kimi-for-coding")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_048_576),
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
                r#"{"supportsStrictMode":false,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"maxTokensField":"max_tokens","thinkingFormat":"openai","requiresReasoningContentOnAssistantMessages":true,"supportsMidConvoSystemMessages":true,"supportsMidConvoToolAdditions":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "kimi-for-coding-highspeed",
        ModelInfo {
            name: Some(Cow::Borrowed("Kimi For Coding HighSpeed")),
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
            thinking_levels: Cow::Borrowed(&[(ThinkingLevel::Off, None)]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStrictMode":false,"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","thinkingFormat":"deepseek","supportsMidConvoSystemMessages":true}"#,
            )),
            ..BASE
        },
    ),
];
