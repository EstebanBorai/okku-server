use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::{User, UserRepository};
use crate::domain::profile::ProfileRepository;
use crate::error::Result;
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repository::profile::{Repository as ProfileRepositoryInstance};

use super::UserDTO;

pub struct Repository {
    db_pool: &'static DbPool,
    profile_repository: ProfileRepositoryInstance,
}

impl Repository {
    pub fn new(db_pool: &'static DbPool, profile_repository: ProfileRepositoryInstance) -> Self {
        Self { db_pool, profile_repository }
    }
}

#[async_trait]
impl UserRepository for Repository {
    async fn create(&self, name: &str) -> Result<User> {
        let mut tx = self.db_pool.begin().await?;

        let user: UserDTO = sqlx::query_as("INSERT INTO users (name) VALUES ($1) RETURNING *")
            .bind(name)
            .fetch_one(&mut tx)
            .await?;

        let user: User = user.into();

        sqlx::query("INSERT INTO profiles (user_id) VALUES ($1) RETURNING *")
            .bind(user.id)
            .fetch_one(&mut tx)
            .await?;

        // Move query to profile repository if results
        // works as expected.
        // Shared mutability is required for this structure
        //
        // self.profile_repository.create(&user).await?;
        //

        tx.commit().await?;

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
