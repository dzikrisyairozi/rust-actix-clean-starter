use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::{
    application::{
        use_cases::{
            user::CreateUserUseCase,
            UseCase,
        },
        error::ApplicationError,
    },
    domain::entities::user::CreateUserDto,
    infrastructure::persistence::postgres::PostgresUserRepository,
};

pub struct UserController;

impl UserController {
    pub async fn create_user(
        pool: web::Data<sqlx::PgPool>,
        user_data: web::Json<CreateUserDto>,
    ) -> impl Responder {
        let repository = PostgresUserRepository::new(pool.get_ref().clone());
        let use_case = CreateUserUseCase::new(repository);

        match use_case.execute(user_data.into_inner()).await {
            Ok(user) => HttpResponse::Created().json(user),
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