use async_trait::async_trait;
use uuid::Uuid;
use crate::application::{
    error::ApplicationError,
    use_cases::UseCase,
};
use crate::domain::repositories::ProductRepository;

pub struct DeleteProductUseCase<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> DeleteProductUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: ProductRepository + Send + Sync> UseCase<Uuid, (), ApplicationError> for DeleteProductUseCase<R> {
    async fn execute(&self, id: Uuid) -> Result<(), ApplicationError> {
        // Check if product exists
        if self.repository.find_by_id(id).await?.is_none() {
            return Err(ApplicationError::NotFound);
        }

        // Delete product
        self.repository.delete(id).await?;
        Ok(())
    }
}