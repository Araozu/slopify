use axum::{
    http::{header, Method},
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    http::handlers::{
        auth, chat, copilot_models, copilot_tokens, health, openai_tokens, openrouter_keys,
        openrouter_models, streams, system_prompts, tags, threads, zen_keys,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health_check))
        .nest("/api", api_router())
}

fn api_router() -> Router<AppState> {
    Router::new()
        .route("/v1/auth/register", post(auth::register))
        .route("/v1/auth/login", post(auth::login))
        .route("/v1/auth/logout", post(auth::logout))
        .route("/v1/auth/me", get(auth::me))
        .route(
            "/v1/openrouter-keys",
            get(openrouter_keys::list_openrouter_keys).post(openrouter_keys::create_openrouter_key),
        )
        .route(
            "/v1/openrouter-keys/{key_id}",
            axum::routing::patch(openrouter_keys::update_openrouter_key)
                .delete(openrouter_keys::delete_openrouter_key),
        )
        .route(
            "/v1/openrouter-models",
            get(openrouter_models::list_openrouter_models)
                .post(openrouter_models::create_openrouter_model),
        )
        .route(
            "/v1/openrouter-models/{id}",
            axum::routing::delete(openrouter_models::delete_openrouter_model),
        )
        .route(
            "/v1/copilot-models",
            get(copilot_models::list_copilot_models).post(copilot_models::create_copilot_model),
        )
        .route(
            "/v1/copilot-models/{id}",
            axum::routing::delete(copilot_models::delete_copilot_model),
        )
        .route(
            "/v1/copilot-tokens",
            get(copilot_tokens::list_copilot_tokens),
        )
        .route(
            "/v1/copilot-tokens/{token_id}",
            axum::routing::delete(copilot_tokens::delete_copilot_token),
        )
        .route(
            "/v1/copilot/device-code",
            post(copilot_tokens::initiate_device_code),
        )
        .route(
            "/v1/copilot/device-code/poll",
            post(copilot_tokens::poll_device_code),
        )
        .route(
            "/v1/openai-tokens",
            get(openai_tokens::list_openai_tokens).post(openai_tokens::create_openai_token),
        )
        .route(
            "/v1/openai-tokens/{token_id}",
            axum::routing::delete(openai_tokens::delete_openai_token),
        )
        .route(
            "/v1/openai/device-code",
            post(openai_tokens::initiate_device_code),
        )
        .route(
            "/v1/openai/device-code/poll",
            post(openai_tokens::poll_device_code),
        )
        .route(
            "/v1/system-prompts",
            get(system_prompts::list_system_prompts).post(system_prompts::create_system_prompt),
        )
        .route(
            "/v1/system-prompts/{prompt_id}",
            axum::routing::patch(system_prompts::update_system_prompt)
                .delete(system_prompts::delete_system_prompt),
        )
        .route(
            "/v1/chat/completions",
            post(chat::complete_prompt)
                .options(chat::chat_options)
                .layer(chat_cors_layer()),
        )
        .route(
            "/v1/threads",
            get(threads::list_threads)
                .post(threads::create_thread)
                .options(threads::thread_options)
                .layer(chat_cors_layer()),
        )
        .route(
            "/v1/threads/{thread_id}",
            axum::routing::delete(threads::delete_thread)
                .patch(threads::update_thread)
                .options(threads::thread_options)
                .layer(chat_cors_layer()),
        )
        .route(
            "/v1/threads/{thread_id}/messages/{message_id}",
            axum::routing::delete(chat::delete_message_pair)
                .options(chat::chat_options)
                .layer(chat_cors_layer()),
        )
        .route(
            "/v1/threads/{thread_id}/fork",
            post(threads::fork_thread)
                .options(threads::thread_options)
                .layer(chat_cors_layer()),
        )
        .route(
            "/v1/threads/{thread_id}/messages",
            get(chat::list_thread_messages)
                .options(chat::chat_options)
                .layer(chat_cors_layer()),
        )
        .route("/v1/streams/hello", get(streams::hello_stream))
        .route("/v1/tags", get(tags::list_tags).post(tags::create_tag))
        .route("/v1/tags/{tag_id}", axum::routing::delete(tags::delete_tag))
        .route("/v1/threads/{thread_id}/tags", post(tags::add_tag_to_thread))
        .route(
            "/v1/threads/{thread_id}/tags/{tag_id}",
            axum::routing::delete(tags::remove_tag_from_thread),
        )
        .route(
            "/v1/zen-keys",
            get(zen_keys::list_zen_keys).post(zen_keys::create_zen_key),
        )
        .route(
            "/v1/zen-keys/{key_id}",
            axum::routing::patch(zen_keys::update_zen_key)
                .delete(zen_keys::delete_zen_key),
        )
}

fn chat_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::COOKIE])
}
