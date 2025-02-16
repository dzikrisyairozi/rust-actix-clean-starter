use async_trait::async_trait;
use rust_decimal_macros::dec;
use crate::application::{
    error::ApplicationError,
    use_cases::UseCase,
};
use crate::domain::{
    entities::product::{CreateProductDto, Product},
    repositories::ProductRepository,
};

pub struct CreateProductUseCase<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> CreateProductUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: ProductRepository + Send + Sync> UseCase<CreateProductDto, Product, ApplicationError> for CreateProductUseCase<R> {
    async fn execute(&self, input: CreateProductDto) -> Result<Product, ApplicationError> {
        // Validate input
        if input.name.is_empty() {
            return Err(ApplicationError::Validation("Product name is required".to_string()));
        }

        if input.price < dec!(0) {
            return Err(ApplicationError::Validation("Price cannot be negative".to_string()));
        }

        if input.stock < 0 {
            return Err(ApplicationError::Validation("Stock cannot be negative".to_string()));
        }

        // Create product
        let product = self.repository.create(input).await?;
        Ok(product)
    }
}