use std::{fs, io};

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use blocking::unblock;
use serde::Deserialize;
use serde_json::json;

use super::{spawn, with_path};
use crate::{ToolApp, ToolCall, ToolRun, ToolSpec};

/// Adds the `list_dir` tool, which lists the entries of one directory.
#[derive(Clone, Copy, Debug, Default)]
pub struct ListDirPlugin;

impl Plugin for ListDirPlugin {
    fn build(&self, app: &mut App) {
        let spec = ToolSpec::new(
            "list_dir",
            "List a directory, one entry per line, sorted. Directories end with `/`.",
            json!({
                "type": "object",
                "properties": {
                    "target_directory": {
                        "type": "string",
                        "description": "The directory to list, absolute or relative to the working directory."
                    }
                },
                "required": ["target_directory"]
            }),
        );
        app.add_tool(spec, list_dir);
    }
}

#[derive(Deserialize)]
struct Args {
    target_directory: String,
}

fn list_dir(In(call): In<ToolCall>) -> ToolRun {
    spawn(&call, |args: Args| unblock(move || list(args)))
}

fn list(args: Args) -> io::Result<String> {
    let path = &args.target_directory;
    let mut names = fs::read_dir(path)
        .and_then(|entries| {
            entries
                .map(|entry| {
                    let entry = entry?;
                    let mut name = entry.file_name().to_string_lossy().into_owned();
                    if entry.file_type()?.is_dir() {
                        name.push('/');
                    }
                    Ok(name)
                })
                .collect::<io::Result<Vec<_>>>()
        })
        .map_err(with_path(path))?;
    if names.is_empty() {
        return Ok("(empty directory)".into());
    }
    names.sort();
    Ok(names.join("\n"))
}
