//! Agents as Bevy entities: a model that calls tools in a loop until it has
//! an answer.
//!
//! An agent is an entity with an [`Agent`]. Write a [`Prompt`] for it, and it
//! works in turns until the model answers without calling a tool:
//!
//! ```text
//! prompt -> model -> tools -> model -> tools -> ... -> answer -> next prompt
//! ```
//!
//! ```no_run
//! use bevy_agent::{Agent, AgentPlugins, Prompt, Transcript};
//! use bevy_app::{App, ScheduleRunnerPlugin, Startup, TaskPoolPlugin};
//! use bevy_ecs::{prelude::*, query::QueryData};
//! use bevy_sampler::{Item, SamplerSettings};
//!
//! fn spawn(mut commands: Commands, mut prompts: MessageWriter<Prompt>) {
//!     let agent = commands
//!         .spawn((Agent::default(), Transcript(vec![Item::system("You are a coding agent.")])))
//!         .id();
//!     prompts.write(Prompt::new(agent, "Why does the test in src/lib.rs fail?"));
//! }
//!
//! App::new()
//!     .add_plugins((TaskPoolPlugin::default(), ScheduleRunnerPlugin::default(), AgentPlugins))
//!     .insert_resource(SamplerSettings {
//!         model: "grok-4.6".into(),
//!         api_key: std::env::var("XAI_API_KEY").ok(),
//!         ..Default::default()
//!     })
//!     .add_systems(Startup, spawn)
//!     .run();
//! ```
//!
//! # How an agent works
//!
//! An agent is always in one of three phases. Each phase is a marker component:
//!
//! | Phase              | Waiting for                     | Then                                       |
//! |--------------------|---------------------------------|--------------------------------------------|
//! | [`Idle`]           | a prompt in its [`Inbox`]       | [`Thinking`]                               |
//! | [`Thinking`]       | the model's reply               | [`RunningTools`], or [`Idle`] if no calls    |
//! | [`RunningTools`]   | the output of every tool call   | [`Thinking`]                               |
//!
//! 1. A turn starts. The inbox moves into the [`Transcript`], and the agent
//!    gets a [`SampleRequest`] with the transcript and every registered tool.
//!    `bevy_sampler` sends it.
//! 2. The reply arrives as a [`SampleResponse`], and its items join the
//!    transcript. Each tool call becomes a child entity with a [`ToolCall`],
//!    which `bevy_tool` runs.
//! 3. When every call has a [`ToolOutput`], the outputs join the transcript,
//!    the call entities are despawned, and the next turn starts.
//!
//! Prompts that arrive while the agent works wait in its inbox and join the
//! transcript when the next turn starts, so a user can steer a busy agent. If
//! the model fails for good, the agent keeps the [`SampleFailed`] and goes back
//! to [`Idle`]. Despawning the agent cancels its request and its tool calls.
//!
//! The reply streams in as [`SampleDelta`](bevy_sampler::SampleDelta) messages
//! whose entity is the agent.

use bevy_app::{plugin_group, App, Plugin, Update};
use bevy_ecs::{prelude::*, query::QueryData};
use bevy_sampler::{Item, Request, SampleFailed, SampleRequest, SampleResponse};
use bevy_tool::{ToolCall, ToolOutput, Tools};

plugin_group! {
    /// Everything an agent needs:
    pub struct AgentPlugins {
        bevy_sampler:::SamplerPlugin,
        :AgentPlugin,
        #[plugin_group]
        bevy_tool:::DefaultTools,
        #[plugin_group]
        bevy_model:::DefaultModels,
    }
    /// The app also needs `TaskPoolPlugin`, which `DefaultPlugins` and
    /// `MinimalPlugins` include.
}

/// Moves every [`Agent`] through its turns. Needs `SamplerPlugin` and
/// `ToolPlugin`; [`AgentPlugins`] adds both.
#[derive(Clone, Copy, Debug, Default)]
pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Prompt>().add_systems(
            Update,
            (
                queue_prompts,
                read_failures,
                read_replies,
                finish_tool_rounds,
                start_turns,
            )
                .chain()
                .in_set(AgentSystems),
        );
    }
}

/// `Update`: the systems that move agents through their turns.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AgentSystems;

/// A model that works in turns, calling tools until it has an answer.
#[derive(Component, Clone, Debug, Default)]
#[require(Transcript, Inbox, Idle)]
pub struct Agent {
    /// Settings for every request, such as the model or the reasoning effort.
    /// Each turn, its items are replaced by the transcript, and its tools by
    /// the registered tools.
    pub request: Request,
}

/// The agent's conversation so far. Start it with an [`Item::System`] to give
/// the agent its instructions.
#[derive(Component, Clone, Debug, Default)]
pub struct Transcript(pub Vec<Item>);

/// Items that join the transcript when the next turn starts.
#[derive(Component, Clone, Debug, Default)]
pub struct Inbox(pub Vec<Item>);

/// Phase: waiting for a prompt.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Idle;

/// Phase: waiting for the model's reply.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Thinking;

/// Phase: waiting for the tool calls, which are the agent's children.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RunningTools;

/// A user message for an agent.
#[derive(Message, Clone, Debug)]
pub struct Prompt {
    /// The agent.
    pub agent: Entity,
    /// The message, usually an [`Item::User`].
    pub item: Item,
}

impl Prompt {
    /// A text message for `agent`.
    pub fn new(agent: Entity, text: impl Into<String>) -> Self {
        Self {
            agent,
            item: Item::user(text),
        }
    }
}

/// Put each prompt in its agent's inbox.
fn queue_prompts(mut prompts: MessageReader<Prompt>, mut inboxes: Query<&mut Inbox>) {
    for prompt in prompts.read() {
        if let Ok(mut inbox) = inboxes.get_mut(prompt.agent) {
            inbox.0.push(prompt.item.clone());
        }
    }
}

/// `Thinking` -> `Idle`: the model failed for good.
fn read_failures(
    mut commands: Commands,
    agents: Query<Entity, (With<Thinking>, With<SampleFailed>)>,
) {
    for agent in &agents {
        commands.entity(agent).remove::<Thinking>().insert(Idle);
    }
}

/// The parts of an agent that its turns read and change.
#[derive(QueryData)]
#[query_data(mutable)]
struct AgentQuery {
    entity: Entity,
    config: &'static Agent,
    transcript: &'static mut Transcript,
    inbox: &'static mut Inbox,
}

/// `Thinking` -> `RunningTools`, or `Idle` when the reply has no tool calls.
fn read_replies(
    mut commands: Commands,
    tools: Res<Tools>,
    mut agents: Query<(AgentQuery, &SampleResponse), With<Thinking>>,
) {
    for (mut agent, SampleResponse(response)) in &mut agents {
        agent.transcript.0.extend(response.items());
        if !response.tool_calls.is_empty() {
            for call in &response.tool_calls {
                commands.spawn((call.clone(), ChildOf(agent.entity)));
            }
            commands
                .entity(agent.entity)
                .remove::<Thinking>()
                .insert(RunningTools);
        } else if !agent.inbox.0.is_empty() {
            // The user wrote while the model was answering.
            start_turn(&mut agent, &mut commands, &tools);
        } else {
            commands
                .entity(agent.entity)
                .remove::<Thinking>()
                .insert(Idle);
        }
    }
}

/// `RunningTools` -> `Thinking`: once every tool call has its output, add the
/// outputs to the transcript and start the next turn.
fn finish_tool_rounds(
    mut commands: Commands,
    tools: Res<Tools>,
    mut agents: Query<(AgentQuery, &Children), With<RunningTools>>,
    calls: Query<(&ToolCall, Option<&ToolOutput>)>,
) {
    for (mut agent, children) in &mut agents {
        // `None` while any call is still running.
        let finished: Option<Vec<_>> = children
            .iter()
            .filter_map(|child| Some((child, calls.get(child).ok()?)))
            .map(|(child, (call, output))| Some((child, call, output?)))
            .collect();
        if let Some(finished) = finished {
            for (child, call, output) in finished {
                agent.transcript.0.push(Item::ToolResult {
                    call_id: call.id.clone(),
                    content: output.content.clone(),
                    images: output.images.clone(),
                });
                commands.entity(child).despawn();
            }
            start_turn(&mut agent, &mut commands, &tools);
        }
    }
}

/// `Idle` -> `Thinking`: start a turn for each idle agent with a prompt waiting.
fn start_turns(
    mut commands: Commands,
    tools: Res<Tools>,
    mut agents: Query<AgentQuery, With<Idle>>,
) {
    for mut agent in &mut agents {
        if !agent.inbox.0.is_empty() {
            start_turn(&mut agent, &mut commands, &tools);
        }
    }
}

/// Move the inbox into the transcript, and ask the model for the next turn.
fn start_turn(agent: &mut AgentQueryItem<'_, '_>, commands: &mut Commands, tools: &Tools) {
    agent.transcript.0.append(&mut agent.inbox.0);
    let request = Request {
        items: agent.transcript.0.clone(),
        tools: tools.specs().to_vec(),
        ..agent.config.request.clone()
    };
    commands
        .entity(agent.entity)
        .remove::<(Idle, RunningTools)>()
        .insert((SampleRequest(request), Thinking));
}
