use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use log::error;

use crate::domain::{
    entities::product::{Product, CreateProductDto, UpdateProductDto},
    repositories::{ProductRepository, RepositoryError},
};

pub struct PostgresProductRepository {
    pool: PgPool,
}

impl PostgresProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for PostgresProductRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, RepositoryError> {
        let product = sqlx::query_as::<_, Product>(
            "SELECT * FROM products WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(product)
    }

    async fn create(&self, product: CreateProductDto) -> Result<Product, RepositoryError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let product = sqlx::query_as::<_, Product>(
            r#"
            INSERT INTO products (id, name, description, price, stock, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(&product.name)
        .bind(&product.description)
        .bind(product.price)
        .bind(product.stock)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(product)
    }

    async fn update(&self, id: Uuid, product: UpdateProductDto) -> Result<Product, RepositoryError> {
        let _current_product = self.find_by_id(id).await?
            .ok_or(RepositoryError::NotFound)?;

        let product = sqlx::query_as::<_, Product>(
            r#"
            UPDATE products 
            SET 
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                price = COALESCE($3, price),
                stock = COALESCE($4, stock),
                updated_at = $5
            WHERE id = $6
            RETURNING *
            "#
        )
        .bind(product.name)
        .bind(product.description)
        .bind(product.price)
        .bind(product.stock)
        .bind(Utc::now())
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(product)
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let result = sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    async fn list(&self) -> Result<Vec<Product>, RepositoryError> {
        match sqlx::query_as::<_, Product>(
            "SELECT * FROM products ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await {
            Ok(products) => Ok(products),
            Err(e) => {
                error!("Database error in list products: {:?}", e);
                Err(RepositoryError::DatabaseError(e.to_string()))
            }
        }
    }
}