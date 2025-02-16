use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use crate::{
    application::{
        use_cases::{
            product::{CreateProductUseCase, UpdateProductUseCase, DeleteProductUseCase},
            UseCase,
        },
        error::ApplicationError,
    },
    domain::entities::product::{CreateProductDto, UpdateProductDto},
    infrastructure::persistence::postgres::PostgresProductRepository,
    interfaces::http::{
        requests::product_requests::{CreateProductRequest, UpdateProductRequest},
        responses::product_responses::ProductResponse,
    },
};

pub struct ProductController;

impl ProductController {
    pub async fn create_product(
        pool: web::Data<sqlx::PgPool>,
        product_data: web::Json<CreateProductDto>,
    ) -> impl Responder {
        let repository = PostgresProductRepository::new(pool.get_ref().clone());
        let use_case = CreateProductUseCase::new(repository);

        match use_case.execute(product_data.into_inner()).await {
            Ok(product) => HttpResponse::Created().json(product),
            Err(ApplicationError::Validation(msg)) => {
                HttpResponse::BadRequest().json(json!({ "error": msg }))
            }
            Err(ApplicationError::Repository(_)) => {
                HttpResponse::InternalServerError().json(json!({ "error": "Internal server error" }))
            }
            Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Internal server error" })),
        }
    }

    pub async fn update_product(
        pool: web::Data<sqlx::PgPool>,
        product_id: web::Path<Uuid>,
        product_data: web::Json<UpdateProductRequest>,
    ) -> impl Responder {
        let repository = PostgresProductRepository::new(pool.get_ref().clone());
        let use_case = UpdateProductUseCase::new(repository);

        let update_dto = UpdateProductDto {
            name: product_data.name.clone(),
            description: product_data.description.clone(),
            price: product_data.price,
            stock: product_data.stock,
        };

        match use_case.execute((product_id.into_inner(), update_dto)).await {
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

    pub async fn delete_product(
        pool: web::Data<sqlx::PgPool>,
        product_id: web::Path<Uuid>,
    ) -> impl Responder {
        let repository = PostgresProductRepository::new(pool.get_ref().clone());
        let use_case = DeleteProductUseCase::new(repository);

        match use_case.execute(product_id.into_inner()).await {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(ApplicationError::NotFound) => {
                HttpResponse::NotFound().json(json!({
                    "error": "Product not found"
                }))
            }
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }

    // Other controller methods will go here
}