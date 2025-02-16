pub mod user_repository;
pub mod product_repository;

pub use user_repository::UserRepository;
pub use product_repository::ProductRepository;

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Not found")]
    NotFound,
    #[error("Duplicate entry")]
    DuplicateEntry,
}