//! The models of `TypeSafe`, pi's provider `typesafe`.

use bevy_app::{App, Plugin};

use crate::ModelInfo;

/// Registers the 0 models pi lists for `typesafe`.
#[derive(Clone, Copy, Debug, Default)]
pub struct TypeSafeModelsPlugin;

impl Plugin for TypeSafeModelsPlugin {
    fn build(&self, app: &mut App) {
        super::register(app, MODELS);
    }
}

/// Each model's id and what is known about it.
const MODELS: &[(&str, ModelInfo)] = &[];
