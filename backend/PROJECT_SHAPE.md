# Backend Project Shape

## Goals

- Keep HTTP concerns separate from chat-domain contracts.
- Make provider adapters easy to add without leaking vendor-specific payloads everywhere.
- Normalize streamed output into a stable client-facing event shape.

## Current Structure

```text
backend/
|- Cargo.toml
|- PROJECT_SHAPE.md
`- src/
   |- app.rs
   |- config.rs
   |- main.rs
   |- chat/
   |  |- contracts.rs
   |  `- mod.rs
   `- http/
      |- mod.rs
      |- routes.rs
      `- handlers/
         |- health.rs
         |- mod.rs
         `- streams.rs
```

## Folder Responsibilities

- `src/main.rs`
  - Bootstraps Tokio, loads config, binds TCP listener, starts Axum.
- `src/app.rs`
  - Builds the top-level router and composes modules.
- `src/config.rs`
  - Holds environment-backed runtime config.
- `src/http/`
  - HTTP-only concerns: routes, handlers, transport-specific responses like SSE.
- `src/chat/`
  - Client-facing chat contracts and future provider-agnostic message/event types.

## Existing Endpoints

- `GET /health`
  - Basic JSON health check.
- `GET /api/v1/streams/hello`
  - Demo SSE endpoint that emits a started event, two deltas, and a completed event.

## Suggested Next Folders

When the backend grows, add these without reshuffling the whole codebase:

```text
src/
|- providers/
|  |- anthropic.rs
|  |- gemini.rs
|  |- openai_compatible.rs
|  |- mod.rs
|  `- shared.rs
|- services/
|  `- chat_service.rs
|- storage/
|  |- conversations.rs
|  `- mod.rs
|- auth/
|  `- mod.rs
`- errors/
   `- mod.rs
```

## Client-Facing Event Shape

The backend currently models a thin, stable chat protocol:

- `ClientChatMessage`
  - Final client-facing message object.
- `ClientChatEvent`
  - Stream event enum with started, delta, completed, and failed variants.
- `vendor_metadata`
  - Preserves provider-specific extras without polluting the shared schema.

## Design Notes

- Use typed common fields for UI-critical data.
- Store provider extras under metadata instead of pretending all providers match.
- Prefer SSE for model output streaming; add WebSockets later only for truly bidirectional flows.
