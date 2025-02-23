use crate::application::{error::ApplicationError, use_cases::UseCase};
use crate::domain::{entities::product::Product, repositories::ProductRepository};
use async_trait::async_trait;

pub struct ListProductsUseCase<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> ListProductsUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: ProductRepository + Send + Sync> UseCase<(), Vec<Product>, ApplicationError>
    for ListProductsUseCase<R>
{
    async fn execute(&self, _: ()) -> Result<Vec<Product>, ApplicationError> {
        // Fetch all products
        let products = self.repository.list().await?;
        Ok(products)
    }
}
