use async_trait::async_trait;
use std::string::ToString;
use uuid::Uuid;

use crate::error::Result;

use super::{File, FileMimeType};

#[async_trait]
pub trait FileRepository {
    async fn create<M: FileMimeType + ToString + Send + Sync>(
        &self,
        filename: &str,
        mime: &M,
        bytes: &[u8],
        size: usize,
        url: &str,
        user_id: &Uuid,
    ) -> Result<File>;
    async fn find_by_filename(&self, filename: &str) -> Result<Vec<u8>>;
}
