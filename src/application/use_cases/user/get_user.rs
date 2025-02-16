use async_trait::async_trait;
use uuid::Uuid;
use crate::application::{
    error::ApplicationError,
    use_cases::UseCase,
};
use crate::domain::{
    entities::user::User,
    repositories::UserRepository,
};

pub struct GetUserUseCase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> GetUserUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: UserRepository + Send + Sync> UseCase<Uuid, User, ApplicationError> for GetUserUseCase<R> {
    async fn execute(&self, id: Uuid) -> Result<User, ApplicationError> {
        // Fetch user by ID
        let user = self.repository
            .find_by_id(id)
            .await?
            .ok_or(ApplicationError::NotFound)?;

        Ok(user)
    }
}