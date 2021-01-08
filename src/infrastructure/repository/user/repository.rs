use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::{User, UserRepository};
use crate::error::Result;
use crate::infrastructure::database::DbPool;

use super::UserDTO;

pub struct Repository {
    db_pool: &'static DbPool,
}

impl Repository {
    pub fn new(db_pool: &'static DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl UserRepository for Repository {
    async fn create(&self, name: &str) -> Result<User> {
        let user: UserDTO = sqlx::query_as("INSERT INTO users (name) VALUES ($1) RETURNING *")
            .bind(name)
            .fetch_one(self.db_pool)
            .await?;

        Ok(user.into())
    }

    async fn find_one(&self, id: &Uuid) -> Result<User> {
        let user: UserDTO = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(self.db_pool)
            .await?;

        Ok(user.into())
    }

    async fn find_by_name(&self, name: &str) -> Result<User> {
        let user: UserDTO = sqlx::query_as("SELECT * FROM users WHERE name = $1")
            .bind(name)
            .fetch_one(self.db_pool)
            .await?;

        Ok(user.into())
    }
}
