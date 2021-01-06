use image::{load_from_memory, GenericImageView};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::file::{FileRepository, FileService};
use crate::error::{Error, Result};

use super::{Avatar, AvatarRepository};

pub trait FileServiceRepository: FileRepository + Sized {}

pub struct AvatarService<R, F>
where
    R: AvatarRepository,
    F: FileServiceRepository,
{
    avatar_repository: R,
    file_service: Arc<FileService<F>>,
}

impl<R, F> AvatarService<R, F>
where
    R: AvatarRepository,
    F: FileServiceRepository,
{
    pub fn new(avatar_repository: R, file_service: Arc<FileService<F>>) -> Self {
        Self {
            avatar_repository,
            file_service,
        }
    }

    pub async fn create(&self, bytes: &[u8], user_id: &Uuid) -> Result<Avatar> {
        let img = load_from_memory(&bytes)?;
        let (height, width) = img.dimensions();

        if height < 300 || width < 300 {
            return Err(Error::AvatarImageIsTooSmall(height, width));
        }

        if height != width {
            return Err(Error::AvatarImageIsNot1_1(height, width));
        }

        let file = self.file_service.upload(bytes, user_id).await?;
        let avatar = self.avatar_repository.create(file).await?;

        Ok(avatar)
    }
}
