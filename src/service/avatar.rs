use futures::TryStreamExt;
use std::sync::Arc;
use sqlx::Row;
use uuid::Uuid;
use warp::filters::multipart::Part;

use crate::database::DbPool;
use crate::error::{AppError, Result};
use crate::model::{
    avatar::Avatar,
    image::Image,
};

use super::image::ImageService;

#[derive(Clone)]
pub struct AvatarVariations {
    pub large: Image,
    pub medium: Image,
    pub normal: Image,
    pub small: Image,
    pub retina_1x: Image,
}

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

    pub async fn from_part(&self, p: Part) -> Result<Image> {
        let image = self.image_service.from_part(p).await?;

        if image.height < 600 || image.width < 600 {
            return Err(AppError::InvalidAvatarImageSize(image.width as i32, image.height as i32));
        }

        Ok(image)
    }

    pub fn make_variations(&self, image: &Image) -> Result<AvatarVariations> {
        let large = self.image_service.resize_image(image, 48, 48)?;
        let medium = self.image_service.resize_image(image, 32, 32)?;
        let normal = self.image_service.resize_image(image, 24, 24)?;
        let small = self.image_service.resize_image(image, 16, 16)?;
        let retina_1x = self.image_service.resize_image(image, 600, 600)?;

        Ok(AvatarVariations {
            large,
            medium,
            normal,
            small,
            retina_1x,
        })
    }

    /// THIS NEEDS A REFACTOR ASAP
    pub async fn save(&self, owner_id: Uuid, variations: &AvatarVariations) -> Result<Avatar> {
        // as we want to make sure all of these images are inserted as expected
        // we would have to implement a transaction instead of storing each image
        // separately
        let large = self.image_service.save(&variations.large, owner_id).await?;
        let medium = self.image_service.save(&variations.medium, owner_id).await?;
        let normal = self.image_service.save(&variations.normal, owner_id).await?;
        let small = self.image_service.save(&variations.small, owner_id).await?;
        let retina_1x = self.image_service.save(&variations.retina_1x, owner_id).await?;

        let mut rows = sqlx::query(
            r#"
        INSERT INTO avatars (
            user_id,
            large_id,
            medium_id,&
            normal_id,
            small_id,
            retina_1x_id
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6
        ) RETURNING *"#,
        )
        .bind(&owner_id)
        .bind(&large.id)
        .bind(&medium.id)
        .bind(&normal.id)
        .bind(&small.id)
        .bind(&retina_1x.id)
        .fetch(&self.db_pool);

        if let Some(row) = rows.try_next().await? {
            let id = row.try_get("id")?;
            let large_id: Uuid= row.try_get("large_id")?;
            let medium_id: Uuid = row.try_get("medium_id")?;
            let normal_id: Uuid = row.try_get("normal_id")?;
            let small_id: Uuid = row.try_get("small_id")?;
            let retina_1x_id: Uuid = row.try_get("retina_1x_id")?;

            let large = self.image_service.get_info(large_id).await?;
            let medium = self.image_service.get_info(medium_id).await?;
            let normal = self.image_service.get_info(normal_id).await?;
            let small = self.image_service.get_info(small_id).await?;
            let retina_1x = self.image_service.get_info(retina_1x_id).await?;

            return Ok(Avatar {
                id,
                large,
                medium,
                normal,
                small,
                retina_1x,
            });
        }

        Err(AppError::UnexpectedServerError(String::from("Unable to store avatar into the database")))
    }
}
