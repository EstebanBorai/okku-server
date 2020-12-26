use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use super::image::Image;

#[derive(FromRow, Serialize)]
pub struct Avatar {
    pub id: Uuid,
    pub large: Image,
    pub medium: Image,
    pub normal: Image,
    pub small: Image,
    pub retina_1x: Image,
}
