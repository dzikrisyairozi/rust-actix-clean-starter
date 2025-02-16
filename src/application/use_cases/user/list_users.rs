use async_trait::async_trait;
use crate::application::{
    error::ApplicationError,
    use_cases::UseCase,
};
use crate::domain::{
    entities::user::User,
    repositories::UserRepository,
};

pub struct ListUsersUseCase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> ListUsersUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: UserRepository + Send + Sync> UseCase<(), Vec<User>, ApplicationError> for ListUsersUseCase<R> {
    async fn execute(&self, _: ()) -> Result<Vec<User>, ApplicationError> {
        // Fetch all users
        let users = self.repository.list().await?;
        Ok(users)
    }
}