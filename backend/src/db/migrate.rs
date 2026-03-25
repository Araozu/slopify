use std::path::Path;

use sqlx::{migrate::{MigrateError, Migrator}, PgPool};

pub async fn run_migrations(
    pool: &PgPool,
    migrations_dir: impl AsRef<Path>,
) -> Result<(), MigrateError> {
    let migrator = Migrator::new(migrations_dir.as_ref()).await?;
    migrator.run(pool).await
}
