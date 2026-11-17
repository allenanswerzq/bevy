//! Stream one reply from an LLM endpoint.
//!
//! ```sh
//! SAMPLER_BASE_URL=https://api.x.ai/v1 SAMPLER_MODEL=<model> SAMPLER_API_KEY=<key> \
//! SAMPLER_BACKEND=responses cargo run --example chat -- "Why is the sky blue?"
//! ```
//!
//! `SAMPLER_BACKEND` is `chat` (default), `responses`, or `messages`.
#![expect(clippy::print_stdout, reason = "the example prints the reply")]

use std::{env, io::Write, time::Duration};

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_sampler::{
    Backend, Delta, Item, Request, SampleDelta, SampleFailed, SampleRequest, SampleResponse,
    SampleRetrying, SamplerPlugin, SamplerSettings,
};

fn main() -> AppExit {
    let backend = match env::var("SAMPLER_BACKEND").as_deref() {
        Ok("responses") => Backend::Responses,
        Ok("messages") => Backend::Messages,
        _ => Backend::ChatCompletions,
    };
    let defaults = SamplerSettings::default();
    let settings = SamplerSettings {
        backend,
        base_url: env::var("SAMPLER_BASE_URL").unwrap_or(defaults.base_url.clone()),
        model: env::var("SAMPLER_MODEL").expect("set SAMPLER_MODEL"),
        api_key: env::var("SAMPLER_API_KEY").ok(),
        ..defaults
    };
    let prompt = env::args()
        .nth(1)
        .unwrap_or_else(|| "Say hi in five words.".into());

    App::new()
        .add_plugins((
            TaskPoolPlugin::default(),
            ScheduleRunnerPlugin::run_loop(Duration::from_millis(5)),
            SamplerPlugin,
        ))
        .insert_resource(settings)
        .add_systems(Startup, move |mut commands: Commands| {
            commands.spawn(SampleRequest(Request::new(vec![Item::user(
                prompt.clone(),
            )])));
        })
        .add_systems(Update, (print_progress, exit_when_done))
        .run()
}

fn print_progress(
    mut deltas: MessageReader<SampleDelta>,
    mut retries: MessageReader<SampleRetrying>,
) {
    for SampleDelta { delta, .. } in deltas.read() {
        match delta {
            Delta::Text(text) => print!("{text}"),
            Delta::ToolCall {
                name: Some(name), ..
            } => print!("\n[tool call: {name}]"),
            _ => {}
        }
    }
    for retry in retries.read() {
        println!("\n[retrying in {:?}: {}]", retry.delay, retry.error);
    }
    std::io::stdout().flush().unwrap();
}

fn exit_when_done(
    done: Query<&SampleResponse, Added<SampleResponse>>,
    failed: Query<&SampleFailed, Added<SampleFailed>>,
    mut exit: MessageWriter<AppExit>,
) {
    for SampleResponse(response) in &done {
        println!("\n\n{:?}, {:?}", response.usage, response.metrics);
        exit.write(AppExit::Success);
    }
    for SampleFailed(error) in &failed {
        println!("\nfailed: {error}");
        exit.write(AppExit::error());
    }
}
