use super::RepositoryError;
use crate::domain::entities::product::{CreateProductDto, Product, UpdateProductDto};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, RepositoryError>;
    async fn create(&self, product: CreateProductDto) -> Result<Product, RepositoryError>;
    async fn update(&self, id: Uuid, product: UpdateProductDto)
        -> Result<Product, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
    async fn list(&self) -> Result<Vec<Product>, RepositoryError>;
}
