use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct SystemPromptRecord {
    pub id: Uuid,
    pub name: String,
    pub content: String,
}

pub async fn list_system_prompts(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<SystemPromptRecord>, sqlx::Error> {
    sqlx::query_as::<_, SystemPromptRecord>(
        r#"
        SELECT id, name, content
        FROM system_prompts
        WHERE user_id = $1
        ORDER BY created_at ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn create_system_prompt(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    name: &str,
    content: &str,
) -> Result<SystemPromptRecord, sqlx::Error> {
    sqlx::query_as::<_, SystemPromptRecord>(
        r#"
        INSERT INTO system_prompts (id, user_id, name, content)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, content
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(name)
    .bind(content)
    .fetch_one(pool)
    .await
}

pub async fn find_system_prompt_by_id(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<Option<SystemPromptRecord>, sqlx::Error> {
    sqlx::query_as::<_, SystemPromptRecord>(
        r#"
        SELECT id, name, content
        FROM system_prompts
        WHERE id = $1
          AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_system_prompt(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    name: &str,
    content: &str,
) -> Result<Option<SystemPromptRecord>, sqlx::Error> {
    sqlx::query_as::<_, SystemPromptRecord>(
        r#"
        UPDATE system_prompts
        SET name = $3,
            content = $4,
            updated_at = NOW()
        WHERE id = $1
          AND user_id = $2
        RETURNING id, name, content
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(name)
    .bind(content)
    .fetch_optional(pool)
    .await
}

pub async fn delete_system_prompt(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM system_prompts
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
