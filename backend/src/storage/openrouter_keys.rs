use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct OpenRouterApiKeyRecord {
    pub id: Uuid,
    pub name: String,
    pub api_key: String,
}

pub async fn list_keys(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<OpenRouterApiKeyRecord>, sqlx::Error> {
    sqlx::query_as::<_, OpenRouterApiKeyRecord>(
        r#"
        SELECT id, name, api_key
        FROM openrouter_api_keys
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn create_key(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    name: &str,
    api_key: &str,
) -> Result<OpenRouterApiKeyRecord, sqlx::Error> {
    sqlx::query_as::<_, OpenRouterApiKeyRecord>(
        r#"
        INSERT INTO openrouter_api_keys (id, user_id, name, api_key)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, api_key
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(name)
    .bind(api_key)
    .fetch_one(pool)
    .await
}

pub async fn find_key_by_id(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<Option<OpenRouterApiKeyRecord>, sqlx::Error> {
    sqlx::query_as::<_, OpenRouterApiKeyRecord>(
        r#"
        SELECT id, name, api_key
        FROM openrouter_api_keys
        WHERE id = $1
          AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_key(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    name: &str,
    api_key: &str,
) -> Result<Option<OpenRouterApiKeyRecord>, sqlx::Error> {
    sqlx::query_as::<_, OpenRouterApiKeyRecord>(
        r#"
        UPDATE openrouter_api_keys
        SET name = $3,
            api_key = $4,
            updated_at = NOW()
        WHERE id = $1
          AND user_id = $2
        RETURNING id, name, api_key
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(name)
    .bind(api_key)
    .fetch_optional(pool)
    .await
}

pub async fn delete_key(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM openrouter_api_keys
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
