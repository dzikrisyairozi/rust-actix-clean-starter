use crate::{
    application::{
        error::ApplicationError,
        use_cases::{
            user::{
                CreateUserUseCase, DeleteUserUseCase, GetUserUseCase, ListUsersUseCase,
                UpdateUserUseCase,
            },
            UseCase,
        },
    },
    domain::entities::user::{CreateUserDto, UpdateUserDto},
    infrastructure::persistence::postgres::PostgresUserRepository,
    interfaces::http::{
        requests::user_requests::{CreateUserRequest, UpdateUserRequest},
        responses::user_responses::{UserResponse, UsersListResponse},
    },
};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

pub struct UserController;

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "users",
    responses(
        (status = 200, description = "List all users successfully", body = UsersListResponse),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn list_users_doc() {}

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Invalid input", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn create_user_doc() {}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn get_user_doc() {}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 400, description = "Invalid input", body = String),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn update_user_doc() {}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn delete_user_doc() {}

impl UserController {
    /// Create a new user
    pub async fn create_user(
        pool: web::Data<sqlx::PgPool>,
        user_data: web::Json<CreateUserRequest>,
    ) -> impl Responder {
        let repository = PostgresUserRepository::new(pool.get_ref().clone());
        let use_case = CreateUserUseCase::new(repository);

        let create_dto = CreateUserDto {
            email: user_data.email.clone(),
            username: user_data.username.clone(),
            password: user_data.password.clone(),
        };

        match use_case.execute(create_dto).await {
            Ok(user) => HttpResponse::Created().json(UserResponse::from(user)),
            Err(ApplicationError::Validation(msg)) => {
                HttpResponse::BadRequest().json(json!({ "error": msg }))
            }
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }

    /// Update an existing user
    pub async fn update_user(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
        user_data: web::Json<UpdateUserRequest>,
    ) -> impl Responder {
        let repository = PostgresUserRepository::new(pool.get_ref().clone());
        let use_case = UpdateUserUseCase::new(repository);

        let update_dto = UpdateUserDto {
            email: user_data.email.clone(),
            username: user_data.username.clone(),
            password: user_data.password.clone(),
        };

        match use_case.execute((user_id.into_inner(), update_dto)).await {
            Ok(user) => HttpResponse::Ok().json(UserResponse::from(user)),
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

    /// Delete a user
    pub async fn delete_user(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
    ) -> impl Responder {
        let repository = PostgresUserRepository::new(pool.get_ref().clone());
        let use_case = DeleteUserUseCase::new(repository);

        match use_case.execute(user_id.into_inner()).await {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(ApplicationError::NotFound) => HttpResponse::NotFound().json(json!({
                "error": "User not found"
            })),
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }

    /// Get a user by ID
    pub async fn get_user(
        pool: web::Data<sqlx::PgPool>,
        user_id: web::Path<Uuid>,
    ) -> impl Responder {
        let repository = PostgresUserRepository::new(pool.get_ref().clone());
        let use_case = GetUserUseCase::new(repository);

        match use_case.execute(user_id.into_inner()).await {
            Ok(user) => HttpResponse::Ok().json(UserResponse::from(user)),
            Err(ApplicationError::NotFound) => HttpResponse::NotFound().json(json!({
                "error": "User not found"
            })),
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }

    /// List all users
    pub async fn list_users(pool: web::Data<sqlx::PgPool>) -> impl Responder {
        let repository = PostgresUserRepository::new(pool.get_ref().clone());
        let use_case = ListUsersUseCase::new(repository);

        match use_case.execute(()).await {
            Ok(users) => {
                let response = UsersListResponse::from(users);
                HttpResponse::Ok().json(response)
            }
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
        }
    }
}
