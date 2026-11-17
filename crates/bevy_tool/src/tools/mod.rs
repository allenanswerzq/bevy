//! The built-in tools. Each one is its own plugin, and
//! [`DefaultTools`](crate::DefaultTools) adds all of them.
//!
//! | Plugin                    | Tool             | What it does                                 |
//! |---------------------------|------------------|----------------------------------------------|
//! | [`BashPlugin`]            | `bash`           | Runs a shell command                         |
//! | [`ReadFilePlugin`]        | `read_file`      | Reads a file, with line numbers              |
//! | [`SearchReplacePlugin`]   | `search_replace` | Replaces text in a file, or writes a new one |
//! | [`ListDirPlugin`]         | `list_dir`       | Lists a directory                            |
//! | [`GrepPlugin`]            | `grep`           | Searches files with ripgrep                  |
//!
//! The tools work on the local machine. Relative paths resolve against the
//! process's current directory. They run in the background, so a slow tool
//! never blocks a frame, and output longer than [`MAX_OUTPUT`] bytes is cut.

mod bash;
mod grep;
mod list_dir;
mod read_file;
mod search_replace;

pub use bash::BashPlugin;
pub use grep::GrepPlugin;
pub use list_dir::ListDirPlugin;
pub use read_file::ReadFilePlugin;
pub use search_replace::SearchReplacePlugin;

use std::io;

use bevy_tasks::IoTaskPool;
use serde::de::DeserializeOwned;

use crate::{ToolCall, ToolOutput, ToolRun};

/// The most output, in bytes, that a built-in tool sends back to the model.
pub const MAX_OUTPUT: usize = 40 * 1024;

/// Parse the call's arguments, then run `work` on the IO task pool.
///
/// Bad arguments and errors become an error [`ToolOutput`], so the model can
/// see what went wrong and try again.
fn spawn<A, F>(call: &ToolCall, work: impl FnOnce(A) -> F) -> ToolRun
where
    A: DeserializeOwned,
    F: Future<Output = io::Result<String>> + Send + 'static,
{
    let arguments = match call.arguments.trim() {
        "" => "{}",
        arguments => arguments,
    };
    let work = match serde_json::from_str(arguments) {
        Ok(args) => work(args),
        Err(error) => return ToolOutput::text(format!("error: bad arguments: {error}")).into(),
    };
    IoTaskPool::get()
        .spawn(async move {
            ToolOutput::text(match work.await {
                Ok(text) => truncate(text),
                Err(error) => format!("error: {error}"),
            })
        })
        .into()
}

/// Cut `text` to [`MAX_OUTPUT`] bytes, and say that it was cut.
fn truncate(mut text: String) -> String {
    if text.len() > MAX_OUTPUT {
        text.truncate(text.floor_char_boundary(MAX_OUTPUT));
        text.push_str("\n[output truncated]");
    }
    text
}

/// Put the path in front of an IO error, so the model knows which file failed.
fn with_path(path: &str) -> impl FnOnce(io::Error) -> io::Error + '_ {
    move |error| io::Error::new(error.kind(), format!("{path}: {error}"))
}
