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
        /// How long the server asked us to wait, from `retry-after-ms` or
        /// `Retry-After`.
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
    /// The input filled the context window, but the server did not say so. It
    /// either reported more input tokens than fit, or stopped at the output
    /// limit without writing anything.
    #[error("{input} input tokens overflow the {window}-token context window")]
    ContextOverflow {
        /// Input tokens the server reported.
        input: u64,
        /// The model's context window.
        window: u64,
    },
}

const CONTEXT_CODES: [&str; 2] = ["context_length_exceeded", "exceed_context_size_error"];
/// How providers say that the conversation no longer fits, in lowercase.
const CONTEXT_PHRASES: &[&str] = &[
    "prompt is too long",                     // Anthropic
    "prompt too long",                        // z.ai, Ollama
    "prompt exceeds max length",              // z.ai
    "request too large",                      // Anthropic
    "input is too long for requested model",  // Bedrock
    "exceeds the context window",             // OpenAI
    "maximum context length",                 // OpenAI-compatible, OpenRouter, Mistral
    "context window exceeded",                // LiteLLM
    "exceeds the maximum number of tokens",   // Google
    "maximum prompt length",                  // xAI
    "too long for this model",                // xAI
    "reduce the length of the messages",      // Groq
    "maximum allowed input length",           // OpenRouter
    "longer than the model's context length", // Together
    "exceeds the limit of",                   // GitHub Copilot
    "exceeds the available context size",     // llama.cpp
    "greater than the context length",        // LM Studio
    "context window exceeds limit",           // MiniMax
    "exceeded model token limit",             // Kimi
    "but the configured context size is",     // DS4
    "model_context_window_exceeded",          // z.ai
    "range of input length should be",        // Qwen
    "context length exceeded",                // generic
    "context_length_exceeded",                // generic
    "too many tokens",                        // generic
    "token limit exceeded",                   // generic
];
/// Rate limits that would otherwise match a phrase above, such as Bedrock's
/// "Too many tokens, please wait".
const NOT_CONTEXT_PHRASES: &[&str] = &["rate limit", "too many requests", "throttl"];
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

    /// Whether the same request might succeed later. The server's
    /// `x-should-retry` header wins over the status.
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Http { kind, .. } => *kind != HttpKind::Other,
            Self::Status {
                status,
                should_retry,
                ..
            } => should_retry.unwrap_or(
                matches!(status, 408 | 409 | 429)
                    || (*status >= 500 && !matches!(status, 525 | 526)),
            ),
            Self::Stream { .. } | Self::Empty { .. } => true,
            _ => false,
        }
    }

    /// The conversation no longer fits the model. Compact it before trying again.
    pub fn is_context_length(&self) -> bool {
        if let Self::ContextOverflow { .. } = self {
            return true;
        }
        let message = self.server_message().to_lowercase();
        let mentions = |phrases: &[&str]| phrases.iter().any(|p| message.contains(p));
        self.code().is_some_and(|c| CONTEXT_CODES.contains(&c))
            || (mentions(CONTEXT_PHRASES) && !mentions(NOT_CONTEXT_PHRASES))
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
