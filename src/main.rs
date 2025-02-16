mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;

use actix_web::{web, App, HttpServer};
use log::info;
use sqlx::PgPool;

use crate::config::AppConfig;
use crate::interfaces::api::routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize application configuration
    let config = AppConfig::new().await;
    
    let server_url = format!("{}:{}", config.env.server_host, config.env.server_port);
    info!("Server running at http://{}", server_url);
    
    // Get the database pool
    let db_pool = config.db.pool;
    
    // Start HTTP server
    HttpServer::new(move || {
        info!("Configuring application routes...");
        App::new()
            // Share database pool with app state
            .app_data(web::Data::new(db_pool.clone()))
            // Add middleware and routes here
            .wrap(actix_web::middleware::Logger::default())
            .configure(configure_routes)
    })
    .bind(&server_url)?
    .run()
    .await
}