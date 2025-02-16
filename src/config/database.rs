use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub pool: PgPool,
}

impl DatabaseConfig {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(60))
            .connect(database_url)
            .await?;

        // Verify database connection
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }

    pub async fn init_pool(database_url: &str) -> Self {
        Self::new(database_url)
            .await
            .expect("Failed to initialize database pool")
    }
}