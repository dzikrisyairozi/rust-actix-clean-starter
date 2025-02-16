use async_trait::async_trait;
use uuid::Uuid;
use crate::application::{
    error::ApplicationError,
    use_cases::UseCase,
};
use crate::domain::{
    entities::user::{User, UpdateUserDto},
    repositories::UserRepository,
};

pub struct UpdateUserUseCase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UpdateUserUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: UserRepository + Send + Sync> UseCase<(Uuid, UpdateUserDto), User, ApplicationError> for UpdateUserUseCase<R> {
    async fn execute(&self, input: (Uuid, UpdateUserDto)) -> Result<User, ApplicationError> {
        let (id, update_dto) = input;

        // Validate input
        if let Some(email) = &update_dto.email {
            if email.is_empty() {
                return Err(ApplicationError::Validation("Email cannot be empty".to_string()));
            }
        }

        if let Some(username) = &update_dto.username {
            if username.is_empty() {
                return Err(ApplicationError::Validation("Username cannot be empty".to_string()));
            }
        }

        if let Some(password) = &update_dto.password {
            if password.is_empty() {
                return Err(ApplicationError::Validation("Password cannot be empty".to_string()));
            }
        }

        // Check if user exists
        if self.repository.find_by_id(id).await?.is_none() {
            return Err(ApplicationError::NotFound);
        }

        // Update user
        let user = self.repository.update(id, update_dto).await?;
        Ok(user)
    }
}