use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::entities::product::Product;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductResponse {
    /// Product's unique identifier
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: uuid::Uuid,
    /// Product name
    #[schema(example = "Awesome Product")]
    pub name: String,
    /// Product description
    #[schema(example = "This is an awesome product")]
    pub description: String,
    /// Product price
    #[schema(example = "99.99")]
    pub price: rust_decimal::Decimal,
    /// Product creation timestamp
    #[schema(example = "2024-02-16T00:00:00Z")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Product last update timestamp
    #[schema(example = "2024-02-16T00:00:00Z")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductsListResponse {
    /// List of products
    pub products: Vec<ProductResponse>,
    /// Total number of products
    #[schema(example = 10)]
    pub total: i64,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}

impl From<Vec<Product>> for ProductsListResponse {
    fn from(products: Vec<Product>) -> Self {
        Self {
            total: products.len() as i64,
            products: products.into_iter().map(ProductResponse::from).collect(),
        }
    }
}