use async_trait::async_trait;
use sqlx::postgres::Postgres;
use sqlx::Transaction;
use uuid::Uuid;

use crate::error::Result;

use super::Secret;

#[async_trait]
pub trait SecretRepository {
    async fn create_tx(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        user_id: &Uuid,
        hash: &str,
    ) -> Result<Secret>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Secret>;
}
