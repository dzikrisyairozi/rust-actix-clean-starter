use crate::application::{error::ApplicationError, use_cases::UseCase};
use crate::domain::{entities::product::Product, repositories::ProductRepository};
use async_trait::async_trait;
use uuid::Uuid;

pub struct GetProductUseCase<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> GetProductUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: ProductRepository + Send + Sync> UseCase<Uuid, Product, ApplicationError>
    for GetProductUseCase<R>
{
    async fn execute(&self, id: Uuid) -> Result<Product, ApplicationError> {
        // Fetch product by ID
        let product = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(ApplicationError::NotFound)?;

        Ok(product)
    }
}
