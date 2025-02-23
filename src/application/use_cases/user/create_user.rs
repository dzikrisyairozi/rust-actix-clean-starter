use crate::application::error::ApplicationError;
use crate::application::use_cases::UseCase;
use crate::domain::{
    entities::user::{CreateUserDto, User},
    repositories::UserRepository,
};
use async_trait::async_trait;

pub struct CreateUserUseCase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: UserRepository + Send + Sync> UseCase<CreateUserDto, User, ApplicationError>
    for CreateUserUseCase<R>
{
    async fn execute(&self, input: CreateUserDto) -> Result<User, ApplicationError> {
        // Validate input
        if input.email.is_empty() || input.username.is_empty() || input.password.is_empty() {
            return Err(ApplicationError::Validation(
                "Email, username and password are required".to_string(),
            ));
        }

        // Check if user already exists
        if let Ok(Some(_)) = self.repository.find_by_email(&input.email).await {
            return Err(ApplicationError::Validation(
                "User with this email already exists".to_string(),
            ));
        }

        // Create user
        let user = self.repository.create(input).await?;
        Ok(user)
    }
}
