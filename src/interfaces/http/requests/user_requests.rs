use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    /// Updated email address (optional)
    #[schema(example = "john.doe.updated@example.com")]
    pub email: Option<String>,
    /// Updated username (optional)
    #[schema(example = "johndoe_updated")]
    pub username: Option<String>,
    /// New password (optional, will be hashed)
    #[schema(example = "newpassword123")]
    pub password: Option<String>,
}