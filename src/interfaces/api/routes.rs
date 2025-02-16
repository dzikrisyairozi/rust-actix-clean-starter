use actix_web::web;
use crate::interfaces::http::controllers::{
    user_controller::UserController,
    product_controller::ProductController,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/users")
                    .route("", web::post().to(UserController::create_user))
                    .route("/{id}", web::put().to(UserController::update_user))
                    // Add other user routes
            )
            .service(
                web::scope("/products")
                    .route("", web::post().to(ProductController::create_product))
                    // Add other product routes
            ),
    );
}