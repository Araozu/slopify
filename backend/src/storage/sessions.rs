use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct SessionRecord {
    pub user_id: Uuid,
}

pub async fn create_session(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO sessions (id, user_id, token)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(token)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_session_by_token(
    pool: &PgPool,
    token: &str,
) -> Result<Option<SessionRecord>, sqlx::Error> {
    sqlx::query_as::<_, SessionRecord>(
        r#"
        SELECT user_id
        FROM sessions
        WHERE token = $1
        "#,
    )
    .bind(token)
    .fetch_optional(pool)
    .await
}

pub async fn delete_sessions_for_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM sessions
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}
