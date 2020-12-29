use async_trait::async_trait;

use crate::domain::user::{User, UserRepository};
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
    async fn create(&self, name: &str) -> crate::error::Result<crate::domain::user::User> {
        let user: UserDTO = sqlx::query_as("INSERT INTO users (name) VALUES ($1) RETURNING *")
            .bind(name)
            .fetch_one(self.db_pool)
            .await?;

        Ok(user.into())
    }

    async fn find_by_name(&self, name: &str) -> crate::error::Result<crate::domain::user::User> {
        let user: UserDTO = sqlx::query_as("SELECT * FROM users WHERE name = $1")
            .bind(name)
            .fetch_one(self.db_pool)
            .await?;

        Ok(user.into())
    }

    async fn find_all(&self) -> crate::error::Result<Vec<crate::domain::user::User>> {
        let users: Vec<UserDTO> = sqlx::query_as("SELECT * FROM users")
            .fetch_all(self.db_pool)
            .await?;

        let users = users
            .into_iter()
            .map(|u| {
                let user: User = u.into();

                user
            })
            .collect::<Vec<User>>();

        Ok(users)
    }
}
