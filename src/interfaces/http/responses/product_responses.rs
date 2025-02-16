use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::entities::product::Product;
use rust_decimal::Decimal;

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub stock: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProductsListResponse {
    pub products: Vec<ProductResponse>,
    pub total: usize,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            stock: product.stock,
            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}

impl From<Vec<Product>> for ProductsListResponse {
    fn from(products: Vec<Product>) -> Self {
        let products: Vec<ProductResponse> = products.into_iter()
            .map(ProductResponse::from)
            .collect();
        let total = products.len();
        Self { products, total }
    }
}