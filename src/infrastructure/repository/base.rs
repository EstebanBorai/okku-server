use async_trait::async_trait;
use sqlx::postgres::Postgres;
use sqlx::Transaction;

use crate::error::Result;

#[async_trait]
pub trait BaseRepository {
    async fn begin_tx(&self) -> Result<Transaction<'static, Postgres>>;
}
