use async_trait::async_trait;

use crate::domain::avatar::{Avatar, AvatarRepository};
use crate::domain::file::File;
use crate::error::Result;
use crate::infrastructure::database::DbPool;

use super::AvatarDTO;

pub struct Repository {
    db_pool: &'static DbPool,
}

impl Repository {
    pub fn new(db_pool: &'static DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AvatarRepository for Repository {
    async fn create(&self, file: File) -> Result<Avatar> {
        let avatar: AvatarDTO =
            sqlx::query_as("INSERT INTO avatars (file_id) VALUES ($1) RETURNING *")
                .bind(&file.id)
                .fetch_one(self.db_pool)
                .await?;

        Ok(AvatarDTO::into_avatar(&avatar, file))
    }
}
