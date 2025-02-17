use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use serde_json::json;
use uuid::Uuid;

use crate::{
    application::{
        error::ApplicationError,
        use_cases::{
            product::{
                CreateProductUseCase, DeleteProductUseCase, GetProductUseCase, ListProductsUseCase,
                UpdateProductUseCase,
            },
            UseCase,
        },
    },
    domain::entities::product::{CreateProductDto, UpdateProductDto},
    infrastructure::persistence::postgres::PostgresProductRepository,
    interfaces::http::responses::product_responses::{ProductResponse, ProductsListResponse},
};

pub struct ProductController;

#[utoipa::path(
    get,
    path = "/api/v1/products",
    tag = "products",
    responses(
        (status = 200, description = "List all products successfully", body = ProductsListResponse),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn list_products_doc() {}

#[utoipa::path(
    post,
    path = "/api/v1/products",
    tag = "products",
    request_body = CreateProductDto,
    responses(
        (status = 201, description = "Product created successfully", body = ProductResponse),
        (status = 400, description = "Invalid input", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn create_product_doc() {}

#[utoipa::path(
    get,
    path = "/api/v1/products/{id}",
    tag = "products",
    params(
        ("id" = Uuid, Path, description = "Product ID")
    ),
    responses(
        (status = 200, description = "Product found", body = ProductResponse),
        (status = 404, description = "Product not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn get_product_doc() {}

#[utoipa::path(
    put,
    path = "/api/v1/products/{id}",
    tag = "products",
    params(
        ("id" = Uuid, Path, description = "Product ID")
    ),
    request_body = UpdateProductDto,
    responses(
        (status = 200, description = "Product updated successfully", body = ProductResponse),
        (status = 400, description = "Invalid input", body = String),
        (status = 404, description = "Product not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn update_product_doc() {}

#[utoipa::path(
    delete,
    path = "/api/v1/products/{id}",
    tag = "products",
    params(
        ("id" = Uuid, Path, description = "Product ID")
    ),
    responses(
        (status = 204, description = "Product deleted successfully"),
        (status = 404, description = "Product not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn delete_product_doc() {}

impl ProductController {
    /// List all products
    pub async fn list_products(pool: web::Data<sqlx::PgPool>) -> impl Responder {
        let repository = PostgresProductRepository::new(pool.get_ref().clone());
        let use_case = ListProductsUseCase::new(repository);

        match use_case.execute(()).await {
            Ok(products) => {
                info!("Successfully retrieved {} products", products.len());
                let response = ProductsListResponse::from(products);
                HttpResponse::Ok().json(response)
            }
            Err(e) => {
                error!("Error listing products: {:?}", e);
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal server error"
                }))
            }
        }
    }

    /// Create a new product
    pub async fn create_product(
        pool: web::Data<sqlx::PgPool>,
        product_data: web::Json<CreateProductDto>,
    ) -> impl Responder {
        let repository = PostgresProductRepository::new(pool.get_ref().clone());
        let use_case = CreateProductUseCase::new(repository);

        match use_case.execute(product_data.into_inner()).await {
            Ok(product) => HttpResponse::Created().json(ProductResponse::from(product)),
            Err(ApplicationError::Validation(msg)) => {
                HttpResponse::BadRequest().json(json!({ "error": msg }))
            }
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }

    /// Get a product by ID
    pub async fn get_product(
        pool: web::Data<sqlx::PgPool>,
        product_id: web::Path<Uuid>,
    ) -> impl Responder {
        let repository = PostgresProductRepository::new(pool.get_ref().clone());
        let use_case = GetProductUseCase::new(repository);

        match use_case.execute(product_id.into_inner()).await {
            Ok(product) => HttpResponse::Ok().json(ProductResponse::from(product)),
            Err(ApplicationError::NotFound) => HttpResponse::NotFound().json(json!({
                "error": "Product not found"
            })),
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }

    /// Update a product
    pub async fn update_product(
        pool: web::Data<sqlx::PgPool>,
        product_id: web::Path<Uuid>,
        product_data: web::Json<UpdateProductDto>,
    ) -> impl Responder {
        let repository = PostgresProductRepository::new(pool.get_ref().clone());
        let use_case = UpdateProductUseCase::new(repository);

        match use_case
            .execute((product_id.into_inner(), product_data.into_inner()))
            .await
        {
            Ok(product) => HttpResponse::Ok().json(ProductResponse::from(product)),
            Err(ApplicationError::NotFound) => {
                HttpResponse::NotFound().json(json!({ "error": "Product not found" }))
            }
            Err(ApplicationError::Validation(msg)) => {
                HttpResponse::BadRequest().json(json!({ "error": msg }))
            }
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }

    /// Delete a product
    pub async fn delete_product(
        pool: web::Data<sqlx::PgPool>,
        product_id: web::Path<Uuid>,
    ) -> impl Responder {
        let repository = PostgresProductRepository::new(pool.get_ref().clone());
        let use_case = DeleteProductUseCase::new(repository);

        match use_case.execute(product_id.into_inner()).await {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(ApplicationError::NotFound) => HttpResponse::NotFound().json(json!({
                "error": "Product not found"
            })),
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }
}
