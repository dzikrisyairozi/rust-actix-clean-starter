use super::RepositoryError;
use crate::domain::entities::user::{CreateUserDto, UpdateUserDto, User};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError>;
    async fn create(&self, user: CreateUserDto) -> Result<User, RepositoryError>;
    async fn update(&self, id: Uuid, user: UpdateUserDto) -> Result<User, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
    async fn list(&self) -> Result<Vec<User>, RepositoryError>;
}
