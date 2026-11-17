//! Tools an LLM can call, defined and run as Bevy systems.
//!
//! A tool is a [`ToolSpec`], which tells the model what the tool does, plus a
//! system that runs it. Register both with [`ToolApp::add_tool`]. The system
//! takes the call as `In<ToolCall>` and returns a [`ToolOutput`] for quick
//! work, or a [`Task<ToolOutput>`] for slow work such as running a command.
//!
//! ```no_run
//! use bevy_app::{App, Update};
//! use bevy_ecs::prelude::*;
//! use bevy_tool::{ToolApp, ToolCall, ToolOutput, ToolPlugin, ToolSpec};
//! use serde_json::json;
//!
//! fn echo(In(call): In<ToolCall>) -> ToolOutput {
//!     ToolOutput::text(call.arguments)
//! }
//!
//! fn print(outputs: Query<&ToolOutput, Added<ToolOutput>>) {
//!     for output in &outputs {
//!         println!("{}", output.content);
//!     }
//! }
//!
//! App::new()
//!     .add_plugins(ToolPlugin)
//!     .add_tool(
//!         ToolSpec::new("echo", "Repeat the arguments back.", json!({"type": "object"})),
//!         echo,
//!     )
//!     .add_systems(Update, print)
//!     .run();
//! ```
//!
//! # Running a call
//!
//! 1. Spawn an entity with a [`ToolCall`], usually as a child of the session
//!    that asked for it.
//! 2. In `PostUpdate`, the tool's system runs. A [`ToolOutput`] is inserted
//!    right away, or the entity gets [`ToolRunning`] while its task runs.
//! 3. In `PreUpdate`, finished tasks give their entity a [`ToolOutput`].
//!
//! A call to a tool that is not registered gets an error [`ToolOutput`], so the
//! model learns about its mistake. To cancel, remove [`ToolCall`] or despawn the
//! entity. To run a call again, insert a new [`ToolCall`].
//!
//! # Built-in tools
//!
//! [`DefaultTools`] adds [`ToolPlugin`] and every tool in [`tools`], such as
//! `bash` and `read_file`. Each tool is its own plugin, so you can leave one out:
//!
//! ```no_run
//! use bevy_app::{App, PluginGroup, TaskPoolPlugin};
//! use bevy_tool::{tools::BashPlugin, DefaultTools};
//!
//! App::new().add_plugins((
//!     TaskPoolPlugin::default(),
//!     DefaultTools.build().disable::<BashPlugin>(),
//! ));
//! ```
//!
//! The built-in tools run on Bevy's IO task pool, so the app needs
//! `TaskPoolPlugin`, which `DefaultPlugins` and `MinimalPlugins` already include.

pub mod tools;

use bevy_app::{plugin_group, App, Plugin, PostUpdate, PreUpdate};
use bevy_ecs::{prelude::*, system::SystemId};
use bevy_platform::collections::HashMap;
use bevy_tasks::{futures::check_ready, Task};
use serde_json::Value;

plugin_group! {
    /// The tool runner and every built-in tool:
    #[derive(Default)]
    pub struct DefaultTools {
        :ToolPlugin,
        tools:::BashPlugin,
        tools:::ReadFilePlugin,
        tools:::SearchReplacePlugin,
        tools:::ListDirPlugin,
        tools:::GrepPlugin,
    }
}

/// Adds the tool registry and runs [`ToolCall`] entities.
#[derive(Clone, Copy, Debug, Default)]
pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tools>()
            .add_observer(restart_on_insert)
            .add_observer(cancel_on_remove)
            .add_systems(PreUpdate, finish_tools.in_set(ToolSystems::Finish))
            .add_systems(PostUpdate, start_tools.in_set(ToolSystems::Start));
    }
}

/// Order your systems against the tool runner.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolSystems {
    /// `PreUpdate`: give finished calls their [`ToolOutput`].
    Finish,
    /// `PostUpdate`: run the systems of new calls.
    Start,
}

/// What the model is told about a tool.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolSpec {
    /// Tool name.
    pub name: String,
    /// What the tool does, for the model.
    pub description: String,
    /// JSON schema of the arguments.
    pub parameters: Value,
}

impl ToolSpec {
    /// A tool with this name, description, and argument schema.
    pub fn new(name: impl Into<String>, description: impl Into<String>, parameters: Value) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters,
        }
    }
}

/// A call the model made. Spawn it on an entity to run the tool.
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct ToolCall {
    /// Id that the tool result must refer to.
    pub id: String,
    /// Tool name.
    pub name: String,
    /// Arguments as a JSON string.
    pub arguments: String,
}

/// Whether and which tools the model must call.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum ToolChoice {
    /// The model decides.
    #[default]
    Auto,
    /// No tool calls.
    None,
    /// At least one tool call.
    Required,
    /// Call this tool.
    Function(String),
}

/// The result of a finished [`ToolCall`].
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct ToolOutput {
    /// Text output.
    pub content: String,
    /// Image URLs or `data:` URLs.
    pub images: Vec<String>,
}

impl ToolOutput {
    /// Text output with no images.
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            images: Vec::new(),
        }
    }
}

/// Present while a call's task runs. Dropping it cancels the task.
#[derive(Component)]
pub struct ToolRunning(Task<ToolOutput>);

/// What a tool system returns. Tool systems may also return a [`ToolOutput`] or
/// a [`Task<ToolOutput>`] directly.
pub enum ToolRun {
    /// The output is ready.
    Done(ToolOutput),
    /// The output arrives when the task finishes.
    Running(Task<ToolOutput>),
}

impl From<ToolOutput> for ToolRun {
    fn from(output: ToolOutput) -> Self {
        Self::Done(output)
    }
}

impl From<Task<ToolOutput>> for ToolRun {
    fn from(task: Task<ToolOutput>) -> Self {
        Self::Running(task)
    }
}

/// The registered tools.
#[derive(Resource, Default)]
pub struct Tools {
    specs: Vec<ToolSpec>,
    systems: HashMap<String, SystemId<In<ToolCall>, ToolRun>>,
}

impl Tools {
    /// Every tool's spec, in the order they were added. Send these with a
    /// request; a stable order keeps the prompt cache warm.
    pub fn specs(&self) -> &[ToolSpec] {
        &self.specs
    }

    /// The spec of the tool with this name.
    pub fn get(&self, name: &str) -> Option<&ToolSpec> {
        self.specs.iter().find(|spec| spec.name == name)
    }
}

/// Adds tools to an [`App`].
pub trait ToolApp {
    /// Register a tool. Adding a tool with the same name replaces it.
    fn add_tool<O, M>(
        &mut self,
        spec: ToolSpec,
        system: impl IntoSystem<In<ToolCall>, O, M> + 'static,
    ) -> &mut Self
    where
        O: Into<ToolRun> + 'static;
}

impl ToolApp for App {
    fn add_tool<O, M>(
        &mut self,
        spec: ToolSpec,
        system: impl IntoSystem<In<ToolCall>, O, M> + 'static,
    ) -> &mut Self
    where
        O: Into<ToolRun> + 'static,
    {
        let world = self.world_mut();
        let id = world.register_system(system.map(Into::into));
        let mut tools = world.get_resource_or_init::<Tools>();
        let old = tools.systems.insert(spec.name.clone(), id);
        match tools.specs.iter_mut().find(|s| s.name == spec.name) {
            Some(existing) => *existing = spec,
            None => tools.specs.push(spec),
        }
        if let Some(old) = old {
            let _ = world.unregister_system(old);
        }
        self
    }
}

fn restart_on_insert(insert: On<Insert<ToolCall>>, mut commands: Commands) {
    commands
        .entity(insert.entity)
        .try_remove::<(ToolRunning, ToolOutput)>();
}

fn cancel_on_remove(remove: On<Remove<ToolCall>>, mut commands: Commands) {
    commands.entity(remove.entity).try_remove::<ToolRunning>();
}

/// `PostUpdate`: run the system of every new call.
fn start_tools(
    mut commands: Commands,
    tools: Res<Tools>,
    new: Query<(Entity, &ToolCall), (Without<ToolRunning>, Without<ToolOutput>)>,
) {
    for (entity, call) in &new {
        let system = tools.systems.get(&call.name).copied();
        let call = call.clone();
        // Tool systems need the whole World, so they run when commands apply.
        commands.queue(move |world: &mut World| {
            let run = match system {
                Some(system) => world
                    .run_system_with(system, call)
                    .unwrap_or_else(|error| ToolOutput::text(format!("error: {error}")).into()),
                None => ToolOutput::text(format!("error: unknown tool `{}`", call.name)).into(),
            };
            // The tool may have despawned the entity.
            if let Ok(mut entity) = world.get_entity_mut(entity) {
                match run {
                    ToolRun::Done(output) => entity.insert(output),
                    ToolRun::Running(task) => entity.insert(ToolRunning(task)),
                };
            }
        });
    }
}

/// `PreUpdate`: give finished calls their output.
fn finish_tools(mut commands: Commands, mut running: Query<(Entity, &mut ToolRunning)>) {
    for (entity, mut running) in &mut running {
        if let Some(output) = check_ready(&mut running.bypass_change_detection().0) {
            commands
                .entity(entity)
                .insert(output)
                .remove::<ToolRunning>();
        }
    }
}
