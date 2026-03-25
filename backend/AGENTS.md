# Backend Agent Guide

This backend is a Rust service built for long-term transport flexibility.

## Core Rule

- Keep transport code separate from domain/application code.
- HTTP, SSE, and future WebSocket or Tauri adapters must be thin wrappers around shared backend logic.
- Do not put provider orchestration or chat assembly directly inside Axum handlers.

## Architectural Intent

- `src/http/` is for Axum-specific transport code only.
- `src/chat/` is for client-facing contracts and transport-agnostic streaming/message logic.
- Future Tauri integration should be able to reuse chat/domain modules without depending on Axum SSE types.

## Rust Workflow

- Use `cargo check` after backend changes.
- Prefer small modules with explicit responsibilities.
- Keep serde models typed for shared fields and preserve provider-specific extras in metadata.
- Prefer enums over stringly-typed state when the state is part of the stable contract.

## Streaming Guidance

- SSE formatting belongs in transport modules.
- Stream production belongs in transport-agnostic modules.
- If WebSockets are added later, reuse the same chat event stream/types and add a separate transport mapper.

## Editing Guidelines

- Keep comments minimal and only for non-obvious design constraints.
- Avoid introducing framework-specific types into shared modules unless that module is explicitly transport-specific.
