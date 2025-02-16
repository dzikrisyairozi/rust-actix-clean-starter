use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::user::{User, CreateUserDto, UpdateUserDto},
    repositories::{UserRepository, RepositoryError},
};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    async fn create(&self, user: CreateUserDto) -> Result<User, RepositoryError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, username, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.password) // Note: In real app, hash password before storing
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(ref e) if e.constraint() == Some("users_email_key") => {
                RepositoryError::DuplicateEntry
            }
            _ => RepositoryError::DatabaseError(e.to_string()),
        })?;

        Ok(user)
    }

    async fn update(&self, id: Uuid, user: UpdateUserDto) -> Result<User, RepositoryError> {
        let _current_user = self.find_by_id(id).await?
            .ok_or(RepositoryError::NotFound)?;

        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users 
            SET 
                email = COALESCE($1, email),
                username = COALESCE($2, username),
                password_hash = COALESCE($3, password_hash),
                updated_at = $4
            WHERE id = $5
            RETURNING *
            "#
        )
        .bind(user.email)
        .bind(user.username)
        .bind(user.password) // Note: In real app, hash password before storing
        .bind(Utc::now())
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    async fn list(&self) -> Result<Vec<User>, RepositoryError> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(users)
    }
}