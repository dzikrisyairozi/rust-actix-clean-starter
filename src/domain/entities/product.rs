use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Product {
    /// The unique identifier for the product
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    /// The name of the product
    #[schema(example = "iPhone 14 Pro")]
    pub name: String,
    /// Detailed description of the product
    #[schema(example = "Latest iPhone model with dynamic island")]
    pub description: String,
    /// The price of the product
    #[schema(example = "999.99")]
    pub price: Decimal,
    /// Current stock quantity
    #[schema(example = "100")]
    pub stock: i32,
    /// When the product was created
    #[schema(example = "2024-02-16T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    #[schema(example = "2024-02-16T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateProductDto {
    /// The name of the product
    #[schema(example = "iPhone 14 Pro")]
    pub name: String,
    /// Detailed description of the product
    #[schema(example = "Latest iPhone model with dynamic island")]
    pub description: String,
    /// The price of the product
    #[schema(example = "999.99")]
    pub price: Decimal,
    /// Initial stock quantity
    #[schema(example = "100")]
    pub stock: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateProductDto {
    /// Updated name of the product
    #[schema(example = "iPhone 14 Pro Max")]
    pub name: Option<String>,
    /// Updated description of the product
    #[schema(example = "Updated description for iPhone")]
    pub description: Option<String>,
    /// Updated price of the product
    #[schema(example = "1099.99")]
    pub price: Option<Decimal>,
    /// Updated stock quantity
    #[schema(example = "50")]
    pub stock: Option<i32>,
}
