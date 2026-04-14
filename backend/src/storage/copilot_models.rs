use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct CopilotModelRecord {
    pub id: Uuid,
    pub model_id: String,
}

pub async fn list_models(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<CopilotModelRecord>, sqlx::Error> {
    sqlx::query_as::<_, CopilotModelRecord>(
        r#"
        SELECT id, model_id
        FROM copilot_models
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn create_model(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    model_id: &str,
) -> Result<CopilotModelRecord, sqlx::Error> {
    sqlx::query_as::<_, CopilotModelRecord>(
        r#"
        INSERT INTO copilot_models (id, user_id, model_id)
        VALUES ($1, $2, $3)
        RETURNING id, model_id
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(model_id)
    .fetch_one(pool)
    .await
}

pub async fn delete_model(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM copilot_models
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

pub async fn find_model_by_model_id(
    pool: &PgPool,
    user_id: Uuid,
    model_id: &str,
) -> Result<Option<CopilotModelRecord>, sqlx::Error> {
    sqlx::query_as::<_, CopilotModelRecord>(
        r#"
        SELECT id, model_id
        FROM copilot_models
        WHERE user_id = $1
          AND model_id = $2
        "#,
    )
    .bind(user_id)
    .bind(model_id)
    .fetch_optional(pool)
    .await
}
