//! What is known about one model.

use alloc::borrow::Cow;
use core::time::Duration;

/// What is known about one model. A field is `None` or empty when it is unknown.
///
/// The built-in provider plugins fill every field pi's catalog has for the
/// model. A sampler can fill in limits later as responses report them.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModelInfo {
    /// Display name, such as "Claude Opus 5".
    pub name: Option<Cow<'static, str>>,
    /// pi's id of the provider serving the model, such as `anthropic`.
    pub provider: Option<Cow<'static, str>>,
    /// pi's id of the request protocol, such as `anthropic-messages`,
    /// `openai-responses`, or `openai-completions`.
    pub api: Option<Cow<'static, str>>,
    /// Endpoint the requests go to.
    pub base_url: Option<Cow<'static, str>>,
    /// Whether the model can think before it answers.
    pub reasoning: Option<bool>,
    /// What the model accepts as input.
    pub input: Cow<'static, [Modality]>,
    /// Context window in tokens.
    pub context_window: Option<u64>,
    /// Largest allowed output in tokens.
    pub max_output_tokens: Option<u64>,
    /// What the model's tokens cost.
    pub cost: Option<Cost>,
    /// The provider's value for each thinking level, in pi's format. A level
    /// mapped to `None` is not supported. A level left out uses its own
    /// name, except [`ThinkingLevel::Xhigh`] and [`ThinkingLevel::Max`], which are
    /// only supported when listed.
    pub thinking_levels: Cow<'static, [(ThinkingLevel, Option<Cow<'static, str>>)]>,
    /// Size limits on a request and its images.
    pub input_limits: Option<InputLimits>,
    /// How long the provider keeps a cached prompt.
    pub prompt_cache: Option<PromptCache>,
    /// Extra HTTP headers to send with each request, as (name, value) pairs.
    pub headers: Cow<'static, [(Cow<'static, str>, Cow<'static, str>)]>,
    /// Provider quirks in pi's `compat` format, as JSON object text, such as
    /// `{"supportsStrictMode":false}`. Its keys depend on [`Self::api`].
    pub compat: Option<Cow<'static, str>>,
}

/// A kind of input a model accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Modality {
    /// Text.
    Text,
    /// Images.
    Image,
}

/// How hard a model thinks before it answers, from least to most.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ThinkingLevel {
    /// No thinking.
    Off,
    /// Minimal thinking.
    Minimal,
    /// Low effort.
    Low,
    /// Medium effort.
    Medium,
    /// High effort.
    High,
    /// Extra high effort.
    Xhigh,
    /// The most effort.
    Max,
}

/// Token prices, in US dollars per million tokens.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Prices {
    /// Uncached input.
    pub input: f64,
    /// Output, reasoning included.
    pub output: f64,
    /// Input read from the prompt cache.
    pub cache_read: f64,
    /// Input written to the prompt cache.
    pub cache_write: f64,
}

/// What a model's tokens cost.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Cost {
    /// Prices of a request below every tier.
    pub prices: Prices,
    /// Prices for long requests.
    pub tiers: Cow<'static, [CostTier]>,
}

impl Cost {
    /// The prices of a request with this many input tokens, cached ones
    /// included: those of the highest tier it goes past.
    pub fn prices(&self, input_tokens: u64) -> Prices {
        self.tiers
            .iter()
            .filter(|tier| input_tokens > tier.input_tokens_above)
            .max_by_key(|tier| tier.input_tokens_above)
            .map_or(self.prices, |tier| tier.prices)
    }
}

/// Prices that apply once a request has more input tokens than a threshold.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CostTier {
    /// The threshold, counting cached input tokens.
    pub input_tokens_above: u64,
    /// Prices of a request past the threshold.
    pub prices: Prices,
}

/// Size limits on a request and its images.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputLimits {
    /// Largest request body in bytes.
    pub max_request_bytes: Option<u64>,
    /// Limits on images.
    pub images: Option<ImageLimits>,
}

/// Limits on the images in a request.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ImageLimits {
    /// Most images in one message.
    pub max_per_message: Option<u32>,
    /// Most images in one request.
    pub max_per_request: Option<u32>,
    /// How to shrink an image before sending it.
    pub resize: Option<ImageResize>,
}

/// How to shrink an image before sending it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ImageResize {
    /// Largest width in pixels.
    pub max_width: Option<u32>,
    /// Largest height in pixels.
    pub max_height: Option<u32>,
    /// Largest encoded size in bytes.
    pub max_bytes: Option<u64>,
    /// JPEG quality, from 1 to 100.
    pub jpeg_quality: Option<u8>,
}

/// How long the provider keeps a cached prompt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PromptCache {
    /// Lifetime with the default retention.
    pub short: Option<Duration>,
    /// Lifetime with long retention.
    pub long: Option<Duration>,
}
