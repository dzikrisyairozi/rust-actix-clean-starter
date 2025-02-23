use crate::domain::entities::user::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    /// User's unique identifier
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: uuid::Uuid,
    /// User's email address
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    /// User's username
    #[schema(example = "johndoe")]
    pub username: String,
    /// User creation timestamp
    #[schema(example = "2024-02-16T00:00:00Z")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// User last update timestamp
    #[schema(example = "2024-02-16T00:00:00Z")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersListResponse {
    /// List of users
    pub users: Vec<UserResponse>,
    /// Total number of users
    #[schema(example = 10)]
    pub total: i64,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<Vec<User>> for UsersListResponse {
    fn from(users: Vec<User>) -> Self {
        Self {
            total: users.len() as i64,
            users: users.into_iter().map(UserResponse::from).collect(),
        }
    }
}
