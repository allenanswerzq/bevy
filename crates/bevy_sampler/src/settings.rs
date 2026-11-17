//! Where and how to send requests.

use core::fmt;
use std::time::Duration;

use bevy_ecs::prelude::*;

/// The wire protocol of the endpoint.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Backend {
    /// `POST /chat/completions` (`OpenAI` style, most compatible).
    #[default]
    ChatCompletions,
    /// `POST /responses` (`OpenAI` and xAI, supports hosted tools).
    Responses,
    /// `POST /messages` (Anthropic).
    Messages,
}

impl Backend {
    pub(crate) fn path(self) -> &'static str {
        match self {
            Self::ChatCompletions => "chat/completions",
            Self::Responses => "responses",
            Self::Messages => "messages",
        }
    }
}

/// How the API key is sent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AuthScheme {
    /// `x-api-key` for [`Backend::Messages`], `Bearer` for the others.
    #[default]
    Auto,
    /// `Authorization: Bearer <key>`.
    Bearer,
    /// `x-api-key: <key>`.
    XApiKey,
}

impl AuthScheme {
    /// Whether the key goes in an `x-api-key` header instead of `Authorization: Bearer`.
    pub(crate) fn uses_x_api_key(self, backend: Backend) -> bool {
        match self {
            Self::Auto => backend == Backend::Messages,
            Self::XApiKey => true,
            Self::Bearer => false,
        }
    }
}

/// When and how often to retry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetryPolicy {
    /// Retries after the first attempt.
    pub max_retries: u32,
    /// Total attempts allowed when rate limited (429).
    pub rate_limit_attempts: u32,
    /// Never retry once output has been streamed.
    pub only_before_output: bool,
    /// First backoff delay; doubles on each retry.
    pub base_delay: Duration,
    /// Backoff cap.
    pub max_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 15,
            rate_limit_attempts: 2,
            only_before_output: false,
            base_delay: Duration::from_secs(2),
            max_delay: Duration::from_secs(30),
        }
    }
}

/// Server-side detection of a model repeating itself (xAI).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoomLoopPolicy {
    /// Abort when a `tail_repetition` trigger is at or below this value.
    pub max_threshold: u64,
    /// Restarts with a recovery reminder. The last one runs unchecked.
    pub max_retries: u32,
    /// Tokens the server inspects.
    pub window_tokens: u64,
}

impl Default for DoomLoopPolicy {
    fn default() -> Self {
        Self {
            max_threshold: 64,
            max_retries: 2,
            window_tokens: 1024,
        }
    }
}

/// Where and how requests are sent. [`SamplerPlugin`](crate::SamplerPlugin)
/// adds the default; insert your own before or after adding the plugin, and
/// change it at any time. To use different settings for one entity, add
/// [`SamplerOverride`](crate::SamplerOverride).
///
/// ```
/// # use bevy_sampler::{Backend, SamplerSettings};
/// let settings = SamplerSettings {
///     backend: Backend::Responses,
///     model: "grok-4".into(),
///     api_key: std::env::var("XAI_API_KEY").ok(),
///     ..Default::default()
/// };
/// ```
///
/// Its `Debug` output hides the API key and the values of headers and query
/// parameters, so it is safe to log.
#[derive(Resource, Clone)]
pub struct SamplerSettings {
    /// Wire protocol.
    pub backend: Backend,
    /// API root. Defaults to `https://api.x.ai/v1`.
    pub base_url: String,
    /// Model name. Empty by default, so set it.
    pub model: String,
    /// API key, if the endpoint needs one.
    pub api_key: Option<String>,
    /// How the key is sent.
    pub auth: AuthScheme,
    /// Extra headers for every request.
    pub headers: Vec<(String, String)>,
    /// Extra query parameters for every request.
    pub query: Vec<(String, String)>,
    /// Most requests streaming at once. Only the resource's value counts;
    /// entities with a [`SamplerOverride`](crate::SamplerOverride) share it.
    pub max_in_flight: usize,
    /// Fail when the stream produces nothing for this long.
    pub idle_timeout: Duration,
    /// Retry behavior.
    pub retry: RetryPolicy,
    /// Doom-loop detection; off when `None`.
    pub doom_loop: Option<DoomLoopPolicy>,
    /// Compress request bodies of 64 KiB or more with zstd.
    pub compress: bool,
    /// Your own client, for proxies, custom CAs, or mTLS.
    pub http: Option<reqwest::Client>,
    /// Open a connection to the API at startup.
    pub prewarm: bool,
}

impl Default for SamplerSettings {
    fn default() -> Self {
        Self {
            backend: Backend::default(),
            base_url: "https://api.x.ai/v1".into(),
            model: String::new(),
            api_key: None,
            auth: AuthScheme::default(),
            headers: Vec::new(),
            query: Vec::new(),
            max_in_flight: 64,
            idle_timeout: Duration::from_secs(300),
            retry: RetryPolicy::default(),
            doom_loop: None,
            compress: false,
            http: None,
            prewarm: false,
        }
    }
}

impl fmt::Debug for SamplerSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names = |pairs: &[(String, String)]| {
            pairs
                .iter()
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>()
        };
        f.debug_struct("SamplerSettings")
            .field("backend", &self.backend)
            .field("base_url", &self.base_url)
            .field("model", &self.model)
            .field("api_key", &self.api_key.as_ref().map(|_| "<redacted>"))
            .field("auth", &self.auth)
            .field("headers", &names(&self.headers))
            .field("query", &names(&self.query))
            .field("max_in_flight", &self.max_in_flight)
            .field("idle_timeout", &self.idle_timeout)
            .field("retry", &self.retry)
            .field("doom_loop", &self.doom_loop)
            .field("compress", &self.compress)
            .field("http", &self.http.is_some())
            .field("prewarm", &self.prewarm)
            .finish()
    }
}
