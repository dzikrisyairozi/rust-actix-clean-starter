use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    /// The unique identifier for the user
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    /// User's email address
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    /// User's username
    #[schema(example = "johndoe")]
    pub username: String,
    /// Hashed password
    #[schema(example = "hashed_password")]
    pub password_hash: String,
    /// When the user was created
    #[schema(example = "2024-02-16T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    #[schema(example = "2024-02-16T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserDto {
    /// User's email address
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    /// User's username
    #[schema(example = "johndoe")]
    pub username: String,
    /// User's password (will be hashed)
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserDto {
    /// Updated email address
    #[schema(example = "john.doe.updated@example.com")]
    pub email: Option<String>,
    /// Updated username
    #[schema(example = "johndoe_updated")]
    pub username: Option<String>,
    /// New password (will be hashed)
    #[schema(example = "newpassword123")]
    pub password: Option<String>,
}
