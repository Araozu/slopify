use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct TagRecord {
    pub id: Uuid,
    pub name: String,
    pub color: String,
}

pub async fn list_tags(pool: &PgPool, user_id: Uuid) -> Result<Vec<TagRecord>, sqlx::Error> {
    sqlx::query_as::<_, TagRecord>(
        r#"SELECT id, name, color FROM tags WHERE user_id = $1 ORDER BY name ASC"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn create_tag(
    pool: &PgPool,
    user_id: Uuid,
    name: &str,
    color: &str,
) -> Result<TagRecord, sqlx::Error> {
    sqlx::query_as::<_, TagRecord>(
        r#"INSERT INTO tags (id, user_id, name, color) VALUES ($1, $2, $3, $4) RETURNING id, name, color"#,
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(name)
    .bind(color)
    .fetch_one(pool)
    .await
}

pub async fn delete_tag(pool: &PgPool, user_id: Uuid, tag_id: Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query(r#"DELETE FROM tags WHERE id = $1 AND user_id = $2"#)
        .bind(tag_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}

pub async fn add_tag_to_thread(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
    tag_id: Uuid,
) -> Result<(), sqlx::Error> {
    let exists: bool = sqlx::query_scalar::<_, bool>(
        r#"SELECT EXISTS(SELECT 1 FROM tags WHERE id = $1 AND user_id = $2)"#,
    )
    .bind(tag_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    if !exists {
        return Err(sqlx::Error::RowNotFound);
    }

    let thread_exists: bool = sqlx::query_scalar::<_, bool>(
        r#"SELECT EXISTS(SELECT 1 FROM threads WHERE id = $1 AND user_id = $2)"#,
    )
    .bind(thread_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    if !thread_exists {
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query(
        r#"INSERT INTO thread_tags (thread_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"#,
    )
    .bind(thread_id)
    .bind(tag_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_tag_from_thread(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
    tag_id: Uuid,
) -> Result<(), sqlx::Error> {
    let owned: bool = sqlx::query_scalar::<_, bool>(
        r#"SELECT EXISTS(SELECT 1 FROM threads WHERE id = $1 AND user_id = $2)"#,
    )
    .bind(thread_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    if !owned {
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query(r#"DELETE FROM thread_tags WHERE thread_id = $1 AND tag_id = $2"#)
        .bind(thread_id)
        .bind(tag_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_tags_for_thread(
    pool: &PgPool,
    thread_id: Uuid,
) -> Result<Vec<TagRecord>, sqlx::Error> {
    sqlx::query_as::<_, TagRecord>(
        r#"
        SELECT t.id, t.name, t.color
        FROM tags t
        INNER JOIN thread_tags tt ON tt.tag_id = t.id
        WHERE tt.thread_id = $1
        ORDER BY t.name ASC
        "#,
    )
    .bind(thread_id)
    .fetch_all(pool)
    .await
}
