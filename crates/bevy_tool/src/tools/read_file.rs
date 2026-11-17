use std::{fs, io};

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use blocking::unblock;
use serde::Deserialize;
use serde_json::json;

use super::{spawn, with_path};
use crate::{ToolApp, ToolCall, ToolRun, ToolSpec};

/// Lines returned when the model does not give a `limit`.
const DEFAULT_LIMIT: usize = 2000;

/// Adds the `read_file` tool, which reads a text file with line numbers.
#[derive(Clone, Copy, Debug, Default)]
pub struct ReadFilePlugin;

impl Plugin for ReadFilePlugin {
    fn build(&self, app: &mut App) {
        let spec = ToolSpec::new(
            "read_file",
            "Read a text file. Each line starts with its line number and `→`. If the file has \
             more lines, the output ends with the offset to read on from.",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The file to read, absolute or relative to the working directory."
                    },
                    "offset": {
                        "type": "integer",
                        "description": "The line number to start from. Only for files too large to read at once."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "The number of lines to read. Defaults to 2000."
                    }
                },
                "required": ["path"]
            }),
        );
        app.add_tool(spec, read_file);
    }
}

#[derive(Deserialize)]
struct Args {
    path: String,
    offset: Option<usize>,
    limit: Option<usize>,
}

fn read_file(In(call): In<ToolCall>) -> ToolRun {
    spawn(&call, |args: Args| unblock(move || read(args)))
}

fn read(args: Args) -> io::Result<String> {
    let text = fs::read_to_string(&args.path).map_err(with_path(&args.path))?;
    let total = text.lines().count();
    let start = args.offset.unwrap_or(1).max(1);
    let limit = args.limit.unwrap_or(DEFAULT_LIMIT);
    if start > total {
        return Ok(format!("(the file has {total} lines)"));
    }

    let mut out: String = text
        .lines()
        .enumerate()
        .skip(start - 1)
        .take(limit)
        .map(|(index, line)| format!("{}→{line}\n", index + 1))
        .collect();
    let next = start + limit;
    if next <= total {
        out.push_str(&format!(
            "[lines {next} to {total} not shown; read on with offset {next}]"
        ));
    }
    Ok(out)
}
