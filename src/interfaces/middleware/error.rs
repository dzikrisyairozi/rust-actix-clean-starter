use actix_web::{
    error::ResponseError,
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
use serde_json::json;
use crate::application::error::ApplicationError;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Not Found")]
    NotFound,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::InternalServerError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": self.to_string()
                }))
            }
            ApiError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(json!({
                    "error": message
                }))
            }
            ApiError::Unauthorized => {
                HttpResponse::Unauthorized().json(json!({
                    "error": self.to_string()
                }))
            }
            ApiError::NotFound => {
                HttpResponse::NotFound().json(json!({
                    "error": self.to_string()
                }))
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<ApplicationError> for ApiError {
    fn from(error: ApplicationError) -> ApiError {
        match error {
            ApplicationError::Validation(msg) => ApiError::BadRequest(msg),
            ApplicationError::NotFound => ApiError::NotFound,
            ApplicationError::Unauthorized => ApiError::Unauthorized,
            _ => ApiError::InternalServerError,
        }
    }
}