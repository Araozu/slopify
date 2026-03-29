use axum::{
    extract::{Path, State},
    Json,
    http::{HeaderMap, StatusCode, header},
    response::{
        IntoResponse, Response,
        sse::{Event, KeepAlive, Sse},
    },
};
use async_stream::stream;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    chat::contracts::{
        ChatMessageStatus, ChatRole, ClientChatError, ClientChatEvent, ClientChatMessage,
        ClientChatMessageMetadata, ClientChatPart, PromptMessage, ProviderDescriptor,
    },
    http::auth::AuthSession,
    services::{
        chat_service::{self, ChatServiceError, ChatServiceStreamEvent},
        thread_service::{self, ThreadServiceError},
    },
    state::AppState,
};
use std::{convert::Infallible, time::Duration};

#[derive(Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
    pub model: String,
    pub thread_id: Option<Uuid>,
}

#[derive(Serialize)]
struct ChatEventEnvelope {
    payload: ClientChatEvent,
}

pub async fn complete_prompt(
    State(state): State<AppState>,
    session: AuthSession,
    headers: HeaderMap,
    Json(payload): Json<PromptRequest>,
) -> Response {
    let authorization = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let thread_id = payload.thread_id;
    let prompt_messages = if let Some(thread_id) = thread_id {
        if let Err(error) = thread_service::save_message(
            &state.db_pool,
            session.user_id,
            thread_id,
            "user",
            &payload.prompt,
        )
        .await
        {
            return ApiError::from(error).into_response();
        }

        match thread_service::list_messages(&state.db_pool, session.user_id, thread_id).await {
            Ok(messages) => messages
                .into_iter()
                .filter_map(|m| {
                    let content = m.content.trim().to_string();
                    if content.is_empty() {
                        return None;
                    }
                    let role = ChatRole::parse(&m.role)?;
                    Some(PromptMessage { role, content })
                })
                .collect::<Vec<_>>(),
            Err(error) => return ApiError::from(error).into_response(),
        }
    } else {
        vec![PromptMessage {
            role: ChatRole::User,
            content: payload.prompt.clone(),
        }]
    };

    let stream = match chat_service::stream_prompt(
        &state.http_client,
        payload.prompt.clone(),
        prompt_messages,
        payload.model.clone(),
        authorization,
    )
    .await
    {
        Ok(stream) => stream,
        Err(error) => return ApiError::from(error).into_response(),
    };

    let db_pool = state.db_pool.clone();
    let user_id = session.user_id;
    let requested_model = payload.model.clone();

    let sse_stream = stream! {
        let mut text = String::new();
        let mut reasoning = String::new();
        let mut finish_reason: Option<String> = None;
        let mut vendor_metadata = serde_json::json!({});
        let mut resolved_model = requested_model.clone();
        let mut pending_deltas = 0usize;
        let mut last_flush = tokio::time::Instant::now();
        let flush_every = Duration::from_millis(500);
        let flush_delta_count = 48usize;

        let started = match build_initial_message(
            &db_pool,
            user_id,
            thread_id,
            &requested_model,
        ).await {
            Ok(message) => message,
            Err(error) => {
                let event = ClientChatEvent::MessageFailed {
                    message_id: format!("msg_{}", Uuid::new_v4()),
                    error: ClientChatError {
                        code: "storage_error".to_string(),
                        message: error.to_string(),
                        retryable: false,
                    },
                };
                yield Ok::<Event, Infallible>(build_sse_event(event));
                return;
            }
        };

        let message_id = started.id.clone();
        let persisted_message_id = Uuid::parse_str(&started.id).ok();
        yield Ok::<Event, Infallible>(build_sse_event(ClientChatEvent::MessageStarted { message: started.clone() }));

        tokio::pin!(stream);
        while let Some(event) = stream.next().await {
            match event {
                Ok(ChatServiceStreamEvent::TextDelta(delta)) => {
                    text.push_str(&delta);
                    pending_deltas += 1;
                    yield Ok::<Event, Infallible>(build_sse_event(ClientChatEvent::TextDelta {
                        message_id: message_id.clone(),
                        delta,
                    }));
                }
                Ok(ChatServiceStreamEvent::ReasoningDelta(delta)) => {
                    reasoning.push_str(&delta);
                    pending_deltas += 1;
                    yield Ok::<Event, Infallible>(build_sse_event(ClientChatEvent::ReasoningDelta {
                        message_id: message_id.clone(),
                        delta,
                    }));
                }
                Ok(ChatServiceStreamEvent::Completed {
                    model,
                    finish_reason: new_finish_reason,
                    vendor_metadata: new_vendor_metadata,
                }) => {
                    resolved_model = model;
                    finish_reason = new_finish_reason;
                    vendor_metadata = new_vendor_metadata;
                    break;
                }
                Err(error) => {
                    if let (Some(thread_id), Some(message_id_uuid)) = (thread_id, persisted_message_id) {
                        let _ = persist_snapshot(
                            &db_pool,
                            user_id,
                            thread_id,
                            message_id_uuid,
                            "failed",
                            &text,
                            &reasoning,
                            finish_reason.clone(),
                            vendor_metadata.clone(),
                        )
                        .await;
                    }
                    yield Ok::<Event, Infallible>(build_sse_event(ClientChatEvent::MessageFailed {
                        message_id: message_id.clone(),
                        error: ClientChatError {
                            code: "provider_error".to_string(),
                            message: error.to_string(),
                            retryable: false,
                        },
                    }));
                    return;
                }
            }

            let elapsed = last_flush.elapsed();
            if pending_deltas >= flush_delta_count || elapsed >= flush_every {
                if let (Some(thread_id), Some(message_id_uuid)) = (thread_id, persisted_message_id) {
                    let _ = persist_snapshot(
                        &db_pool,
                        user_id,
                        thread_id,
                        message_id_uuid,
                        "streaming",
                        &text,
                        &reasoning,
                        finish_reason.clone(),
                        vendor_metadata.clone(),
                    )
                    .await;
                }
                pending_deltas = 0;
                last_flush = tokio::time::Instant::now();
            }
        }

        let completed_message = ClientChatMessage {
            id: message_id.clone(),
            conversation_id: started.conversation_id.clone(),
            role: ChatRole::Assistant,
            status: ChatMessageStatus::Completed,
            parts: build_parts(&text, &reasoning),
            provider: ProviderDescriptor {
                provider: "openrouter".to_string(),
                model: resolved_model.clone(),
                endpoint: Some("https://openrouter.ai/api/v1/chat/completions".to_string()),
            },
            created_at: started.created_at.clone(),
            metadata: ClientChatMessageMetadata {
                finish_reason: finish_reason.clone(),
                vendor_metadata: vendor_metadata.clone(),
            },
        };

        if let (Some(thread_id), Some(message_id_uuid)) = (thread_id, persisted_message_id) {
            let _ = persist_snapshot(
                &db_pool,
                user_id,
                thread_id,
                message_id_uuid,
                "completed",
                &text,
                &reasoning,
                finish_reason.clone(),
                vendor_metadata.clone(),
            )
            .await;
            let _ = thread_service::update_thread_model(&db_pool, user_id, thread_id, &resolved_model).await;
        }

        yield Ok::<Event, Infallible>(build_sse_event(ClientChatEvent::MessageCompleted {
            message: completed_message,
        }));
    };

    Sse::new(sse_stream)
        .keep_alive(KeepAlive::new().interval(Duration::from_secs(10)).text("ping"))
        .into_response()
}

pub async fn list_thread_messages(
    State(state): State<AppState>,
    session: AuthSession,
    Path(thread_id): Path<Uuid>,
) -> Response {
    match thread_service::list_messages(&state.db_pool, session.user_id, thread_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn chat_options() -> StatusCode {
    StatusCode::NO_CONTENT
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<ChatServiceError> for ApiError {
    fn from(value: ChatServiceError) -> Self {
        match value {
            ChatServiceError::InvalidPrompt | ChatServiceError::InvalidModel => Self {
                status: StatusCode::BAD_REQUEST,
                message: value.to_string(),
            },
            ChatServiceError::MissingApiKey => Self {
                status: StatusCode::UNAUTHORIZED,
                message: value.to_string(),
            },
            ChatServiceError::Provider(error) => Self {
                status: StatusCode::BAD_GATEWAY,
                message: error.to_string(),
            },
        }
    }
}

impl From<ThreadServiceError> for ApiError {
    fn from(value: ThreadServiceError) -> Self {
        match value {
            ThreadServiceError::NotFound => Self {
                status: StatusCode::NOT_FOUND,
                message: value.to_string(),
            },
            ThreadServiceError::Storage(_) => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: value.to_string(),
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(ErrorResponse { error: self.message })).into_response()
    }
}

async fn build_initial_message(
    db_pool: &sqlx::PgPool,
    user_id: Uuid,
    thread_id: Option<Uuid>,
    model: &str,
) -> Result<ClientChatMessage, ThreadServiceError> {
    let provider = ProviderDescriptor {
        provider: "openrouter".to_string(),
        model: model.to_string(),
        endpoint: Some("https://openrouter.ai/api/v1/chat/completions".to_string()),
    };
    let created_at = chrono::Utc::now().to_rfc3339();
    let conversation_id = thread_id
        .map(|id| id.to_string())
        .unwrap_or_else(|| "ephemeral".to_string());

    if let Some(thread_id) = thread_id {
        let shell = thread_service::create_assistant_message_shell(
            db_pool,
            user_id,
            thread_id,
            serde_json::to_value(&provider).unwrap_or_else(|_| serde_json::json!({ "provider": "openrouter" })),
        )
        .await?;
        return Ok(ClientChatMessage {
            id: shell.id.clone(),
            conversation_id,
            role: ChatRole::Assistant,
            status: ChatMessageStatus::Streaming,
            parts: vec![],
            provider,
            created_at: shell.timestamp,
            metadata: ClientChatMessageMetadata {
                finish_reason: None,
                vendor_metadata: serde_json::json!({}),
            },
        });
    }

    Ok(ClientChatMessage {
        id: Uuid::new_v4().to_string(),
        conversation_id,
        role: ChatRole::Assistant,
        status: ChatMessageStatus::Streaming,
        parts: vec![],
        provider,
        created_at,
        metadata: ClientChatMessageMetadata {
            finish_reason: None,
            vendor_metadata: serde_json::json!({}),
        },
    })
}

fn build_parts(text: &str, reasoning: &str) -> Vec<ClientChatPart> {
    let mut parts = Vec::new();
    if !text.is_empty() {
        parts.push(ClientChatPart::Text {
            text: text.to_string(),
        });
    }
    if !reasoning.is_empty() {
        parts.push(ClientChatPart::Reasoning {
            text: reasoning.to_string(),
        });
    }
    parts
}

async fn persist_snapshot(
    db_pool: &sqlx::PgPool,
    user_id: Uuid,
    thread_id: Uuid,
    message_id: Uuid,
    status: &str,
    text: &str,
    reasoning: &str,
    finish_reason: Option<String>,
    vendor_metadata: serde_json::Value,
) -> Result<(), ThreadServiceError> {
    thread_service::update_streaming_assistant_message(
        db_pool,
        user_id,
        thread_id,
        message_id,
        status,
        text,
        build_thread_parts(text, reasoning),
        serde_json::json!({
            "finish_reason": finish_reason,
            "vendor_metadata": vendor_metadata
        }),
    )
    .await
}

fn build_thread_parts(text: &str, reasoning: &str) -> Vec<crate::threads::contracts::MessagePart> {
    let mut parts = Vec::new();
    if !text.is_empty() {
        parts.push(crate::threads::contracts::MessagePart::Text {
            text: text.to_string(),
        });
    }
    if !reasoning.is_empty() {
        parts.push(crate::threads::contracts::MessagePart::Reasoning {
            text: reasoning.to_string(),
        });
    }
    parts
}

fn build_sse_event(payload: ClientChatEvent) -> Event {
    let event_name = match &payload {
        ClientChatEvent::MessageStarted { .. } => "message_started",
        ClientChatEvent::TextDelta { .. } => "text_delta",
        ClientChatEvent::ReasoningDelta { .. } => "reasoning_delta",
        ClientChatEvent::MessageCompleted { .. } => "message_completed",
        ClientChatEvent::MessageFailed { .. } => "message_failed",
    };

    let data = serde_json::to_string(&ChatEventEnvelope { payload }).expect("chat stream payload is valid");
    Event::default().event(event_name).data(data)
}
