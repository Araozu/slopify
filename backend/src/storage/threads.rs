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

#[derive(Debug, Clone, FromRow)]
pub struct MessageRecord {
    pub id: Uuid,
    pub role: String,
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
        INSERT INTO messages (id, thread_id, role, status, content, provider)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, role, content, created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(thread_id)
    .bind(role)
    .bind("completed")
    .bind(content)
    .bind(serde_json::json!({"name": "openrouter"}))
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
        SELECT messages.id, messages.role, messages.content, messages.created_at
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
