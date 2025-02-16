use crate::domain::repositories::RepositoryError;

#[derive(thiserror::Error, Debug)]
pub enum ApplicationError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
}