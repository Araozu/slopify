# Provider Adapter Extensibility Plan

## Problem

The backend was designed to support multiple LLM providers (`PROJECT_SHAPE.md` line 7:
_"Make provider adapters easy to add without leaking vendor-specific payloads everywhere"_),
but the current wiring is entirely hardcoded to a single OpenRouter adapter. There is no trait,
no registry, and no dispatch — just direct static function calls from service to provider.

### Current call chain (all hardcoded)

```
http/handlers/chat.rs:112  →  services/chat_service.rs:56  →  providers/openai_compatible.rs:83
         |                              |                                  |
  hardcodes "openrouter"       imports openai_compatible            hardcodes OPENROUTER_API_URL
  in ProviderDescriptor        directly, calls stream_prompt()      as a compile-time const
```

Adding a second provider today would require editing at least three files in the core path.

---

## Phase 1 — Define the provider trait

Create `src/providers/trait.rs` (re-exported from `providers/mod.rs`) with a trait that captures
the implicit contract already in use:

```rust
// src/providers/trait.rs

use async_trait::async_trait;
use futures_util::stream::BoxStream;
use reqwest::Client;
use serde_json::Value;

use crate::chat::contracts::PromptMessage;

/// The normalized event shape every provider adapter must produce.
/// This already exists in openai_compatible.rs — move it here so it
/// belongs to the contract, not to a single adapter.
#[derive(Debug, Clone)]
pub enum ProviderStreamEvent {
    TextDelta(String),
    ReasoningDelta(String),
    Completed {
        model: String,
        finish_reason: Option<String>,
        vendor_metadata: Value,
    },
}

/// A unified error type for provider failures.
#[derive(Debug)]
pub enum ProviderError {
    Http(reqwest::Error),
    InvalidPayload(serde_json::Error),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

/// The contract every provider adapter implements.
#[async_trait]
pub trait ProviderAdapter: Send + Sync {
    /// A stable identifier for this provider (e.g. "openrouter", "anthropic").
    fn name(&self) -> &str;

    /// The base endpoint URL used by this adapter.
    fn endpoint(&self) -> &str;

    /// Open a streaming completion and return a normalized event stream.
    async fn stream_prompt(
        &self,
        client: &Client,
        messages: &[PromptMessage],
        model: &str,
        api_key: &str,
    ) -> Result<
        BoxStream<'static, Result<ProviderStreamEvent, ProviderError>>,
        ProviderError,
    >;
}
```

### Migration notes

- Move `ProviderStreamEvent` out of `openai_compatible.rs` and into the shared trait module.
  The existing `openai_compatible.rs` keeps its internal SSE parsing but maps into the shared
  event type.
- `OpenAiCompatibleError` gets converted into `ProviderError` at the adapter boundary via a
  `From` impl instead of leaking into `chat_service.rs`.

### Files touched

| File | Change |
|---|---|
| `src/providers/trait.rs` | **New** — trait + shared types |
| `src/providers/mod.rs` | Add `pub mod r#trait;` re-export |
| `src/providers/openai_compatible.rs` | Remove `ProviderStreamEvent` (import from trait), implement `ProviderAdapter` on a new `OpenRouterAdapter` struct |
| `src/services/chat_service.rs` | Replace `openai_compatible::OpenAiCompatibleError` with `ProviderError` in `ChatServiceError::Provider` |

---

## Phase 2 — Implement the adapter struct for OpenRouter

Turn the current free function into a struct that holds configuration and implements the trait:

```rust
// src/providers/openai_compatible.rs (refactored)

pub struct OpenRouterAdapter {
    endpoint: String,
}

impl OpenRouterAdapter {
    pub fn new() -> Self {
        Self {
            endpoint: "https://openrouter.ai/api/v1/chat/completions".to_string(),
        }
    }

    /// Allows pointing at any OpenAI-compatible endpoint.
    pub fn with_endpoint(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
        }
    }
}

#[async_trait]
impl ProviderAdapter for OpenRouterAdapter {
    fn name(&self) -> &str { "openrouter" }
    fn endpoint(&self) -> &str { &self.endpoint }

    async fn stream_prompt(
        &self,
        client: &Client,
        messages: &[PromptMessage],
        model: &str,
        api_key: &str,
    ) -> Result<BoxStream<'static, Result<ProviderStreamEvent, ProviderError>>, ProviderError> {
        // ... existing SSE logic, using self.endpoint instead of the const
    }
}
```

The `with_endpoint` constructor means this single struct already covers any OpenAI-compatible
service (OpenRouter, Together, Groq, local vLLM, etc.) — you just pass a different URL.

### Files touched

| File | Change |
|---|---|
| `src/providers/openai_compatible.rs` | Wrap in struct, implement trait, remove hardcoded const |

---

## Phase 3 — Build the provider registry

Create a simple registry that maps provider names to trait objects and lives in `AppState`:

```rust
// src/providers/registry.rs

use std::collections::HashMap;
use std::sync::Arc;

use super::r#trait::ProviderAdapter;

pub struct ProviderRegistry {
    adapters: HashMap<String, Arc<dyn ProviderAdapter>>,
    default: String,
}

impl ProviderRegistry {
    pub fn new(default: &str) -> Self {
        Self {
            adapters: HashMap::new(),
            default: default.to_string(),
        }
    }

    pub fn register(&mut self, adapter: Arc<dyn ProviderAdapter>) {
        self.adapters.insert(adapter.name().to_string(), adapter);
    }

    /// Resolve by explicit name, or fall back to the default.
    pub fn resolve(&self, name: Option<&str>) -> Option<Arc<dyn ProviderAdapter>> {
        let key = name.unwrap_or(&self.default);
        self.adapters.get(key).cloned()
    }

    pub fn list(&self) -> Vec<&str> {
        self.adapters.keys().map(String::as_str).collect()
    }
}
```

Then wire it into `AppState`:

```rust
// src/state.rs
pub struct AppState {
    pub http_client: Client,
    pub db_pool: PgPool,
    pub providers: Arc<ProviderRegistry>,   // <-- new
}
```

And build it during startup in `main.rs` / `app.rs`:

```rust
let mut registry = ProviderRegistry::new("openrouter");
registry.register(Arc::new(OpenRouterAdapter::new()));
// registry.register(Arc::new(AnthropicAdapter::new()));  ← future
```

### Files touched

| File | Change |
|---|---|
| `src/providers/registry.rs` | **New** — registry type |
| `src/providers/mod.rs` | Add `pub mod registry;` |
| `src/state.rs` | Add `providers: Arc<ProviderRegistry>` |
| `src/main.rs` or `src/app.rs` | Build registry at startup |

---

## Phase 4 — Route through the registry in the service layer

`chat_service::stream_prompt` currently imports `openai_compatible` directly. Replace that with
a registry lookup:

```rust
// src/services/chat_service.rs (refactored signature)

pub async fn stream_prompt(
    client: &Client,
    providers: &ProviderRegistry,          // <-- new
    provider_name: Option<&str>,           // <-- new (from request payload)
    prompt: String,
    messages: Vec<PromptMessage>,
    model: String,
    authorization: Option<&str>,
) -> Result<BoxStream<'static, ...>, ChatServiceError> {
    let (_, trimmed_model, api_key) = validate_request(&prompt, &model, authorization)?;

    let adapter = providers
        .resolve(provider_name)
        .ok_or(ChatServiceError::UnknownProvider)?;

    let stream = adapter
        .stream_prompt(client, &messages, trimmed_model, api_key)
        .await?;

    // ... same mapping logic as today
}
```

### Files touched

| File | Change |
|---|---|
| `src/services/chat_service.rs` | Accept registry + provider name; resolve dynamically; replace `OpenAiCompatibleError` with `ProviderError` |
| `src/http/handlers/chat.rs` | Pass `state.providers` and the request's provider name into the service; stop hardcoding `"openrouter"` in `ProviderDescriptor` — use `adapter.name()` and `adapter.endpoint()` instead |

---

## Phase 5 — Accept provider in the request payload

Extend the HTTP payload so the frontend can choose a provider:

```rust
// in http/handlers/chat.rs
#[derive(Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
    pub model: String,
    pub thread_id: Option<Uuid>,
    pub system_prompt_id: Option<Uuid>,
    pub provider: Option<String>,          // <-- new, optional, defaults to registry default
}
```

The handler passes `payload.provider.as_deref()` to the service layer.

### Files touched

| File | Change |
|---|---|
| `src/http/handlers/chat.rs` | Add `provider` field to `PromptRequest`; thread it through |

---

## Phase 6 — Add a second adapter (proof of extensibility)

To validate the design, add a native Anthropic adapter:

```rust
// src/providers/anthropic.rs

pub struct AnthropicAdapter;

#[async_trait]
impl ProviderAdapter for AnthropicAdapter {
    fn name(&self) -> &str { "anthropic" }
    fn endpoint(&self) -> &str { "https://api.anthropic.com/v1/messages" }

    async fn stream_prompt(&self, ...) -> ... {
        // Anthropic uses a different request shape and SSE format;
        // parse it here, emit the same ProviderStreamEvent variants.
    }
}
```

Registration is one line in startup:

```rust
registry.register(Arc::new(AnthropicAdapter::new()));
```

No other file in the core path needs to change. That's the goal.

### Files touched

| File | Change |
|---|---|
| `src/providers/anthropic.rs` | **New** — Anthropic adapter |
| `src/providers/mod.rs` | Add `pub mod anthropic;` |
| `src/main.rs` / `src/app.rs` | One line to register it |

---

## Dependency additions

| Crate | Why |
|---|---|
| `async-trait` | Required for `async fn` in the `ProviderAdapter` trait (until Rust stabilizes async trait methods without boxing — nightly has it, stable does not as of 1.85) |

Check the project's Rust edition / MSRV. If on nightly or edition 2024, you can skip
`async-trait` and use native `async fn in trait` + `#[trait_variant::make(Send)]`.

---

## Execution order

| Step | Phase | Estimated scope |
|---|---|---|
| 1 | Phase 1 — trait + shared types | Small, foundational |
| 2 | Phase 2 — refactor OpenRouter into struct | Medium, mostly moving code |
| 3 | Phase 3 — registry + AppState wiring | Small |
| 4 | Phase 4 — service layer dispatch | Medium, touches the hot path |
| 5 | Phase 5 — request payload extension | Small |
| 6 | Phase 6 — second adapter (Anthropic) | Medium, proves the design |

Each phase compiles and works independently. You can ship Phases 1-5 without adding a second
adapter and still get a cleaner architecture. Phase 6 is the validation step.

---

## What this does NOT cover

- **API key management per provider.** Today, keys come from the user's `Authorization` header
  and are stored per-user for OpenRouter only (`openrouter_api_keys` table). A multi-provider
  world needs a `provider_api_keys` table keyed on `(user_id, provider_name)`, or a similar
  scheme. That's a separate migration + service change.
- **Provider-specific request options** (temperature, top_p, tool use, etc.). The trait's
  `stream_prompt` signature could be extended with an `options: &ProviderOptions` bag later,
  but keeping it minimal for now avoids over-engineering.
- **Tests.** The backend currently has zero tests. A good follow-up would be to add unit tests
  for the registry, the trait mapping, and integration tests for at least one adapter using a
  mock HTTP server (e.g., `wiremock`).
