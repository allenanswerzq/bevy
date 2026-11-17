#!/usr/bin/env python3
"""Generate src/providers/ from pi's model catalog.

Run from anywhere: python3 crates/bevy_model/generate_providers.py
To update the catalog, change REV to a newer revision from pi.dev.
"""

import json, os, re, subprocess

names = {
 "amazon-bedrock": ("AmazonBedrock", "Amazon Bedrock"),
 "ant-ling": ("AntLing", "Ant Ling"),
 "anthropic": ("Anthropic", "Anthropic"),
 "azure": ("Azure", "Azure"),
 "baseten": ("Baseten", "Baseten"),
 "cerebras": ("Cerebras", "Cerebras"),
 "cloudflare-ai-gateway": ("CloudflareAiGateway", "Cloudflare AI Gateway"),
 "cloudflare-workers-ai": ("CloudflareWorkersAi", "Cloudflare Workers AI"),
 "deepseek": ("DeepSeek", "DeepSeek"),
 "fireworks": ("Fireworks", "Fireworks"),
 "github-copilot": ("GithubCopilot", "GitHub Copilot"),
 "google": ("Google", "Google"),
 "google-vertex": ("GoogleVertex", "Google Vertex AI"),
 "groq": ("Groq", "Groq"),
 "huggingface": ("HuggingFace", "Hugging Face"),
 "kimi-coding": ("KimiCoding", "Kimi For Coding"),
 "meta": ("Meta", "Meta"),
 "minimax": ("MiniMax", "MiniMax"),
 "minimax-cn": ("MiniMaxCn", "MiniMax CN"),
 "mistral": ("Mistral", "Mistral"),
 "moonshotai": ("MoonshotAi", "Moonshot AI"),
 "moonshotai-cn": ("MoonshotAiCn", "Moonshot AI CN"),
 "nvidia": ("Nvidia", "NVIDIA"),
 "openai": ("OpenAi", "OpenAI"),
 "openai-codex": ("OpenAiCodex", "OpenAI Codex"),
 "opencode": ("OpenCode", "OpenCode Zen"),
 "opencode-go": ("OpenCodeGo", "OpenCode Go"),
 "openrouter": ("OpenRouter", "OpenRouter"),
 "qwen-token-plan": ("QwenTokenPlan", "Qwen Token Plan Team"),
 "qwen-token-plan-cn": ("QwenTokenPlanCn", "Qwen Token Plan Team CN"),
 "qwen-token-plan-individual": ("QwenTokenPlanIndividual", "Qwen Token Plan Individual"),
 "qwen-token-plan-individual-cn": ("QwenTokenPlanIndividualCn", "Qwen Token Plan Individual CN"),
 "radius": ("Radius", "Radius"),
 "together": ("Together", "Together"),
 "typesafe": ("TypeSafe", "TypeSafe"),
 "vercel-ai-gateway": ("VercelAiGateway", "Vercel AI Gateway"),
 "xai": ("Xai", "xAI"),
 "xiaomi": ("Xiaomi", "Xiaomi"),
 "xiaomi-token-plan-ams": ("XiaomiTokenPlanAms", "Xiaomi Token Plan AMS"),
 "xiaomi-token-plan-cn": ("XiaomiTokenPlanCn", "Xiaomi Token Plan CN"),
 "xiaomi-token-plan-sgp": ("XiaomiTokenPlanSgp", "Xiaomi Token Plan SGP"),
 "zai": ("Zai", "Z.AI"),
 "zai-coding-cn": ("ZaiCodingCn", "Z.AI Coding CN"),
}
REV = "sha256-4cbd75daee429216860f6e5b7e13b4ddb12b495f6947061e5777edd27ce0b4f1"
B = chr(96)
URL = f"https://pi.dev/api/models/revisions/{REV}?types=chat"
d = json.loads(subprocess.run(["curl", "-fsSL", URL], check=True, capture_output=True).stdout)
assert set(names) == set(d), set(d) ^ set(names)
ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'src', 'providers')
LEVELS = ["off", "minimal", "low", "medium", "high", "xhigh", "max"]
# Fields that go in the per-file BASE when every model of the provider agrees.
HOISTED = ["provider", "api", "baseUrl", "headers", "inputLimits", "compat", "promptCache"]
ORDER = ["name", "provider", "api", "baseUrl", "reasoning", "input", "contextWindow", "maxTokens", "cost",
         "thinkingLevelMap", "inputLimits", "promptCache", "headers", "compat"]
# Radius-only routing fields, which are not about the model itself.
SKIPPED = {"id", "type", "lab", "enabled", "providers", "free"}


def rstr(s):
    return '"' + s.replace('\\', '\\\\').replace('"', '\\"') + '"'


def cow(s):
    return "Cow::Borrowed(" + rstr(s) + ")"


def num(n):
    assert isinstance(n, int) and not isinstance(n, bool) and n >= 0, n
    return format(n, '_')


def price(x):
    if float(x).is_integer():
        return format(int(x), '_') + ".0"
    r = repr(float(x))
    assert 'e' not in r, r
    return r


def opt(v, f):
    return "None" if v is None else "Some(" + f(v) + ")"


def prices(c):
    return "Prices {{ input: {}, output: {}, cache_read: {}, cache_write: {} }}".format(
        price(c['input']), price(c['output']), price(c['cacheRead']), price(c['cacheWrite']))


def cost(v):
    assert set(v) <= {'input', 'output', 'cacheRead', 'cacheWrite', 'tiers'}, v
    tiers = ", ".join("CostTier {{ input_tokens_above: {}, prices: {} }}".format(num(t['inputTokensAbove']), prices(t))
                      for t in v.get('tiers', []))
    return "Cost {{ prices: {}, tiers: Cow::Borrowed(&[{}]) }}".format(prices(v), tiers)


def resize(r):
    assert set(r) <= {'maxWidth', 'maxHeight', 'maxBytes', 'jpegQuality'}, r
    return "ImageResize {{ max_width: {}, max_height: {}, max_bytes: {}, jpeg_quality: {} }}".format(
        opt(r.get('maxWidth'), num), opt(r.get('maxHeight'), num), opt(r.get('maxBytes'), num),
        opt(r.get('jpegQuality'), num))


def images(i):
    assert set(i) <= {'maxPerMessage', 'maxPerRequest', 'resize'}, i
    return "ImageLimits {{ max_per_message: {}, max_per_request: {}, resize: {} }}".format(
        opt(i.get('maxPerMessage'), num), opt(i.get('maxPerRequest'), num), opt(i.get('resize'), resize))


def input_limits(v):
    assert set(v) <= {'maxRequestBytes', 'images'}, v
    return "InputLimits {{ max_request_bytes: {}, images: {} }}".format(
        opt(v.get('maxRequestBytes'), num), opt(v.get('images'), images))


def secs(s):
    return "Duration::from_secs(" + num(s) + ")"


def prompt_cache(v):
    assert set(v) <= {'short', 'long'}, v
    return "PromptCache {{ short: {}, long: {} }}".format(opt(v.get('short'), secs), opt(v.get('long'), secs))


def thinking(v):
    assert set(v) <= set(LEVELS), v
    return "Cow::Borrowed(&[" + ", ".join(
        "(ThinkingLevel::{}, {})".format(l.capitalize(), opt(v[l], cow)) for l in LEVELS if l in v) + "])"


def compat(v):
    text = json.dumps(v, separators=(',', ':'), ensure_ascii=False)
    assert '"#' not in text
    return 'Some(Cow::Borrowed(r#"' + text + '"#))'


MODALITIES = {"text": "Modality::Text", "image": "Modality::Image"}
FIELDS = {
    "name": ("name", lambda v: "Some(" + cow(v) + ")"),
    "provider": ("provider", lambda v: "Some(" + cow(v) + ")"),
    "api": ("api", lambda v: "Some(" + cow(v) + ")"),
    "baseUrl": ("base_url", lambda v: "Some(" + cow(v) + ")"),
    "reasoning": ("reasoning", lambda v: "Some(" + str(v).lower() + ")"),
    "input": ("input", lambda v: "Cow::Borrowed(&[" + ", ".join(MODALITIES[k] for k in v) + "])"),
    "contextWindow": ("context_window", lambda v: "Some(" + num(v) + ")"),
    "maxTokens": ("max_output_tokens", lambda v: "Some(" + num(v) + ")"),
    "cost": ("cost", lambda v: "Some(" + cost(v) + ")"),
    "thinkingLevelMap": ("thinking_levels", thinking),
    "inputLimits": ("input_limits", lambda v: "Some(" + input_limits(v) + ")"),
    "promptCache": ("prompt_cache", lambda v: "Some(" + prompt_cache(v) + ")"),
    "headers": ("headers", lambda v: "Cow::Borrowed(&[" + ", ".join(
        "(" + cow(k) + ", " + cow(x) + ")" for k, x in v.items()) + "])"),
    "compat": ("compat", compat),
}


def literal(model, keys, base):
    unknown = set(model) - set(ORDER) - SKIPPED
    assert not unknown, unknown
    fields = "".join(FIELDS[k][0] + ": " + FIELDS[k][1](model[k]) + ", " for k in ORDER if k in keys and k in model)
    return "ModelInfo { " + fields + ".." + base + " }"


TYPES = ["Cost", "CostTier", "ImageLimits", "ImageResize", "InputLimits", "Modality", "Prices", "PromptCache",
         "ThinkingLevel"]
os.makedirs(ROOT, exist_ok=True)
mods, total = [], 0
for pid in sorted(d, key=lambda p: p.replace('-', '_')):
    camel, display = names[pid]
    display = " ".join(B + w + B if re.search("[a-z][A-Z]", w) else w for w in display.split())
    models = sorted({m['id']: m for m in d[pid] if m['type'] == 'chat'}.values(), key=lambda m: m['id'])
    total += len(models)
    module = pid.replace('-', '_')
    plugin = camel + "ModelsPlugin"
    shared = [k for k in HOISTED if models and all(k in m for m in models)
              and len({json.dumps(m[k], sort_keys=True) for m in models}) == 1]
    base = literal(models[0], shared, "super::UNKNOWN") if models else "super::UNKNOWN"
    per_model = [k for k in ORDER if k not in shared]
    rows = "\n".join("    (" + rstr(m['id']) + ", " + literal(m, per_model, "BASE") + ")," for m in models)
    body = base + "\n" + rows
    crate_uses = ["ModelInfo"] + [t for t in TYPES if re.search(r"\b" + t + r"\b", body)]
    std_uses = []
    if "Cow::" in body:
        std_uses.append("use alloc::borrow::Cow;\n")
    if "Duration::" in body:
        std_uses.append("use core::time::Duration;\n")
    crate_use = "ModelInfo" if len(crate_uses) == 1 else "{" + ", ".join(sorted(crate_uses)) + "}"
    code = (
        f"//! The models of {display}, pi's provider {B}{pid}{B}.\n\n"
        + "".join(std_uses) + ("\n" if std_uses else "")
        + "use bevy_app::{App, Plugin};\n\n"
        + f"use crate::{crate_use};\n\n"
        + f"/// Registers the {len(models)} models pi lists for {B}{pid}{B}.\n"
        + "#[derive(Clone, Copy, Debug, Default)]\n"
        + f"pub struct {plugin};\n\n"
        + f"impl Plugin for {plugin} {{\n"
        + "    fn build(&self, app: &mut App) {\n"
        + "        super::register(app, MODELS);\n"
        + "    }\n"
        + "}\n\n"
        + ("/// What every model of this provider has in common.\n"
           + f"const BASE: ModelInfo = {base};\n\n" if models else "")
        + "/// Each model's id and what is known about it.\n"
        + "const MODELS: &[(&str, ModelInfo)] = &[\n"
        + rows + ("\n" if rows else "")
        + "];\n"
    )
    open(os.path.join(ROOT, module + '.rs'), 'w').write(code)
    mods.append((module, plugin))

decls = "\n".join(f"mod {m};" for m, _ in mods)
uses = "\n".join(f"pub use {m}::{p};" for m, p in mods)
open(os.path.join(ROOT, 'mod.rs'), 'w').write(
    "//! One plugin per provider, each registering the models that provider serves.\n"
    "//!\n"
    "//! The models come from pi's model catalog, revision\n"
    f"//! {B}{REV}{B}:\n"
    f"//! <https://pi.dev/api/models/revisions/{REV}?types=chat>.\n"
    "//!\n"
    "//! Add the plugin of the provider your sampler talks to. One model id can have\n"
    "//! different limits and prices at different providers, so if two added plugins\n"
    "//! list the same id, the one added last wins.\n"
    "//!\n"
    f"//! Generated by {B}generate_providers.py{B}; edit that instead.\n\n"
    f"{decls}\n\n{uses}\n\n"
    "use alloc::borrow::Cow;\n\n"
    "use bevy_app::App;\n\n"
    "use crate::{ModelApp, ModelInfo};\n\n"
    "/// A model nothing is known about, to build the others from.\n"
    "const UNKNOWN: ModelInfo = ModelInfo {\n"
    "    name: None,\n"
    "    provider: None,\n"
    "    api: None,\n"
    "    base_url: None,\n"
    "    reasoning: None,\n"
    "    input: Cow::Borrowed(&[]),\n"
    "    context_window: None,\n"
    "    max_output_tokens: None,\n"
    "    cost: None,\n"
    "    thinking_levels: Cow::Borrowed(&[]),\n"
    "    input_limits: None,\n"
    "    prompt_cache: None,\n"
    "    headers: Cow::Borrowed(&[]),\n"
    "    compat: None,\n"
    "};\n\n"
    "/// Register each (id, info) entry.\n"
    "fn register(app: &mut App, models: &[(&str, ModelInfo)]) {\n"
    "    for (id, info) in models {\n"
    "        app.register_model(*id, info.clone());\n"
    "    }\n"
    "}\n"
)
subprocess.run(["cargo", "fmt", "-p", "bevy_model"], cwd=os.path.dirname(ROOT), check=True)
print(len(mods), "providers", total, "models")
