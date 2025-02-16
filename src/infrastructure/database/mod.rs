use sqlx::PgPool;
use log::{info, error};
use std::time::Duration;

pub async fn init_db(database_url: &str) -> Result<PgPool, sqlx::Error> {
    info!("Initializing database connection...");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(60))
        .connect(database_url)
        .await?;

    // Verify connection
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => info!("Database connection successful"),
        Err(e) => {
            error!("Database connection failed: {}", e);
            return Err(e);
        }
    }

    Ok(pool)
}