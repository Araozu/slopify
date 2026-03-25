use axum::{
    Router,
    routing::{get, post},
};

use crate::http::handlers::{chat, health, streams};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route(
            "/api/v1/chat/completions",
            post(chat::complete_prompt).options(chat::chat_options),
        )
        .route("/api/v1/streams/hello", get(streams::hello_stream))
}
