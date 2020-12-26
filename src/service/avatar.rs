use std::sync::Arc;
use uuid::Uuid;
use warp::filters::multipart::Part;

use crate::database::DbPool;
use crate::error::{AppError, Result};
use crate::model::avatar::Avatar;

use super::image::ImageService;

#[derive(Clone)]
pub struct AvatarService {
    db_pool: DbPool,
    image_service: Arc<ImageService>,
}

impl AvatarService {
    pub fn new(db_pool: DbPool, image_service: Arc<ImageService>) -> Self {
        Self {
            db_pool,
            image_service,
        }
    }

    pub async fn from_part(&self, p: Part) -> Result<Avatar> {
        let image = self.image_service.from_part(p).await?;

        if image.height < 600 || image.width < 600 {
            return Err(AppError::InvalidAvatarImageSize(image.width as i32, image.height as i32));
        }

        let large = self.image_service.resize_image(&image, 48, 48).await?;
        let medium = self.image_service.resize_image(&image, 32, 32).await?;
        let normal = self.image_service.resize_image(&image, 24, 24).await?;
        let small = self.image_service.resize_image(&image, 16, 16).await?;
        let retina_1x = self.image_service.resize_image(&image, 600, 600).await?;

        Ok(Avatar {
            id: Uuid::default(),
            large,
            medium,
            normal,
            small,
            retina_1x,
        })
    }
}
