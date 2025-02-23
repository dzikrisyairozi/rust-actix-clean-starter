use crate::interfaces::api::docs::ApiDoc;
use crate::interfaces::http::controllers::{
    product_controller::ProductController, user_controller::UserController,
};
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Swagger UI
        .service(SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // API routes
        .service(
            web::scope("/api/v1")
                .service(
                    web::scope("/users")
                        .route("", web::get().to(UserController::list_users))
                        .route("", web::post().to(UserController::create_user))
                        .route("/{id}", web::get().to(UserController::get_user))
                        .route("/{id}", web::put().to(UserController::update_user))
                        .route("/{id}", web::delete().to(UserController::delete_user)),
                )
                .service(
                    web::scope("/products")
                        .route("", web::get().to(ProductController::list_products))
                        .route("", web::post().to(ProductController::create_product))
                        .route("/{id}", web::get().to(ProductController::get_product))
                        .route("/{id}", web::put().to(ProductController::update_product))
                        .route("/{id}", web::delete().to(ProductController::delete_product)),
                ),
        );
}
