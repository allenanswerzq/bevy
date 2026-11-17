//! The models of Z.AI, pi's provider `zai`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices, ThinkingLevel,
};

/// Registers the 7 models pi lists for `zai`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ZaiModelsPlugin;

impl Plugin for ZaiModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("zai")),
    api: Some(Cow::Borrowed("openai-completions")),
    base_url: Some(Cow::Borrowed("https://api.z.ai/api/coding/paas/v4")),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "glm-4.7",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-4.7")),
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","thinkingFormat":"zai","supportsStrictMode":true,"zaiToolStream":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "glm-5-turbo",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5-Turbo")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(200_000),
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","thinkingFormat":"zai","supportsStrictMode":true,"zaiToolStream":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "glm-5.2",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.2")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
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
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"maxTokensField":"max_tokens","thinkingFormat":"zai","supportsStrictMode":true,"zaiToolStream":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "glm-5.2-highspeed",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.2 Highspeed")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
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
            thinking_levels: Cow::Borrowed(&[
                (ThinkingLevel::Off, Some(Cow::Borrowed("none"))),
                (ThinkingLevel::Minimal, None),
                (ThinkingLevel::Low, None),
                (ThinkingLevel::Medium, None),
                (ThinkingLevel::High, Some(Cow::Borrowed("high"))),
                (ThinkingLevel::Xhigh, None),
                (ThinkingLevel::Max, Some(Cow::Borrowed("max"))),
            ]),
            compat: Some(Cow::Borrowed(
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"maxTokensField":"max_tokens","thinkingFormat":"zai","supportsStrictMode":true,"zaiToolStream":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "glm-5.3",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.3")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(1_000_000),
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"maxTokensField":"max_tokens","thinkingFormat":"zai","supportsStrictMode":true,"zaiToolStream":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "glm-5.3-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.3-Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"maxTokensField":"max_tokens","thinkingFormat":"zai","supportsStrictMode":true,"zaiToolStream":true}"#,
            )),
            ..BASE
        },
    ),
    (
        "glm-5.3-highspeed",
        ModelInfo {
            name: Some(Cow::Borrowed("GLM-5.3 Highspeed")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
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
                r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":true,"maxTokensField":"max_tokens","thinkingFormat":"zai","supportsStrictMode":true,"zaiToolStream":true}"#,
            )),
            ..BASE
        },
    ),
];
