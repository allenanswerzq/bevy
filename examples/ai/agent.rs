//! A tiny coding agent built with ECS.
//!
//! An agent loop looks like this:
//!
//! ```text
//! user prompt -> turn 1 -> turn 2 -> ... -> answer -> next user prompt -> ...
//! ```
//!
//! In each turn the model replies with text, and maybe some tool calls. If there are tool calls,
//! we run them, add their results to the transcript, and ask the model again. When the model
//! replies with text only, the chain of turns is done and the session waits for the user.
//!
//! The ECS design:
//!
//! - Each conversation is a **session entity**. Its current phase is a marker component:
//!   `Idle`, `NeedsSample`, `Sampling`, or `WaitingTools`. A session has exactly one of them.
//! - Each tool call is its own **entity**, a child of its session through `ChildOf`.
//! - Slow work, such as model requests and tools, runs on background threads. Systems never wait
//!   for it. Results come back through a channel, and `drain_io` turns them into messages.
//! - A chain of small systems in `Update` moves each session from one phase to the next.
//!
//! ```text
//! Idle --prompt--> NeedsSample --request sent--> Sampling --text only--> Idle
//!                       ^                           |
//!                       |                      tool calls
//!                       |                           v
//!                       +----all results ready-- WaitingTools
//! ```
//!
//! The model and the tools are fake, so the example runs offline. Two users talk to the agent at
//! the same time to show that one set of systems drives every session.

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Mutex,
    },
    thread,
    time::Duration,
};

fn main() {
    let (tx, rx) = mpsc::channel();

    App::new()
        // `MinimalPlugins` has no window. The runner calls `app.update()` every 10 ms.
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(10))))
        .insert_resource(Io {
            tx,
            rx: Mutex::new(rx),
        })
        .add_message::<UserPrompt>()
        .add_message::<ModelDone>()
        .add_message::<ToolFinished>()
        .add_systems(Startup, spawn_sessions)
        // Bring outside events into the World.
        .add_systems(PreUpdate, (drain_io, simulate_user))
        // The agent loop. The order matters: first apply what came back, then send out new work.
        // Bevy applies `Commands` between chained systems, so each system sees the markers that the
        // previous one inserted.
        .add_systems(
            Update,
            (
                start_turn,
                apply_model_reply,
                apply_tool_results,
                finish_tool_round,
                dispatch_tools,
                dispatch_samples,
            )
                .chain(),
        )
        .add_systems(Last, exit_when_done)
        .run();
}

// ---------------------------------------------------------------------------------------------
// Data
// ---------------------------------------------------------------------------------------------

/// The conversation so far. It is sent to the model on every turn.
#[derive(Component, Default)]
struct Transcript(Vec<Msg>);

#[derive(Clone, Debug)]
enum Msg {
    User(String),
    Assistant { text: String, calls: Vec<ToolCall> },
    ToolResult { call_id: u32, output: String },
}

/// Prompts that the fake user will type, one after another, whenever the session is idle.
#[derive(Component)]
struct UserScript(VecDeque<&'static str>);

// Session phase markers. A session has exactly one of these at a time.

/// Waiting for the user.
#[derive(Component)]
struct Idle;

/// Ready to send the transcript to the model.
#[derive(Component)]
struct NeedsSample;

/// A model request is in flight.
#[derive(Component)]
struct Sampling;

/// The model asked for tools. Waiting until every tool call has an output.
#[derive(Component)]
struct WaitingTools;

/// One tool call requested by the model. It is spawned as its own entity, a child of the session.
#[derive(Component, Clone, Debug)]
struct ToolCall {
    id: u32,
    name: &'static str,
    args: &'static str,
}

// Tool call phase markers: `Pending` -> `Running` -> has a `ToolOutput`.
// A permission check would fit between `Pending` and `Running`.

#[derive(Component)]
struct Pending;

#[derive(Component)]
struct Running;

#[derive(Component)]
struct ToolOutput(String);

// Messages: events that arrive from outside the systems.

#[derive(Message)]
struct UserPrompt {
    session: Entity,
    text: String,
}

#[derive(Message)]
struct ModelDone {
    session: Entity,
    text: String,
    calls: Vec<ToolCall>,
}

#[derive(Message)]
struct ToolFinished {
    call: Entity,
    output: String,
}

/// The bridge to background threads. Threads send results into `tx`. `drain_io` reads `rx`.
#[derive(Resource)]
struct Io {
    tx: mpsc::Sender<IoEvent>,
    rx: Mutex<mpsc::Receiver<IoEvent>>,
}

enum IoEvent {
    Model(ModelDone),
    Tool(ToolFinished),
}

// ---------------------------------------------------------------------------------------------
// Systems
// ---------------------------------------------------------------------------------------------

fn spawn_sessions(mut commands: Commands) {
    commands.spawn((
        Name::new("alice"),
        Transcript::default(),
        UserScript(VecDeque::from(["fix the failing test", "thanks!"])),
        Idle,
    ));
    commands.spawn((
        Name::new("bob"),
        Transcript::default(),
        UserScript(VecDeque::from(["what does this repo do?"])),
        Idle,
    ));
}

/// `PreUpdate`: move everything the background threads finished into messages.
fn drain_io(
    io: Res<Io>,
    mut model_done: MessageWriter<ModelDone>,
    mut tool_finished: MessageWriter<ToolFinished>,
) {
    for event in io.rx.lock().unwrap().try_iter() {
        match event {
            IoEvent::Model(done) => {
                model_done.write(done);
            }
            IoEvent::Tool(finished) => {
                tool_finished.write(finished);
            }
        }
    }
}

/// `PreUpdate`: the fake user types the next prompt whenever their session is idle.
fn simulate_user(
    mut sessions: Query<(Entity, &mut UserScript), With<Idle>>,
    mut prompts: MessageWriter<UserPrompt>,
) {
    for (session, mut script) in &mut sessions {
        if let Some(text) = script.0.pop_front() {
            prompts.write(UserPrompt {
                session,
                text: text.to_string(),
            });
        }
    }
}

/// `Idle` -> `NeedsSample`: a user prompt starts a new chain of turns.
fn start_turn(
    mut prompts: MessageReader<UserPrompt>,
    mut sessions: Query<(&Name, &mut Transcript), With<Idle>>,
    mut commands: Commands,
) {
    for prompt in prompts.read() {
        let Ok((name, mut transcript)) = sessions.get_mut(prompt.session) else {
            continue;
        };
        println!("{name:>5} | user: {}", prompt.text);
        transcript.0.push(Msg::User(prompt.text.clone()));
        commands
            .entity(prompt.session)
            .remove::<Idle>()
            .insert(NeedsSample);
    }
}

/// `NeedsSample` -> `Sampling`: send the transcript to the model on a background thread.
fn dispatch_samples(
    sessions: Query<(Entity, &Transcript), With<NeedsSample>>,
    io: Res<Io>,
    mut commands: Commands,
) {
    for (session, transcript) in &sessions {
        let tx = io.tx.clone();
        let messages = transcript.0.clone();
        thread::spawn(move || {
            let (text, calls) = fake_model(&messages);
            let _ = tx.send(IoEvent::Model(ModelDone {
                session,
                text,
                calls,
            }));
        });
        commands
            .entity(session)
            .remove::<NeedsSample>()
            .insert(Sampling);
    }
}

/// `Sampling` -> `Idle` or `WaitingTools`: store the model's reply. Each tool call becomes an entity.
fn apply_model_reply(
    mut replies: MessageReader<ModelDone>,
    mut sessions: Query<(&Name, &mut Transcript), With<Sampling>>,
    mut commands: Commands,
) {
    for reply in replies.read() {
        let Ok((name, mut transcript)) = sessions.get_mut(reply.session) else {
            continue;
        };
        println!("{name:>5} | model: {}", reply.text);
        transcript.0.push(Msg::Assistant {
            text: reply.text.clone(),
            calls: reply.calls.clone(),
        });

        let mut session = commands.entity(reply.session);
        session.remove::<Sampling>();
        if reply.calls.is_empty() {
            // Text only: this chain of turns is done.
            session.insert(Idle);
        } else {
            session.insert(WaitingTools);
            for call in &reply.calls {
                println!("{name:>5} |   tool call: {}({})", call.name, call.args);
                commands.spawn((call.clone(), Pending, ChildOf(reply.session)));
            }
        }
    }
}

/// `Pending` -> `Running`: start every pending tool call on a background thread.
fn dispatch_tools(
    calls: Query<(Entity, &ToolCall), With<Pending>>,
    io: Res<Io>,
    mut commands: Commands,
) {
    for (entity, call) in &calls {
        let tx = io.tx.clone();
        let call = call.clone();
        thread::spawn(move || {
            let output = fake_tool(&call);
            let _ = tx.send(IoEvent::Tool(ToolFinished {
                call: entity,
                output,
            }));
        });
        commands.entity(entity).remove::<Pending>().insert(Running);
    }
}

/// `Running` -> `ToolOutput`: store each finished tool result on its tool call entity.
fn apply_tool_results(mut finished: MessageReader<ToolFinished>, mut commands: Commands) {
    for result in finished.read() {
        commands
            .entity(result.call)
            .remove::<Running>()
            .insert(ToolOutput(result.output.clone()));
    }
}

/// `WaitingTools` -> `NeedsSample`: once every tool call has an output, add the results to the
/// transcript, despawn the tool call entities, and start the next turn.
fn finish_tool_round(
    mut sessions: Query<(Entity, &Name, &Children, &mut Transcript), With<WaitingTools>>,
    calls: Query<(&ToolCall, Option<&ToolOutput>)>,
    mut commands: Commands,
) {
    for (session, name, children, mut transcript) in &mut sessions {
        let all_done = children
            .iter()
            .all(|call| matches!(calls.get(call), Ok((_, Some(_)))));
        if !all_done {
            continue;
        }

        for call in children.iter() {
            let (tool_call, Some(output)) = calls.get(call).unwrap() else {
                continue;
            };
            println!(
                "{name:>5} |   tool result: {} -> {}",
                tool_call.name, output.0
            );
            transcript.0.push(Msg::ToolResult {
                call_id: tool_call.id,
                output: output.0.clone(),
            });
            commands.entity(call).despawn();
        }
        commands
            .entity(session)
            .remove::<WaitingTools>()
            .insert(NeedsSample);
    }
}

/// `Last`: quit when every session is idle and no user has anything left to type.
fn exit_when_done(
    sessions: Query<(&Name, Has<Idle>, &UserScript, &Transcript)>,
    mut exit: MessageWriter<AppExit>,
) {
    let done = sessions
        .iter()
        .all(|(_, idle, script, _)| idle && script.0.is_empty());
    if !done {
        return;
    }

    println!();
    for (name, _, _, transcript) in &sessions {
        println!("{name}'s transcript has {} messages:", transcript.0.len());
        for msg in &transcript.0 {
            match msg {
                Msg::User(text) => println!("  user       {text}"),
                Msg::Assistant { text, calls } => {
                    println!("  assistant  {text} ({} tool calls)", calls.len());
                }
                Msg::ToolResult { call_id, output } => {
                    println!("  tool #{call_id:<3}  {output}");
                }
            }
        }
    }
    exit.write(AppExit::Success);
}

// ---------------------------------------------------------------------------------------------
// Fake outside world: a model and some tools
// ---------------------------------------------------------------------------------------------

/// Pretend file system state: has the bug in `src/lib.rs` been fixed yet?
static FIXED: AtomicBool = AtomicBool::new(false);

/// A scripted "model". It looks at the latest user prompt and how many tool rounds have happened
/// since, and picks a reply. A real model would get the same input: the whole transcript.
fn fake_model(transcript: &[Msg]) -> (String, Vec<ToolCall>) {
    thread::sleep(Duration::from_millis(300));

    let last_user = transcript
        .iter()
        .rposition(|msg| matches!(msg, Msg::User(_)))
        .unwrap();
    let Msg::User(prompt) = &transcript[last_user] else {
        unreachable!()
    };
    let round = transcript.len() - last_user - 1;

    // Each tool round adds 2 messages: the assistant's reply and the tool result.
    let (text, tool) = if prompt.contains("fix") {
        match round {
            0 => ("Let me read the code.", Some(("read_file", "src/lib.rs"))),
            2 => (
                "add() subtracts. Fixing it.",
                Some(("edit_file", "src/lib.rs")),
            ),
            4 => ("Running the tests.", Some(("run_tests", "add_works"))),
            _ => ("Fixed: add() now returns a + b, and the test passes.", None),
        }
    } else if prompt.contains("what") {
        match round {
            0 => ("Let me check the README.", Some(("read_file", "README.md"))),
            _ => ("It is a tiny math library with one function, add().", None),
        }
    } else {
        ("You're welcome!", None)
    };

    let calls = tool
        .map(|(name, args)| ToolCall {
            id: transcript.len() as u32,
            name,
            args,
        })
        .into_iter()
        .collect();
    (text.to_string(), calls)
}

fn fake_tool(call: &ToolCall) -> String {
    thread::sleep(Duration::from_millis(150));
    match (call.name, call.args) {
        ("read_file", "README.md") => "# mathlib: a tiny math library".into(),
        ("read_file", _) if FIXED.load(Ordering::SeqCst) => "fn add(a, b) { a + b }".into(),
        ("read_file", _) => "fn add(a, b) { a - b }".into(),
        ("edit_file", _) => {
            FIXED.store(true, Ordering::SeqCst);
            "replaced a - b with a + b".into()
        }
        ("run_tests", _) if FIXED.load(Ordering::SeqCst) => "test add_works ... ok".into(),
        ("run_tests", _) => "test add_works ... FAILED".into(),
        _ => "unknown tool".into(),
    }
}
