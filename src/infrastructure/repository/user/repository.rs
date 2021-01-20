use async_trait::async_trait;
use sqlx::postgres::Postgres;
use sqlx::Transaction;
use uuid::Uuid;

use crate::domain::user::{User, UserRepository};
use crate::error::{Error, Result};
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repository::base::BaseRepository;

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
impl BaseRepository for Repository {
    async fn begin_tx(&self) -> Result<Transaction<'static, Postgres>> {
        self.db_pool.begin().await.map_err(Error::from)
    }
}

#[async_trait]
impl UserRepository for Repository {
    async fn create_tx<'a>(
        &'a self,
        tx: &mut Transaction<'static, Postgres>,
        name: &'a str,
    ) -> Result<User> {
        let user: UserDTO = sqlx::query_as("INSERT INTO users (name) VALUES ($1) RETURNING *")
            .bind(name)
            .fetch_one(tx)
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
