#[path = "../db/mod.rs"]
mod db;

use std::{env, process};

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("migration failed: {error}");
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let database_url = args
        .next()
        .ok_or("expected DATABASE_URL as first argument")?;
    let migrations_dir = args.next().ok_or("expected migrations dir as second argument")?;

    let pool = db::pool::create_pool(&database_url, 1).await?;
    db::migrate::run_migrations(&pool, migrations_dir).await?;

    println!("migrations applied successfully");

    Ok(())
}
