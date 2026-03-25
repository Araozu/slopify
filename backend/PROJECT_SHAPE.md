# Backend Project Shape

## Goals

- Keep HTTP concerns separate from chat-domain contracts.
- Keep transport adapters thin so future Tauri integration can reuse the same chat/application logic.
- Make provider adapters easy to add without leaking vendor-specific payloads everywhere.
- Normalize streamed output into a stable client-facing event shape.

## Current Structure

```text
backend/
|- Cargo.toml
|- PROJECT_SHAPE.md
|- migrations/
|- scripts/
`- src/
   |- app.rs
   |- config.rs
   |- main.rs
   |- chat/
   |  |- contracts.rs
   |  |- streams.rs
   |  `- mod.rs
   |- db/
   |  |- migrate.rs
   |  |- pool.rs
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
  - HTTP-only concerns: routes, handlers, and transport-specific responses like SSE.
- `src/chat/`
  - Client-facing chat contracts and transport-agnostic chat streaming/message logic.
- `src/db/`
  - Postgres connection setup and migration execution logic.
- `migrations/`
  - Ordered SQL files applied exactly once.
- `scripts/`
  - Shell entrypoints for local and CI automation.

## Transport Boundary

- `src/chat/streams.rs` owns event production for the demo stream.
- `src/http/handlers/streams.rs` only maps shared chat events into SSE frames.
- This split is intentional so a future Tauri desktop app can consume the same domain/application layer without Axum or SSE types leaking inward.
- If WebSockets are added later, they should serialize the same `ClientChatEvent` values through a separate transport adapter.

## Existing Endpoints

- `GET /health`
  - Basic JSON health check.
- `GET /api/v1/streams/hello`
  - Demo SSE endpoint that emits shared chat events through an HTTP/SSE adapter.

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
|- application/
|  `- chat_streams.rs
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
- Preserve a hard line between transport adapters and reusable chat/application logic so Tauri can slot in later without a rewrite.

## Migration Plan

- Keep migrations as ordered `.sql` files in `backend/migrations/`.
- Run them through `backend/scripts/migrate.sh`.
- The shell script loads `.env`, requires `DATABASE_URL`, and calls a small Rust migration binary.
- The Rust migration runner uses SQLx's built-in migrator, which tracks applied migrations in `_sqlx_migrations` and only applies new files.
- This keeps the workflow automatable, repeatable, and idempotent for local dev, CI, and deploy hooks.
