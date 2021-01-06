use async_trait::async_trait;

use crate::domain::file::File;
use crate::error::Result;

use super::Avatar;

#[async_trait]
pub trait AvatarRepository {
    async fn create(&self, file: File) -> Result<Avatar>;
}
