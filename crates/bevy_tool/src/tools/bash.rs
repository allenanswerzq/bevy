use core::time::Duration;
use std::{io, process::Stdio};

use async_io::Timer;
use async_process::Command;
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_tasks::futures_lite::FutureExt;
use serde::Deserialize;
use serde_json::json;

use super::spawn;
use crate::{ToolApp, ToolCall, ToolRun, ToolSpec};

/// Adds the `bash` tool, which runs a shell command.
#[derive(Clone, Copy, Debug, Default)]
pub struct BashPlugin;

impl Plugin for BashPlugin {
    fn build(&self, app: &mut App) {
        let spec = ToolSpec::new(
            "bash",
            "Run a bash command. Returns its output and, if it failed, its exit status.",
            json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The bash command to run."
                    },
                    "timeout": {
                        "type": "integer",
                        "description": "Timeout in milliseconds. Defaults to 120000."
                    }
                },
                "required": ["command"]
            }),
        );
        app.add_tool(spec, bash);
    }
}

#[derive(Deserialize)]
struct Args {
    command: String,
    timeout: Option<u64>,
}

fn bash(In(call): In<ToolCall>) -> ToolRun {
    spawn(&call, run)
}

async fn run(args: Args) -> io::Result<String> {
    let timeout = Duration::from_millis(args.timeout.unwrap_or(120_000));
    let timed_out = async {
        Timer::after(timeout).await;
        Err(io::Error::new(
            io::ErrorKind::TimedOut,
            format!("timed out after {timeout:?}"),
        ))
    };
    // Dropping the command kills it, which happens on timeout or when the call
    // is canceled.
    let output = Command::new("bash")
        .arg("-c")
        .arg(&args.command)
        .stdin(Stdio::null())
        .kill_on_drop(true)
        .output()
        .or(timed_out)
        .await?;

    let status = if output.status.success() {
        String::new()
    } else {
        format!("\n[{}]", output.status)
    };
    Ok(format!(
        "{}{}{status}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    ))
}
