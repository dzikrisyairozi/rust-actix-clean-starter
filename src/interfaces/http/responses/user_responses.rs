use serde::Serialize;
use crate::domain::entities::user::User;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UsersListResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
}

impl From<Vec<User>> for UsersListResponse {
    fn from(users: Vec<User>) -> Self {
        let users: Vec<UserResponse> = users.into_iter()
            .map(UserResponse::from)
            .collect();
        let total = users.len();
        Self { users, total }
    }
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