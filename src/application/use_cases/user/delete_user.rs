use crate::application::{error::ApplicationError, use_cases::UseCase};
use crate::domain::repositories::UserRepository;
use async_trait::async_trait;
use uuid::Uuid;

pub struct DeleteUserUseCase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> DeleteUserUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: UserRepository + Send + Sync> UseCase<Uuid, (), ApplicationError> for DeleteUserUseCase<R> {
    async fn execute(&self, id: Uuid) -> Result<(), ApplicationError> {
        // Check if user exists
        if self.repository.find_by_id(id).await?.is_none() {
            return Err(ApplicationError::NotFound);
        }

        // Delete user
        self.repository.delete(id).await?;
        Ok(())
    }
}
