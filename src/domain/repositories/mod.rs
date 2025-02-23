pub mod product_repository;
pub mod user_repository;

pub use product_repository::ProductRepository;
pub use user_repository::UserRepository;

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Not found")]
    NotFound,
    #[error("Duplicate entry")]
    DuplicateEntry,
}
