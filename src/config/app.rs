use super::{DatabaseConfig, Environment, LoggerConfig};
use log::info;

pub struct AppConfig {
    pub env: Environment,
    pub db: DatabaseConfig,
}

impl AppConfig {
    pub async fn new() -> Self {
        // Load environment variables
        dotenv::dotenv().ok();

        // Initialize environment
        let env = Environment::new();

        // Initialize logger
        LoggerConfig::init(&env.rust_log);

        info!("Initializing database connection...");
        // Initialize database
        let db = DatabaseConfig::init_pool(&env.database_url).await;

        Self { env, db }
    }
}
