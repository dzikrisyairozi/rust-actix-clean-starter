use std::env;

#[derive(Debug, Clone)]
pub struct Environment {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub rust_log: String,
    pub environment: EnvironmentType,
}

#[derive(Debug, Clone)]
pub enum EnvironmentType {
    Development,
    Production,
    Testing,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid number"),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "debug".to_string()),
            environment: match env::var("ENVIRONMENT").as_deref() {
                Ok("production") => EnvironmentType::Production,
                Ok("testing") => EnvironmentType::Testing,
                _ => EnvironmentType::Development,
            },
        }
    }

    pub fn is_production(&self) -> bool {
        matches!(self.environment, EnvironmentType::Production)
    }
}
