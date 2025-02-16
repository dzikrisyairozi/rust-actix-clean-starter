mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;

use actix_web::{web, App, HttpServer};
use log::info;

use crate::config::AppConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize application configuration
    let config = AppConfig::new().await;
    
    let server_url = format!("{}:{}", config.env.server_host, config.env.server_port);
    info!("Server running at http://{}", server_url);
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Share database pool with app state
            .app_data(web::Data::new(config.db.clone()))
            // Add middleware and routes here
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(&server_url)?
    .run()
    .await
}