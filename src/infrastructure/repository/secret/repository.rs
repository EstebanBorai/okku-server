use uuid::Uuid;

use crate::error::Result;
use crate::infrastructure::database::DbPool;

use super::SecretDTO;

pub struct Repository {
    db_pool: &'static DbPool,
}

impl Repository {
    pub fn new(db_pool: &'static DbPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, hash: &str, user_id: &Uuid) -> Result<SecretDTO> {
        let secret: SecretDTO =
            sqlx::query_as("INSERT INTO secrets (hash, user_id) VALUES ($1, $2) RETURNING *")
                .bind(hash)
                .bind(user_id)
                .fetch_one(self.db_pool)
                .await?;

        Ok(secret)
    }

    pub async fn update(&self, hash: &str, user_id: &Uuid) -> Result<SecretDTO> {
        let secret: SecretDTO = sqlx::query_as(
            "UPDATE secrets SET hash = $1, user_id = $2 WHERE user_id = $2 RETURNING *",
        )
        .bind(hash)
        .bind(user_id)
        .fetch_one(self.db_pool)
        .await?;

        Ok(secret)
    }

    pub async fn find_by_user_id(&self, user_id: &Uuid) -> Result<SecretDTO> {
        let secret: SecretDTO = sqlx::query_as("SELECT * FROM secrets WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(self.db_pool)
            .await?;

        Ok(secret)
    }
}
