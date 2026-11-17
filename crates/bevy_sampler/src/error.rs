//! Why a sample failed, and how to classify it.

use std::time::Duration;

use thiserror::Error;

/// What went wrong at the transport layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpKind {
    /// Could not connect.
    Connect,
    /// The connection timed out.
    Timeout,
    /// Sending the request failed.
    Request,
    /// The response body broke off.
    Body,
    /// Anything else, such as a malformed URL.
    Other,
}

/// Why a sample failed.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum SampleError {
    /// A transport error.
    #[error("http {kind:?} error: {message}")]
    Http {
        /// What failed.
        kind: HttpKind,
        /// The error and its sources.
        message: String,
    },
    /// The server answered with an error status.
    #[error("status {status}: {message}")]
    Status {
        /// HTTP status.
        status: u16,
        /// Error message from the body.
        message: String,
        /// Error code or type from the body.
        code: Option<String>,
        /// The `Retry-After` header, capped at two minutes.
        retry_after: Option<Duration>,
        /// The `x-should-retry` header.
        should_retry: Option<bool>,
    },
    /// The stream reported an error after it started.
    #[error("stream error: {message}")]
    Stream {
        /// Error message.
        message: String,
        /// Error code or type.
        code: Option<String>,
    },
    /// The stream contained invalid JSON.
    #[error("invalid stream data: {0}")]
    Decode(String),
    /// Nothing arrived for this long.
    #[error("no data for {0:?}")]
    IdleTimeout(Duration),
    /// The model produced no text and no tool calls.
    #[error("empty response (reasoning only: {reasoning_only})")]
    Empty {
        /// It produced reasoning but nothing else.
        reasoning_only: bool,
    },
    /// The output limit cut the response off.
    #[error("response truncated at the output limit")]
    Truncated,
    /// The server flagged the output as repeating itself.
    #[error("model is looping: {triggers:?}")]
    DoomLoop {
        /// The signals that fired.
        triggers: Vec<String>,
    },
}

const CONTEXT_CODES: [&str; 2] = ["context_length_exceeded", "exceed_context_size_error"];
const CONTEXT_PHRASES: [&str; 5] = [
    "prompt is too long",
    "maximum context length",
    "too long for this model",
    "maximum prompt length",
    "request too large",
];
const SIZE_CODES: [&str; 3] = ["413", "payload_too_large", "request_too_large"];
/// Longest server text kept in an error, in bytes.
const MAX_MESSAGE: usize = 2048;

/// Cut text from the server to [`MAX_MESSAGE`] bytes, so a large HTML error
/// page or broken event does not end up in a component.
pub(crate) fn clip(mut text: String) -> String {
    if text.len() > MAX_MESSAGE {
        text.truncate(text.floor_char_boundary(MAX_MESSAGE));
        text.push_str("...");
    }
    text
}

impl SampleError {
    /// The HTTP status, if the server sent one.
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Status { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// The error code from the server.
    pub fn code(&self) -> Option<&str> {
        match self {
            Self::Status { code, .. } | Self::Stream { code, .. } => code.as_deref(),
            _ => None,
        }
    }

    /// The error message from the server.
    pub fn server_message(&self) -> &str {
        match self {
            Self::Status { message, .. } | Self::Stream { message, .. } => message,
            _ => "",
        }
    }

    /// Whether the same request might succeed later.
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Http { kind, .. } => *kind != HttpKind::Other,
            Self::Status { status, .. } => {
                *status == 429 || (*status >= 500 && !matches!(status, 525 | 526))
            }
            Self::Stream { .. } | Self::Empty { .. } => true,
            _ => false,
        }
    }

    /// The conversation no longer fits the model.
    pub fn is_context_length(&self) -> bool {
        let message = self.server_message().to_lowercase();
        self.code().is_some_and(|c| CONTEXT_CODES.contains(&c))
            || CONTEXT_PHRASES.iter().any(|p| message.contains(p))
    }

    /// The request body was too many bytes.
    pub fn is_too_large(&self) -> bool {
        self.status() == Some(413) || self.code().is_some_and(|c| SIZE_CODES.contains(&c))
    }

    /// The provider could not read an image.
    pub fn is_bad_image(&self) -> bool {
        let fits = matches!(self, Self::Stream { .. }) || matches!(self.status(), Some(400 | 500));
        let message = self.server_message();
        fits && (self.code() == Some("invalid_image")
            || message.contains("Could not process image")
            || message.contains(".image.source."))
    }
}

impl From<reqwest::Error> for SampleError {
    fn from(error: reqwest::Error) -> Self {
        // The URL can carry secrets in its query string.
        let error = error.without_url();
        let kind = if error.is_connect() {
            HttpKind::Connect
        } else if error.is_timeout() {
            HttpKind::Timeout
        } else if error.is_request() {
            HttpKind::Request
        } else if error.is_body() || error.is_decode() {
            HttpKind::Body
        } else {
            HttpKind::Other
        };
        let mut message = error.to_string();
        let mut source = core::error::Error::source(&error);
        while let Some(cause) = source {
            message = format!("{message}: {cause}");
            source = cause.source();
        }
        Self::Http { kind, message }
    }
}
