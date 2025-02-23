use crate::{
    domain::entities::{
        product::{CreateProductDto, Product, UpdateProductDto},
        user::{CreateUserDto, UpdateUserDto, User},
    },
    interfaces::http::{
        requests::user_requests::{CreateUserRequest, UpdateUserRequest},
        responses::{
            product_responses::{ProductResponse, ProductsListResponse},
            user_responses::{UserResponse, UsersListResponse},
        },
    },
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Product endpoints
        crate::interfaces::http::controllers::product_controller::list_products_doc,
        crate::interfaces::http::controllers::product_controller::create_product_doc,
        crate::interfaces::http::controllers::product_controller::get_product_doc,
        crate::interfaces::http::controllers::product_controller::update_product_doc,
        crate::interfaces::http::controllers::product_controller::delete_product_doc,
        // User endpoints
        crate::interfaces::http::controllers::user_controller::list_users_doc,
        crate::interfaces::http::controllers::user_controller::create_user_doc,
        crate::interfaces::http::controllers::user_controller::get_user_doc,
        crate::interfaces::http::controllers::user_controller::update_user_doc,
        crate::interfaces::http::controllers::user_controller::delete_user_doc,
    ),
    components(
        schemas(
            // Product schemas
            Product, CreateProductDto, UpdateProductDto,
            ProductResponse, ProductsListResponse,
            // User schemas
            User, CreateUserDto, UpdateUserDto,
            CreateUserRequest, UpdateUserRequest,
            UserResponse, UsersListResponse
        )
    ),
    tags(
        (name = "products", description = "Product management endpoints"),
        (name = "users", description = "User management endpoints")
    ),
    info(
        title = "Rust Clean Architecture API",
        version = "1.0.0",
        description = "REST API built with Rust using clean architecture principles",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    )
)]
pub struct ApiDoc;
