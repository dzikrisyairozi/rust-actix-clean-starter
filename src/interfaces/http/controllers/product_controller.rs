use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::{
    application::{
        use_cases::{
            product::CreateProductUseCase,
            UseCase,
        },
        error::ApplicationError,
    },
    domain::entities::product::CreateProductDto,
    infrastructure::persistence::postgres::PostgresProductRepository,
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

    // Other controller methods will go here
}