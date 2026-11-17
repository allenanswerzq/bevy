use std::{io, process::Stdio};

use async_process::Command;
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use serde::Deserialize;
use serde_json::json;

use super::spawn;
use crate::{ToolApp, ToolCall, ToolRun, ToolSpec};

/// Adds the `grep` tool, which searches file contents with ripgrep (`rg`).
///
/// `rg` must be installed.
#[derive(Clone, Copy, Debug, Default)]
pub struct GrepPlugin;

impl Plugin for GrepPlugin {
    fn build(&self, app: &mut App) {
        let spec = ToolSpec::new(
            "grep",
            "Search file contents with a regular expression, using ripgrep. Returns matching \
             lines as `path:line:text`.",
            json!({
                "type": "object",
                "properties": {
                    "pattern": {
                        "type": "string",
                        "description": "The regular expression to search for."
                    },
                    "path": {
                        "type": "string",
                        "description": "The file or directory to search. Defaults to the working directory."
                    },
                    "glob": {
                        "type": "string",
                        "description": "Only search files matching this glob, such as \"*.rs\"."
                    },
                    "-i": {
                        "type": "boolean",
                        "description": "Ignore case."
                    }
                },
                "required": ["pattern"]
            }),
        );
        app.add_tool(spec, grep);
    }
}

#[derive(Deserialize)]
struct Args {
    pattern: String,
    path: Option<String>,
    glob: Option<String>,
    #[serde(rename = "-i", default)]
    ignore_case: bool,
}

fn grep(In(call): In<ToolCall>) -> ToolRun {
    spawn(&call, run)
}

async fn run(args: Args) -> io::Result<String> {
    let mut command = Command::new("rg");
    command.args(["--line-number", "--no-heading", "--color=never"]);
    if args.ignore_case {
        command.arg("--ignore-case");
    }
    if let Some(glob) = &args.glob {
        command.arg("--glob").arg(glob);
    }
    command
        .arg("--regexp")
        .arg(&args.pattern)
        .arg("--")
        .arg(args.path.as_deref().unwrap_or("."));

    let output = command
        .stdin(Stdio::null())
        .kill_on_drop(true)
        .output()
        .await
        .map_err(|error| io::Error::new(error.kind(), format!("could not run rg: {error}")))?;
    // rg exits with 1 when nothing matched, and 2 on errors. It may still have
    // found matches before an error, such as a file it could not read.
    let found = String::from_utf8_lossy(&output.stdout);
    let errors = String::from_utf8_lossy(&output.stderr);
    match output.status.code() {
        Some(0) => Ok(found.into_owned()),
        Some(1) => Ok("no matches".into()),
        _ if !found.is_empty() => Ok(format!("{found}{errors}")),
        _ => Err(io::Error::other(errors.trim().to_owned())),
    }
}
