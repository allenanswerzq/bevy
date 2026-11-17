//! The models of `MiniMax` CN, pi's provider `minimax-cn`.

use alloc::borrow::Cow;

use bevy_app::{App, Plugin};

use crate::{
    Cost, CostTier, ImageLimits, ImageResize, InputLimits, Modality, ModelInfo, Prices,
    ThinkingLevel,
};

/// Registers the 4 models pi lists for `minimax-cn`.
#[derive(Clone, Copy, Debug, Default)]
pub struct MiniMaxCnModelsPlugin;

impl Plugin for MiniMaxCnModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// What every model of this provider has in common.
const BASE: ModelInfo = ModelInfo {
    provider: Some(Cow::Borrowed("minimax-cn")),
    api: Some(Cow::Borrowed("anthropic-messages")),
    base_url: Some(Cow::Borrowed("https://api.minimaxi.com/anthropic")),
    ..super::UNKNOWN
};

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[
    (
        "MiniMax-M2.7",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax-M2.7")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(204_800),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3137,
                    output: 1.2549,
                    cache_read: 0.0627,
                    cache_write: 0.3922,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "MiniMax-M2.7-highspeed",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax-M2.7-highspeed")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text]),
            context_window: Some(204_800),
            max_output_tokens: Some(131_072),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.6275,
                    output: 2.5098,
                    cache_read: 0.0627,
                    cache_write: 0.3922,
                },
                tiers: Cow::Borrowed(&[]),
            }),
            ..BASE
        },
    ),
    (
        "MiniMax-M3",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax-M3")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(524_288),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.3137,
                    output: 1.2549,
                    cache_read: 0.0627,
                    cache_write: 0.0,
                },
                tiers: Cow::Borrowed(&[CostTier {
                    input_tokens_above: 512_000,
                    prices: Prices {
                        input: 0.6275,
                        output: 2.5098,
                        cache_read: 0.1255,
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
            ..BASE
        },
    ),
    (
        "MiniMax-M3.1-Flash",
        ModelInfo {
            name: Some(Cow::Borrowed("MiniMax-M3.1-Flash")),
            reasoning: Some(true),
            input: Cow::Borrowed(&[Modality::Text, Modality::Image]),
            context_window: Some(1_000_000),
            max_output_tokens: Some(524_288),
            cost: Some(Cost {
                prices: Prices {
                    input: 0.1494,
                    output: 0.5976,
                    cache_read: 0.0149,
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
            compat: Some(Cow::Borrowed(r#"{"forceAdaptiveThinking":true}"#)),
            ..BASE
        },
    ),
];
