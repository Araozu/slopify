use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    storage::threads as thread_storage,
    threads::contracts::{Message, MessagePart, Thread},
};

const DEFAULT_THREAD_TITLE: &str = "New thread";

#[derive(Debug)]
pub enum ThreadServiceError {
    NotFound,
    Storage(sqlx::Error),
}

impl std::fmt::Display for ThreadServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "thread not found"),
            Self::Storage(error) => write!(f, "failed to load thread data: {error}"),
        }
    }
}

impl std::error::Error for ThreadServiceError {}

impl From<sqlx::Error> for ThreadServiceError {
    fn from(value: sqlx::Error) -> Self {
        if matches!(value, sqlx::Error::RowNotFound) {
            Self::NotFound
        } else {
            Self::Storage(value)
        }
    }
}

pub async fn list_threads(pool: &PgPool, user_id: Uuid) -> Result<Vec<Thread>, ThreadServiceError> {
    let threads = thread_storage::list_threads(pool, user_id).await?;
    Ok(threads.into_iter().map(map_thread).collect())
}

pub async fn create_thread(
    pool: &PgPool,
    user_id: Uuid,
    title: Option<String>,
) -> Result<Thread, ThreadServiceError> {
    let trimmed_title = title
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_THREAD_TITLE);
    let thread = thread_storage::create_thread(pool, Uuid::new_v4(), user_id, trimmed_title).await?;

    Ok(map_thread(thread))
}

pub async fn update_thread_model(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
    model: &str,
) -> Result<(), ThreadServiceError> {
    thread_storage::update_thread_model(pool, user_id, thread_id, model).await?;
    Ok(())
}

pub async fn delete_thread(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
) -> Result<(), ThreadServiceError> {
    thread_storage::delete_thread(pool, user_id, thread_id).await?;
    Ok(())
}

pub async fn save_message(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
    role: &str,
    content: &str,
) -> Result<Message, ThreadServiceError> {
    let record = thread_storage::create_message(pool, user_id, thread_id, role, content).await?;
    Ok(map_message(record))
}

pub async fn create_assistant_message_shell(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
    provider: serde_json::Value,
) -> Result<Message, ThreadServiceError> {
    let record = thread_storage::create_assistant_message_shell(pool, user_id, thread_id, provider).await?;
    Ok(map_message(record))
}

pub async fn update_streaming_assistant_message(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
    message_id: Uuid,
    status: &str,
    content: &str,
    parts: Vec<MessagePart>,
    metadata: serde_json::Value,
) -> Result<(), ThreadServiceError> {
    thread_storage::update_message_snapshot(
        pool,
        user_id,
        thread_id,
        message_id,
        status,
        content,
        serde_json::to_value(parts).unwrap_or_else(|_| serde_json::json!([])),
        metadata,
    )
    .await?;

    Ok(())
}

pub async fn list_messages(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
) -> Result<Vec<Message>, ThreadServiceError> {
    let records = thread_storage::list_messages(pool, user_id, thread_id).await?;
    Ok(records.into_iter().map(map_message).collect())
}

fn map_thread(record: thread_storage::ThreadRecord) -> Thread {
    Thread {
        id: record.id.to_string(),
        title: record.title,
        model: record.model,
    }
}

fn map_message(record: thread_storage::MessageRecord) -> Message {
    let mut parts = serde_json::from_value::<Vec<MessagePart>>(record.parts.clone()).unwrap_or_default();
    if parts.is_empty() && !record.content.trim().is_empty() {
        parts.push(MessagePart::Text {
            text: record.content.clone(),
        });
    }

    Message {
        id: record.id.to_string(),
        role: record.role,
        status: record.status,
        parts,
        provider: record.provider,
        metadata: record.metadata,
        content: record.content,
        timestamp: record.created_at.to_rfc3339(),
    }
}
