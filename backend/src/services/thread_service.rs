use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    storage::threads as thread_storage,
    threads::contracts::Thread,
};

const DEFAULT_THREAD_TITLE: &str = "New thread";

#[derive(Debug)]
pub enum ThreadServiceError {
    Storage(sqlx::Error),
}

impl std::fmt::Display for ThreadServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage(error) => write!(f, "failed to load thread data: {error}"),
        }
    }
}

impl std::error::Error for ThreadServiceError {}

impl From<sqlx::Error> for ThreadServiceError {
    fn from(value: sqlx::Error) -> Self {
        Self::Storage(value)
    }
}

pub async fn list_threads(pool: &PgPool) -> Result<Vec<Thread>, ThreadServiceError> {
    let threads = thread_storage::list_threads(pool).await?;
    Ok(threads.into_iter().map(map_thread).collect())
}

pub async fn create_thread(
    pool: &PgPool,
    title: Option<String>,
) -> Result<Thread, ThreadServiceError> {
    let trimmed_title = title
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_THREAD_TITLE);
    let thread = thread_storage::create_thread(pool, Uuid::new_v4(), trimmed_title).await?;

    Ok(map_thread(thread))
}

fn map_thread(record: thread_storage::ThreadRecord) -> Thread {
    Thread {
        id: record.id.to_string(),
        title: record.title,
    }
}
