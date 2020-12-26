use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use super::image::ImageResource;

#[derive(FromRow, Serialize)]
pub struct Avatar {
    pub id: Uuid,
    pub large: ImageResource,
    pub medium: ImageResource,
    pub normal: ImageResource,
    pub small: ImageResource,
    pub retina_1x: ImageResource,
}
