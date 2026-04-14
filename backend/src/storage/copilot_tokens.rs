use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct CopilotTokenRecord {
    pub id: Uuid,
    pub name: String,
    pub github_token: String,
}

pub async fn list_tokens(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<CopilotTokenRecord>, sqlx::Error> {
    sqlx::query_as::<_, CopilotTokenRecord>(
        r#"
        SELECT id, name, github_token
        FROM copilot_tokens
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
    github_token: &str,
) -> Result<CopilotTokenRecord, sqlx::Error> {
    sqlx::query_as::<_, CopilotTokenRecord>(
        r#"
        INSERT INTO copilot_tokens (id, user_id, name, github_token)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, github_token
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(name)
    .bind(github_token)
    .fetch_one(pool)
    .await
}

pub async fn find_token_by_id(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<Option<CopilotTokenRecord>, sqlx::Error> {
    sqlx::query_as::<_, CopilotTokenRecord>(
        r#"
        SELECT id, name, github_token
        FROM copilot_tokens
        WHERE id = $1
          AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_token(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    name: &str,
    github_token: &str,
) -> Result<Option<CopilotTokenRecord>, sqlx::Error> {
    sqlx::query_as::<_, CopilotTokenRecord>(
        r#"
        UPDATE copilot_tokens
        SET name = $3,
            github_token = $4,
            updated_at = NOW()
        WHERE id = $1
          AND user_id = $2
        RETURNING id, name, github_token
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(name)
    .bind(github_token)
    .fetch_optional(pool)
    .await
}

pub async fn delete_token(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM copilot_tokens
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
