use async_trait::async_trait;

use crate::domain::profile::{Profile, ProfileRepository};
use crate::domain::user::User;
use crate::error::Result;
use crate::infrastructure::database::DbPool;

use super::ProfileDTO;

pub struct Repository {
    db_pool: &'static DbPool,
}

impl Repository {
    pub fn new(db_pool: &'static DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl ProfileRepository for Repository {
    async fn create(&self, user: &User, first_name: &str, email: &str) -> Result<Profile> {
        let dto: ProfileDTO = sqlx::query_as(
            "INSERT INTO profiles (user_id, first_name, email) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(user.id)
        .bind(first_name)
        .bind(email)
        .fetch_one(self.db_pool)
        .await?;

        Ok(ProfileDTO::into_profile(&dto, user, None))
    }

    async fn find_by_user(&self, user: &User) -> Result<Profile> {
        let dto: ProfileDTO =
            sqlx::query_as("SELECT * FROM profiles LEFT JOIN users ON users.name = $1")
                .bind(&user.name)
                .fetch_one(self.db_pool)
                .await?;

        Ok(ProfileDTO::into_profile(&dto, user, None))
    }
}
