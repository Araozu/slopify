use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ThreadRecord {
    pub id: Uuid,
    pub title: String,
    pub model: Option<String>,
}

pub async fn list_threads(pool: &PgPool, user_id: Uuid) -> Result<Vec<ThreadRecord>, sqlx::Error> {
    sqlx::query_as::<_, ThreadRecord>(
        r#"
        SELECT id, title, model
        FROM threads
        WHERE user_id = $1
        ORDER BY updated_at DESC, created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn create_thread(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    title: &str,
) -> Result<ThreadRecord, sqlx::Error> {
    sqlx::query_as::<_, ThreadRecord>(
        r#"
        INSERT INTO threads (id, user_id, title)
        VALUES ($1, $2, $3)
        RETURNING id, title, model
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(title)
    .fetch_one(pool)
    .await
}

pub async fn update_thread_title(
    pool: &PgPool,
    user_id: Uuid,
    id: Uuid,
    title: &str,
) -> Result<ThreadRecord, sqlx::Error> {
    let row = sqlx::query_as::<_, ThreadRecord>(
        r#"
        UPDATE threads
        SET title = $1, updated_at = NOW()
        WHERE id = $2 AND user_id = $3
        RETURNING id, title, model
        "#,
    )
    .bind(title)
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or(sqlx::Error::RowNotFound)
}

pub async fn update_thread_model(
    pool: &PgPool,
    user_id: Uuid,
    id: Uuid,
    model: &str,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE threads
        SET model = $1, updated_at = NOW()
        WHERE id = $2 AND user_id = $3
        "#,
    )
    .bind(model)
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}

pub async fn delete_thread(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM threads
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(thread_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}

#[derive(Debug, Clone, FromRow)]
pub struct MessageRecord {
    pub id: Uuid,
    pub role: String,
    pub status: String,
    pub parts: serde_json::Value,
    pub provider: serde_json::Value,
    pub metadata: serde_json::Value,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_message(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
    role: &str,
    content: &str,
) -> Result<MessageRecord, sqlx::Error> {
    if !thread_exists_for_user(pool, user_id, thread_id).await? {
        return Err(sqlx::Error::RowNotFound);
    }

    let record = sqlx::query_as::<_, MessageRecord>(
        r#"
        INSERT INTO messages (id, thread_id, role, status, content, parts, provider, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, role, status, parts, provider, metadata, content, created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(thread_id)
    .bind(role)
    .bind("completed")
    .bind(content)
    .bind(serde_json::json!([{ "kind": "text", "text": content }]))
    .bind(serde_json::json!({"name": "openrouter"}))
    .bind(serde_json::json!({}))
    .fetch_one(pool)
    .await?;

    sqlx::query(
        r#"
        UPDATE threads
        SET updated_at = NOW()
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(thread_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(record)
}

pub async fn create_assistant_message_shell(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
    provider: serde_json::Value,
) -> Result<MessageRecord, sqlx::Error> {
    if !thread_exists_for_user(pool, user_id, thread_id).await? {
        return Err(sqlx::Error::RowNotFound);
    }

    let record = sqlx::query_as::<_, MessageRecord>(
        r#"
        INSERT INTO messages (id, thread_id, role, status, content, parts, provider, metadata)
        VALUES ($1, $2, 'assistant', 'streaming', '', '[]'::jsonb, $3, '{}'::jsonb)
        RETURNING id, role, status, parts, provider, metadata, content, created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(thread_id)
    .bind(provider)
    .fetch_one(pool)
    .await?;

    touch_thread(pool, user_id, thread_id).await?;

    Ok(record)
}

pub async fn update_message_snapshot(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
    message_id: Uuid,
    status: &str,
    content: &str,
    parts: serde_json::Value,
    metadata: serde_json::Value,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE messages
        SET status = $1, content = $2, parts = $3, metadata = $4
        FROM threads
        WHERE messages.id = $5
          AND messages.thread_id = $6
          AND threads.id = messages.thread_id
          AND threads.user_id = $7
        "#,
    )
    .bind(status)
    .bind(content)
    .bind(parts)
    .bind(metadata)
    .bind(message_id)
    .bind(thread_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    touch_thread(pool, user_id, thread_id).await?;
    Ok(())
}

pub async fn list_messages(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
) -> Result<Vec<MessageRecord>, sqlx::Error> {
    if !thread_exists_for_user(pool, user_id, thread_id).await? {
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query_as::<_, MessageRecord>(
        r#"
        SELECT
            messages.id,
            messages.role,
            messages.status,
            messages.parts,
            messages.provider,
            messages.metadata,
            messages.content,
            messages.created_at
        FROM messages
        INNER JOIN threads ON threads.id = messages.thread_id
        WHERE messages.thread_id = $1 AND threads.user_id = $2
        ORDER BY messages.created_at ASC
        "#,
    )
    .bind(thread_id)
    .bind(user_id)
    .fetch_all(pool)
    .await
}

async fn touch_thread(pool: &PgPool, user_id: Uuid, thread_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE threads
        SET updated_at = NOW()
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(thread_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

async fn thread_exists_for_user(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM threads
            WHERE id = $1 AND user_id = $2
        )
        "#,
    )
    .bind(thread_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
}
