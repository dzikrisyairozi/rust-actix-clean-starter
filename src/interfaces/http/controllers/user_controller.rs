use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use crate::{
    application::{
        use_cases::{
            user::{CreateUserUseCase, UpdateUserUseCase, DeleteUserUseCase, GetUserUseCase},
            UseCase,
        },
        error::ApplicationError,
    },
    domain::entities::user::{CreateUserDto, UpdateUserDto},
    infrastructure::persistence::postgres::PostgresUserRepository,
    interfaces::http::requests::user_requests::UpdateUserRequest,
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

    pub async fn update_user(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
        user_data: web::Json<UpdateUserRequest>,
    ) -> impl Responder {
        let repository = PostgresUserRepository::new(pool.get_ref().clone());
        let use_case = UpdateUserUseCase::new(repository);

        // Convert request to DTO
        let update_dto = UpdateUserDto {
            email: user_data.email.clone(),
            username: user_data.username.clone(),
            password: user_data.password.clone(),
        };

        match use_case.execute((user_id.into_inner(), update_dto)).await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(ApplicationError::NotFound) => {
                HttpResponse::NotFound().json(json!({ "error": "User not found" }))
            }
            Err(ApplicationError::Validation(msg)) => {
                HttpResponse::BadRequest().json(json!({ "error": msg }))
            }
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }

    pub async fn delete_user(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
    ) -> impl Responder {
        let repository = PostgresUserRepository::new(pool.get_ref().clone());
        let use_case = DeleteUserUseCase::new(repository);

        match use_case.execute(user_id.into_inner()).await {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(ApplicationError::NotFound) => {
                HttpResponse::NotFound().json(json!({
                    "error": "User not found"
                }))
            }
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }

    pub async fn get_user(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
    ) -> impl Responder {
        let repository = PostgresUserRepository::new(pool.get_ref().clone());
        let use_case = GetUserUseCase::new(repository);

        match use_case.execute(user_id.into_inner()).await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(ApplicationError::NotFound) => {
                HttpResponse::NotFound().json(json!({
                    "error": "User not found"
                }))
            }
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }

    // Other controller methods will go here
}