use std::{fs, io, path::Path};

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use blocking::unblock;
use serde::Deserialize;
use serde_json::json;

use super::{spawn, with_path};
use crate::{ToolApp, ToolCall, ToolRun, ToolSpec};

/// Adds the `search_replace` tool, which edits a file by replacing text, or
/// writes a new file when `old_string` is empty.
#[derive(Clone, Copy, Debug, Default)]
pub struct SearchReplacePlugin;

impl Plugin for SearchReplacePlugin {
    fn build(&self, app: &mut App) {
        let spec = ToolSpec::new(
            "search_replace",
            "Replace `old_string` with `new_string` in a file. `old_string` must match exactly \
             once, unless `replace_all` is true. An empty `old_string` writes `new_string` as \
             the whole file, creating it if needed.",
            json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "The file to change, absolute or relative to the working directory."
                    },
                    "old_string": {
                        "type": "string",
                        "description": "The text to replace."
                    },
                    "new_string": {
                        "type": "string",
                        "description": "The text to replace it with."
                    },
                    "replace_all": {
                        "type": "boolean",
                        "description": "Replace every match of old_string. Defaults to false."
                    }
                },
                "required": ["file_path", "old_string", "new_string"]
            }),
        );
        app.add_tool(spec, search_replace);
    }
}

#[derive(Deserialize)]
struct Args {
    file_path: String,
    old_string: String,
    new_string: String,
    #[serde(default)]
    replace_all: bool,
}

fn search_replace(In(call): In<ToolCall>) -> ToolRun {
    spawn(&call, |args: Args| unblock(move || edit(args)))
}

fn edit(args: Args) -> io::Result<String> {
    let path = &args.file_path;
    if args.old_string.is_empty() {
        if let Some(dir) = Path::new(path).parent() {
            fs::create_dir_all(dir).map_err(with_path(path))?;
        }
        fs::write(path, &args.new_string).map_err(with_path(path))?;
        return Ok(format!("wrote {path}"));
    }

    let text = fs::read_to_string(path).map_err(with_path(path))?;
    let text = match text.matches(&args.old_string).count() {
        0 => return Err(io::Error::other(format!("{path}: old_string not found"))),
        1 => text.replacen(&args.old_string, &args.new_string, 1),
        _ if args.replace_all => text.replace(&args.old_string, &args.new_string),
        count => {
            return Err(io::Error::other(format!(
                "{path}: old_string matches {count} times; add more context or set replace_all"
            )))
        }
    };
    fs::write(path, text).map_err(with_path(path))?;
    Ok(format!("edited {path}"))
}
