mod application;
mod config;
mod domain;
mod infrastructure;
mod interfaces;

use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use log::info;
use sqlx::PgPool;

use crate::config::AppConfig;
use crate::interfaces::api::routes::configure_routes;
use crate::interfaces::api::docs::ApiDoc;

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
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .configure(configure_routes)
    })
    .bind(&server_url)?
    .run()
    .await
}