use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateProductRequest {
    /// The name of the product
    #[schema(example = "iPhone 14 Pro")]
    pub name: String,
    /// Detailed description of the product
    #[schema(example = "Latest iPhone model with dynamic island")]
    pub description: String,
    /// The price of the product
    #[schema(example = "999.99")]
    pub price: Decimal, // Change from f64 to Decimal
    /// Initial stock quantity
    #[schema(example = "100")]
    pub stock: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateProductRequest {
    /// Updated name of the product
    #[schema(example = "iPhone 14 Pro Max")]
    pub name: Option<String>,
    /// Updated description of the product
    #[schema(example = "Updated description for iPhone")]
    pub description: Option<String>,
    /// Updated price of the product
    #[schema(example = "1099.99")]
    pub price: Option<Decimal>, // Change from Option<f64> to Option<Decimal>
    /// Updated stock quantity
    #[schema(example = "50")]
    pub stock: Option<i32>,
}
