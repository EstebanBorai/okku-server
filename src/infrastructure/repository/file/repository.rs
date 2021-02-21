use async_trait::async_trait;
use futures::TryStreamExt;
use sqlx::Row;
use std::convert::TryInto;
use uuid::Uuid;

use crate::domain::avatar::FileServiceRepository;
use crate::domain::file::{File, FileRepository};
use crate::error::{Error, Result};
use crate::infrastructure::database::DbPool;

use super::FileDTO;

pub struct Repository {
    db_pool: &'static DbPool,
}

impl Repository {
    pub fn new(db_pool: &'static DbPool) -> Self {
        Self { db_pool }
    }
}

impl FileServiceRepository for Repository {}

#[async_trait]
impl FileRepository for Repository {
    async fn create<M: crate::domain::file::FileMimeType + ToString + Send + Sync>(
        &self,
        filename: &str,
        mime: &M,
        bytes: &[u8],
        size: usize,
        url: &str,
        user_id: &Uuid,
    ) -> Result<File> {
        let file: FileDTO = sqlx::query_as(
            r#"
            INSERT INTO files (
                filename,
                mime,
                bytes,
                size,
                url,
                user_id
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            ) RETURNING *
        "#,
        )
        .bind(filename)
        .bind(&mime.to_string())
        .bind(bytes)
        .bind(size as i32)
        .bind(url)
        .bind(user_id)
        .fetch_one(self.db_pool)
        .await
        .map_err(Error::from)?;

        Ok(file.try_into()?)
    }

    async fn find_by_filename(&self, filename: &str) -> Result<Vec<u8>> {
        let rows = sqlx::query("SELECT bytes FROM files WHERE filename = $1")
            .bind(filename)
            .fetch_optional(self.db_pool)
            .await?;

        match rows {
            Some(row) => Ok(row.try_get("bytes")?),
            None => Err(Error::FileNotFound(filename.to_string())),
        }
    }
}
