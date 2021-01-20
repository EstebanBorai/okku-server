use async_trait::async_trait;
use sqlx::postgres::Postgres;
use sqlx::Transaction;
use uuid::Uuid;

use crate::domain::secret::{Secret, SecretRepository};
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
}

#[async_trait]
impl SecretRepository for Repository {
    async fn create_tx(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        user_id: &Uuid,
        hash: &str,
    ) -> Result<Secret> {
        let secret: SecretDTO =
            sqlx::query_as("INSERT INTO secrets (hash, user_id) VALUES ($1, $2) RETURNING *")
                .bind(hash)
                .bind(user_id)
                .fetch_one(tx)
                .await?;

        Ok(secret.into())
    }

    // Uncomment this when Password Reset is available
    // pub async fn update(&self, hash: &str, user_id: &Uuid) -> Result<SecretDTO> {
    //     let secret: SecretDTO = sqlx::query_as(
    //         "UPDATE secrets SET hash = $1, user_id = $2 WHERE user_id = $2 RETURNING *",
    //     )
    //     .bind(hash)
    //     .bind(user_id)
    //     .fetch_one(self.db_pool)
    //     .await?;

    //     Ok(secret)
    // }

    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Secret> {
        let secret: SecretDTO = sqlx::query_as("SELECT * FROM secrets WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(self.db_pool)
            .await?;

        Ok(secret.into())
    }
}
