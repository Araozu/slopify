use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct OpenAiTokenRecord {
    pub id: Uuid,
    pub name: String,
    pub auth_type: String,
    pub token: String,
}

pub async fn list_tokens(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<OpenAiTokenRecord>, sqlx::Error> {
    sqlx::query_as::<_, OpenAiTokenRecord>(
        r#"
        SELECT id, name, auth_type, token
        FROM openai_tokens
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn create_token(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    name: &str,
    auth_type: &str,
    token: &str,
) -> Result<OpenAiTokenRecord, sqlx::Error> {
    sqlx::query_as::<_, OpenAiTokenRecord>(
        r#"
        INSERT INTO openai_tokens (id, user_id, name, auth_type, token)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, auth_type, token
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(name)
    .bind(auth_type)
    .bind(token)
    .fetch_one(pool)
    .await
}

pub async fn delete_token(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM openai_tokens
        WHERE id = $1
          AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
