use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ThreadRecord {
    pub id: Uuid,
    pub title: String,
}

pub async fn list_threads(pool: &PgPool) -> Result<Vec<ThreadRecord>, sqlx::Error> {
    sqlx::query_as::<_, ThreadRecord>(
        r#"
        SELECT id, title
        FROM threads
        ORDER BY updated_at DESC, created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn create_thread(
    pool: &PgPool,
    id: Uuid,
    title: &str,
) -> Result<ThreadRecord, sqlx::Error> {
    sqlx::query_as::<_, ThreadRecord>(
        r#"
        INSERT INTO threads (id, title)
        VALUES ($1, $2)
        RETURNING id, title
        "#,
    )
    .bind(id)
    .bind(title)
    .fetch_one(pool)
    .await
}
