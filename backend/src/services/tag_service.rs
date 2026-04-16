use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    storage::tags as tag_storage,
    threads::contracts::Tag,
};

#[derive(Debug)]
pub enum TagServiceError {
    NotFound,
    Storage(sqlx::Error),
}

impl std::fmt::Display for TagServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "tag not found"),
            Self::Storage(e) => write!(f, "failed to load tag data: {e}"),
        }
    }
}

impl std::error::Error for TagServiceError {}

impl From<sqlx::Error> for TagServiceError {
    fn from(value: sqlx::Error) -> Self {
        if matches!(value, sqlx::Error::RowNotFound) {
            Self::NotFound
        } else {
            Self::Storage(value)
        }
    }
}

pub async fn list_tags(pool: &PgPool, user_id: Uuid) -> Result<Vec<Tag>, TagServiceError> {
    let records = tag_storage::list_tags(pool, user_id).await?;
    Ok(records
        .into_iter()
        .map(|r| Tag { id: r.id.to_string(), name: r.name, color: r.color })
        .collect())
}

pub async fn create_tag(
    pool: &PgPool,
    user_id: Uuid,
    name: &str,
    color: &str,
) -> Result<Tag, TagServiceError> {
    let record = tag_storage::create_tag(pool, user_id, name, color).await?;
    Ok(Tag { id: record.id.to_string(), name: record.name, color: record.color })
}

pub async fn delete_tag(
    pool: &PgPool,
    user_id: Uuid,
    tag_id: Uuid,
) -> Result<(), TagServiceError> {
    tag_storage::delete_tag(pool, user_id, tag_id).await?;
    Ok(())
}

pub async fn add_tag_to_thread(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
    tag_id: Uuid,
) -> Result<(), TagServiceError> {
    tag_storage::add_tag_to_thread(pool, user_id, thread_id, tag_id).await?;
    Ok(())
}

pub async fn remove_tag_from_thread(
    pool: &PgPool,
    user_id: Uuid,
    thread_id: Uuid,
    tag_id: Uuid,
) -> Result<(), TagServiceError> {
    tag_storage::remove_tag_from_thread(pool, user_id, thread_id, tag_id).await?;
    Ok(())
}
