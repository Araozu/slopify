use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ThreadRecord {
    pub id: Uuid,
    pub title: String,
}

pub async fn list_threads(pool: &PgPool, user_id: Uuid) -> Result<Vec<ThreadRecord>, sqlx::Error> {
    sqlx::query_as::<_, ThreadRecord>(
        r#"
        SELECT id, title
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
        RETURNING id, title
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(title)
    .fetch_one(pool)
    .await
}
