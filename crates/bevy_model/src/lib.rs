//! The models an app can use, registered as Bevy data.
//!
//! [`ModelPlugin`] adds the [`Models`] resource, which maps each model id to
//! what is known about it, such as its context window. Register models up
//! front with [`ModelApp::register_model`]. A sampler can also fill in limits
//! as responses report them, and other systems read them, for example to decide
//! when a conversation needs to be shortened.
//!
//! [`DefaultModels`] adds the registry together with the models this crate
//! knows about. Each provider is its own plugin, so you can leave one out:
//!
//! ```
//! use bevy_app::{App, PluginGroup};
//! use bevy_model::{DefaultModels, Models, XaiModelsPlugin};
//!
//! let mut app = App::new();
//! app.add_plugins(DefaultModels);
//! assert!(app.world().resource::<Models>().get("grok-4.6").is_some());
//!
//! // The same, without the xAI models.
//! App::new().add_plugins(DefaultModels.build().disable::<XaiModelsPlugin>());
//! ```
//!
//! Register your own models with [`ModelApp::register_model`]:
//!
//! ```
//! use bevy_app::App;
//! use bevy_model::{ModelApp, ModelInfo, ModelPlugin, Models};
//!
//! let mut app = App::new();
//! app.add_plugins(ModelPlugin).register_model(
//!     "my-model",
//!     ModelInfo {
//!         context_window: Some(128_000),
//!         max_output_tokens: Some(16_000),
//!     },
//! );
//! let models = app.world().resource::<Models>();
//! assert_eq!(models.get("my-model").unwrap().context_window, Some(128_000));
//! ```

mod xai;

pub use xai::XaiModelsPlugin;

use bevy_app::{plugin_group, App, Plugin};
use bevy_ecs::prelude::*;
use bevy_platform::collections::HashMap;

plugin_group! {
    /// The model registry and every model this crate knows about:
    #[derive(Default)]
    pub struct DefaultModels {
        :ModelPlugin,
        :XaiModelsPlugin,
    }
}

/// Adds the [`Models`] resource.
#[derive(Clone, Copy, Debug, Default)]
pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Models>();
    }
}

/// What is known about each model, by model id.
#[derive(Resource, Clone, Debug, Default)]
pub struct Models {
    models: HashMap<String, ModelInfo>,
    /// Version of the server's model list. When it changes, the list has new
    /// or updated models.
    pub etag: Option<String>,
}

impl Models {
    /// What is known about this model.
    pub fn get(&self, id: &str) -> Option<&ModelInfo> {
        self.models.get(id)
    }

    /// Record what is known about a model, replacing what was there.
    pub fn insert(&mut self, id: impl Into<String>, info: ModelInfo) {
        self.models.insert(id.into(), info);
    }

    /// Every known model.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &ModelInfo)> {
        self.models.iter().map(|(id, info)| (id.as_str(), info))
    }
}

/// Limits of one model.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModelInfo {
    /// Context window in tokens.
    pub context_window: Option<u64>,
    /// Largest allowed output in tokens.
    pub max_output_tokens: Option<u64>,
}

/// Registers models on an [`App`].
pub trait ModelApp {
    /// Register a model. Registering the same id again replaces it.
    fn register_model(&mut self, id: impl Into<String>, info: ModelInfo) -> &mut Self;
}

impl ModelApp for App {
    fn register_model(&mut self, id: impl Into<String>, info: ModelInfo) -> &mut Self {
        self.world_mut()
            .get_resource_or_init::<Models>()
            .insert(id, info);
        self
    }
}
