//! A coding agent in your terminal, built with `bevy_agent`.
//!
//! The agent can read, search, and edit files, and run commands, in the current
//! directory. Run it in a folder you are happy for it to change.
//!
//! ```sh
//! SAMPLER_BASE_URL=https://api.x.ai/v1 SAMPLER_MODEL=grok-4.6 SAMPLER_API_KEY=<key> \
//! SAMPLER_BACKEND=responses cargo run --example coding_agent -- "What does this project do?"
//! ```
//!
//! `SAMPLER_BACKEND` is `chat` (default), `responses`, or `messages`.
#![expect(clippy::print_stdout, reason = "the example prints the agent's work")]

use std::{env, io::Write, time::Duration};

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_agent::{Agent, AgentPlugins, AgentSystems, Idle, Prompt, Transcript};
use bevy_sampler::{
    Backend, Delta, Item, SampleDelta, SampleFailed, SampleRetrying, SamplerSettings,
};
use bevy_tool::{ToolCall, ToolOutput};

const INSTRUCTIONS: &str = "You are a coding agent working in the current directory. \
    Look at the files with your tools before you answer. Keep answers short.";

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
        .unwrap_or_else(|| "What does this project do? Answer in three sentences.".into());

    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(5))),
            AgentPlugins,
        ))
        .insert_resource(settings)
        .add_systems(
            Startup,
            move |mut commands: Commands, mut prompts: MessageWriter<Prompt>| {
                let agent = commands
                    .spawn((
                        Agent::default(),
                        Transcript(vec![Item::system(INSTRUCTIONS)]),
                    ))
                    .id();
                prompts.write(Prompt::new(agent, prompt.clone()));
            },
        )
        .add_systems(
            Update,
            (print_progress, exit_when_done).before(AgentSystems),
        )
        .run()
}

/// Print the reply as it streams in, and each tool call when it finishes.
fn print_progress(
    mut deltas: MessageReader<SampleDelta>,
    mut retries: MessageReader<SampleRetrying>,
    finished: Query<(&ToolCall, &ToolOutput), Added<ToolOutput>>,
) {
    for SampleDelta { delta, .. } in deltas.read() {
        if let Delta::Text(text) = delta {
            print!("{text}");
        }
    }
    for retry in retries.read() {
        println!("\n[retrying in {:?}: {}]", retry.delay, retry.error);
    }
    for (call, output) in &finished {
        let first_line = output.content.lines().next().unwrap_or_default();
        let lines = output.content.lines().count();
        println!(
            "\n> {}({})\n  {first_line} ({lines} lines)\n",
            call.name, call.arguments
        );
    }
    std::io::stdout().flush().unwrap();
}

/// Quit when the agent has answered, or has failed.
fn exit_when_done(
    agents: Query<(&Transcript, Option<&SampleFailed>), Added<Idle>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (transcript, failed) in &agents {
        if let Some(SampleFailed(error)) = failed {
            println!("\nfailed: {error}");
            exit.write(AppExit::error());
        } else if matches!(transcript.0.last(), Some(Item::Assistant { .. })) {
            println!();
            exit.write(AppExit::Success);
        }
    }
}
