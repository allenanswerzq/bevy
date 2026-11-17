//! Ask an LLM for a reply by adding a component to an entity.
//!
//! Spawn an entity with a [`SampleRequest`]. The reply streams in as
//! [`SampleDelta`] messages, and when it is done the entity gets a
//! [`SampleResponse`]. If it fails for good, the entity gets [`SampleFailed`]
//! instead.
//!
//! ```no_run
//! use bevy_app::{App, Startup, TaskPoolPlugin, Update};
//! use bevy_ecs::prelude::*;
//! use bevy_sampler::{Delta, Item, Request, SampleDelta, SampleRequest, SampleResponse, SamplerSettings, SamplerPlugin};
//!
//! fn ask(mut commands: Commands) {
//!     commands.spawn(SampleRequest(Request::new(vec![Item::user("Hi!")])));
//! }
//!
//! fn read(mut deltas: MessageReader<SampleDelta>, done: Query<&SampleResponse, Added<SampleResponse>>) {
//!     for SampleDelta { delta, .. } in deltas.read() {
//!         if let Delta::Text(text) = delta {
//!             // Show the text as it arrives.
//!         }
//!     }
//!     for SampleResponse(response) in &done {
//!         // Add response.items() to the conversation for the next turn.
//!     }
//! }
//!
//! App::new()
//!     .add_plugins((TaskPoolPlugin::default(), SamplerPlugin))
//!     .insert_resource(SamplerSettings {
//!         model: "grok-4".into(),
//!         api_key: std::env::var("XAI_API_KEY").ok(),
//!         ..Default::default()
//!     })
//!     .add_systems(Startup, ask)
//!     .add_systems(Update, read)
//!     .run();
//! ```
//!
//! # What happens to the entity
//!
//! 1. You add [`SampleRequest`].
//! 2. In `PostUpdate`, the request is sent on Bevy's IO task pool and the entity
//!    gets [`Sampling`]. At most [`SamplerSettings::max_in_flight`] requests run at
//!    once; the rest wait their turn.
//! 3. In every `PreUpdate` after that, new output is written as
//!    [`SampleDelta`] messages. When the request ends, one of three things happens:
//!    - It worked: the entity gets [`SampleResponse`], and [`SampleRequest`] is removed.
//!    - It can be retried: a [`SampleRetrying`] message is written and the entity
//!      gets [`RetryAt`]. Once that time passes, it goes back to step 2.
//!    - It cannot be retried: the entity gets [`SampleFailed`], and
//!      [`SampleRequest`] is removed.
//!
//! To cancel, remove [`SampleRequest`] or despawn the entity. To start over,
//! insert a new [`SampleRequest`]; this also clears the old response or error.
//!
//! Before each attempt, the sampler fixes conversation problems the provider
//! would reject. Reasoning from another model is dropped, and a tool call
//! without a result gets [`NO_RESULT`].
//!
//! The app needs `TaskPoolPlugin`, which `DefaultPlugins` and `MinimalPlugins`
//! already include.
//!
//! # Related crates
//!
//! The tool types come from `bevy_tool`. Fill [`Request::tools`] from
//! [`Tools::specs`](bevy_tool::Tools::specs) to offer the registered tools.
//! [`DefaultTools`](bevy_tool::DefaultTools) registers the built-in tools,
//! and [`DefaultModels`](bevy_model::DefaultModels) registers the known models.
//! If the app has [`ModelPlugin`](bevy_model::ModelPlugin), each response also
//! records the model limits it reports in [`Models`].

mod backend;
mod error;
mod http;
mod model;
mod retry;
mod settings;

pub use error::{HttpKind, SampleError};
pub use model::{
    CacheRetention, Delta, Item, LengthPolicy, Metrics, Part, Reasoning, Request, Response,
    StopReason, Usage, IMAGE_REMOVED, NO_RESULT,
};
pub use settings::{AuthScheme, Backend, ChatCompat, DoomLoopPolicy, RetryPolicy, SamplerSettings};

use core::hash::BuildHasher;
use std::{
    hash::RandomState,
    time::{Duration, Instant},
};

use async_compat::Compat;
use bevy_app::{App, Plugin, PostUpdate, PreStartup, PreUpdate};
use bevy_ecs::prelude::*;
use bevy_model::{ModelInfo, Models};
use bevy_tasks::{futures::check_ready, IoTaskPool, Task};
use reqwest::Client;

use crate::{
    http::{Attempt, Outcome},
    retry::{Decision, RetryState},
};

/// Adds the sampler. Configure it with the [`SamplerSettings`] resource.
#[derive(Clone, Copy, Debug, Default)]
pub struct SamplerPlugin;

impl Plugin for SamplerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SamplerSettings>()
            .init_resource::<SharedClient>()
            .add_message::<SampleDelta>()
            .add_message::<SampleRetrying>()
            .add_observer(restart_on_insert)
            .add_observer(clean_up_on_remove)
            .add_systems(PreStartup, prewarm)
            .add_systems(PreUpdate, receive_samples.in_set(SamplerSystems::Receive))
            .add_systems(
                PostUpdate,
                dispatch_samples.in_set(SamplerSystems::Dispatch),
            );
    }
}

/// Order your systems against the sampler.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SamplerSystems {
    /// `PreUpdate`: turn network results into messages and components.
    Receive,
    /// `PostUpdate`: start attempts for new and due requests.
    Dispatch,
}

/// Insert to start a sample. Inserting again restarts it; removing it cancels.
#[derive(Component, Clone, Debug, Default)]
pub struct SampleRequest(pub Request);

/// Settings for one entity, used instead of the [`SamplerSettings`] resource.
#[derive(Component, Clone, Debug)]
pub struct SamplerOverride(pub SamplerSettings);

/// Present while an HTTP attempt is running on the IO task pool. Dropping it
/// cancels the attempt.
#[derive(Component)]
pub struct Sampling {
    task: Task<Outcome>,
    deltas: async_channel::Receiver<Delta>,
}

impl Sampling {
    /// Run `attempt` on the IO task pool. reqwest needs a tokio reactor, which
    /// [`Compat`] runs on one background thread.
    fn spawn(attempt: Attempt) -> Self {
        let (sender, deltas) = async_channel::unbounded();
        let emit = move |delta| {
            let _ = sender.try_send(delta);
        };
        let task = IoTaskPool::get().spawn(Compat::new(http::run(attempt, emit)));
        Self { task, deltas }
    }
}

/// The client used when [`SamplerSettings::http`] is `None`. Clones share one
/// connection pool.
#[derive(Resource)]
struct SharedClient(Client);

impl Default for SharedClient {
    fn default() -> Self {
        Self(http::client())
    }
}

impl SharedClient {
    fn get(&self, settings: &SamplerSettings) -> Client {
        settings.http.clone().unwrap_or_else(|| self.0.clone())
    }
}

/// Present while a request waits to be retried.
#[derive(Component, Clone, Copy, Debug)]
pub struct RetryAt(pub Instant);

/// The finished sample.
#[derive(Component, Clone, Debug)]
pub struct SampleResponse(pub Response);

/// The sample failed and will not be retried.
#[derive(Component, Clone, Debug)]
pub struct SampleFailed(pub SampleError);

/// Streamed output from the attempt running on `entity`. A retry starts the
/// output over: when `attempt` goes up, drop what the earlier attempt sent.
#[derive(Message, Clone, Debug)]
pub struct SampleDelta {
    /// The entity with the [`SampleRequest`].
    pub entity: Entity,
    /// The attempt that produced it, starting at 1.
    pub attempt: u32,
    /// The new output.
    pub delta: Delta,
}

/// An attempt failed and will be retried after `delay`.
#[derive(Message, Clone, Debug)]
pub struct SampleRetrying {
    /// The entity with the [`SampleRequest`].
    pub entity: Entity,
    /// The attempt that failed, starting at 1.
    pub attempt: u32,
    /// Why it failed.
    pub error: SampleError,
    /// Wait before the next attempt.
    pub delay: Duration,
    /// Images removed from the request before retrying.
    pub stripped_images: usize,
}

/// Bookkeeping across the attempts of one request.
#[derive(Component, Clone)]
struct SampleState {
    retry: RetryState,
    attempts: u32,
    started: Instant,
}

fn restart_on_insert(insert: On<Insert<SampleRequest>>, mut commands: Commands) {
    commands
        .entity(insert.entity)
        .try_remove::<(Sampling, RetryAt, SampleState, SampleResponse, SampleFailed)>();
}

fn clean_up_on_remove(remove: On<Remove<SampleRequest>>, mut commands: Commands) {
    commands
        .entity(remove.entity)
        .try_remove::<(Sampling, RetryAt, SampleState)>();
}

/// `PreStartup`: connect early when [`SamplerSettings::prewarm`] is on.
fn prewarm(client: Res<SharedClient>, settings: Res<SamplerSettings>) {
    if settings.prewarm {
        let warm = http::prewarm(client.get(&settings), settings.base_url.clone());
        IoTaskPool::get().spawn(Compat::new(warm)).detach();
    }
}

/// `PostUpdate`: start attempts for requests that are new or due for a retry.
fn dispatch_samples(
    mut commands: Commands,
    client: Res<SharedClient>,
    settings: Res<SamplerSettings>,
    in_flight: Query<(), With<Sampling>>,
    pending: Query<
        (
            Entity,
            &SampleRequest,
            Option<&SamplerOverride>,
            Option<&SampleState>,
            Option<&RetryAt>,
        ),
        Without<Sampling>,
    >,
) {
    let now = Instant::now();
    let free = settings
        .max_in_flight
        .saturating_sub(in_flight.iter().count());
    let due = pending
        .iter()
        .filter(|(.., at)| at.is_none_or(|at| at.0 <= now));
    for (entity, request, custom, state, _) in due.take(free) {
        let settings = custom.map_or(&*settings, |c| &c.0);
        let mut state = state.cloned().unwrap_or(SampleState {
            retry: RetryState::default(),
            attempts: 0,
            started: now,
        });
        state.attempts += 1;
        let model = request.0.model.as_deref().unwrap_or(&settings.model);
        let mut request = request.0.clone();
        request.repair(model);
        let attempt = Attempt {
            client: client.get(settings),
            settings: settings.clone(),
            request,
            armed: settings.doom_loop.is_some() && !state.retry.disarmed,
        };
        commands
            .entity(entity)
            .insert((Sampling::spawn(attempt), state))
            .remove::<RetryAt>();
    }
}

/// `PreUpdate`: forward deltas and settle finished attempts.
fn receive_samples(
    mut commands: Commands,
    settings: Res<SamplerSettings>,
    mut models: Option<ResMut<Models>>,
    mut active: Query<(
        Entity,
        &mut Sampling,
        &mut SampleRequest,
        &mut SampleState,
        Option<&SamplerOverride>,
    )>,
    mut deltas: MessageWriter<SampleDelta>,
    mut retrying: MessageWriter<SampleRetrying>,
) {
    for (entity, mut sampling, mut request, mut state, custom) in &mut active {
        let sampling = sampling.bypass_change_detection();
        // Poll first: a finished task has already sent all of its deltas.
        let outcome = check_ready(&mut sampling.task);
        let streamed = core::iter::from_fn(|| sampling.deltas.try_recv().ok());
        let attempt = state.attempts;
        deltas.write_batch(streamed.map(|delta| SampleDelta {
            entity,
            attempt,
            delta,
        }));
        let Some(outcome) = outcome else {
            continue; // Still running.
        };

        let settings = custom.map_or(&*settings, |c| &c.0);
        let requested = request.0.model.as_deref().unwrap_or(&settings.model);
        let context_window = |response: &Response| {
            let known = |model: &str| models.as_ref()?.get(model)?.context_window;
            response
                .model_info
                .context_window
                .or_else(|| known(&response.model))
                .or_else(|| known(requested))
        };
        // Check the reply, which may be cut off by the length limit, then decide
        // whether an error is retried or final.
        let result = outcome
            .result
            .and_then(|response| {
                let window = context_window(&response);
                retry::verdict(response, request.0.length_policy, window)
            })
            .map_err(|error| {
                let decision = retry::decide(
                    settings,
                    &mut state.retry,
                    &mut request.0,
                    &error,
                    outcome.output_seen,
                    &outcome.partial,
                    jitter(),
                );
                (error, decision)
            });

        let mut entity_commands = commands.entity(entity);
        match result {
            Ok(mut response) => {
                response.metrics = Metrics {
                    attempts: state.attempts,
                    time_to_first_token: outcome.time_to_first_token,
                    duration: state.started.elapsed(),
                };
                if let Some(models) = &mut models {
                    learn(models, &response);
                }
                entity_commands
                    .insert(SampleResponse(response))
                    .remove::<SampleRequest>();
            }
            Err((error, Decision::Retry { delay, stripped })) => {
                retrying.write(SampleRetrying {
                    entity,
                    attempt: state.attempts,
                    error,
                    delay,
                    stripped_images: stripped,
                });
                entity_commands
                    .insert(RetryAt(Instant::now() + delay))
                    .remove::<Sampling>();
            }
            Err((error, Decision::Fail)) => {
                entity_commands
                    .insert(SampleFailed(error))
                    .remove::<SampleRequest>();
            }
        }
    }
}

/// Record the model limits a response reported. A limit the response leaves
/// out keeps its registered value. [`Models`] is only marked changed when
/// something is new.
fn learn(models: &mut ResMut<Models>, response: &Response) {
    let known = models.get(&response.model).cloned().unwrap_or_default();
    let reported = &response.model_info;
    let info = ModelInfo {
        context_window: reported.context_window.or(known.context_window),
        max_output_tokens: reported.max_output_tokens.or(known.max_output_tokens),
        ..known.clone()
    };
    if info != known {
        models.insert(response.model.clone(), info);
    }
    if response.models_etag.is_some() && models.etag != response.models_etag {
        models.etag.clone_from(&response.models_etag);
    }
}

/// A random number in `0..1`.
fn jitter() -> f64 {
    (RandomState::new().hash_one(0u8) >> 11) as f64 / (1u64 << 53) as f64
}
