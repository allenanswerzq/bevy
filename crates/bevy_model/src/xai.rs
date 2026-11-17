use bevy_app::{App, Plugin};

use crate::{ModelApp, ModelInfo};

/// Registers xAI's Grok models.
///
/// The limits come from grok-build's `default_models.json`.
#[derive(Clone, Copy, Debug, Default)]
pub struct XaiModelsPlugin;

impl Plugin for XaiModelsPlugin {
    fn build(&self, app: &mut App) {
        let grok = ModelInfo {
            context_window: Some(500_000),
            max_output_tokens: None,
        };
        app.register_model("grok-4.6", grok.clone())
            .register_model("grok-4.5", grok);
    }
}
