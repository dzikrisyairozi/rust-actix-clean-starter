use async_trait::async_trait;
use uuid::Uuid;
use rust_decimal_macros::dec;
use crate::application::{
    error::ApplicationError,
    use_cases::UseCase,
};
use crate::domain::{
    entities::product::{Product, UpdateProductDto},
    repositories::ProductRepository,
};

pub struct UpdateProductUseCase<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> UpdateProductUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: ProductRepository + Send + Sync> UseCase<(Uuid, UpdateProductDto), Product, ApplicationError> for UpdateProductUseCase<R> {
    async fn execute(&self, input: (Uuid, UpdateProductDto)) -> Result<Product, ApplicationError> {
        let (id, update_dto) = input;

        // Validate input
        if let Some(name) = &update_dto.name {
            if name.is_empty() {
                return Err(ApplicationError::Validation("Name cannot be empty".to_string()));
            }
        }

        if let Some(description) = &update_dto.description {
            if description.is_empty() {
                return Err(ApplicationError::Validation("Description cannot be empty".to_string()));
            }
        }

        if let Some(price) = update_dto.price {
            if price < dec!(0) {
                return Err(ApplicationError::Validation("Price cannot be negative".to_string()));
            }
        }

        if let Some(stock) = update_dto.stock {
            if stock < 0 {
                return Err(ApplicationError::Validation("Stock cannot be negative".to_string()));
            }
        }

        // Check if product exists
        if self.repository.find_by_id(id).await?.is_none() {
            return Err(ApplicationError::NotFound);
        }

        // Update product
        let product = self.repository.update(id, update_dto).await?;
        Ok(product)
    }
}