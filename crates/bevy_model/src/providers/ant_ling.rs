//! The models of Ant Ling, pi's provider `ant-ling`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{Cost, Modality, ModelInfo, Prices, ThinkingLevel};

/// Registers the 3 models pi lists for `ant-ling`.
#[derive(Clone, Copy, Debug, Default)]
pub struct AntLingModelsPlugin;

impl Plugin for AntLingModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("ant-ling")),
    api: Some(Cow::Borrowed("openai-completions")),
    base_url: Some(Cow::Borrowed("https://api.ant-ling.com/v1")),
    compat: Some(Cow::Borrowed(
        r#"{"supportsStore":false,"supportsDeveloperRole":false,"supportsReasoningEffort":false,"maxTokensField":"max_tokens","thinkingFormat":"ant-ling","supportsStrictMode":true,"supportsLongCacheRetention":false}"#,
    )),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "Ling-2.6-1T",
        ModelInfo {
            name: Some(Cow::Borrowed("Ling 2.6 1T")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.25,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "Ling-2.6-flash",
        ModelInfo {
            name: Some(Cow::Borrowed("Ling 2.6 Flash")),
            reasoning: Some(false),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.01,
                    output: 0.02,
                    cache_read: 0.0,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "Ring-2.6-1T",
        ModelInfo {
            name: Some(Cow::Borrowed("Ring 2.6 1T")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(262_144),
            max_output_tokens: Some(65_536),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.06,
                    output: 0.25,
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
                (ThinkingLevel::Xhigh, Some(Cow::Borrowed("xhigh"))),
            ]),
            ..BASE
        },
    ),
];
