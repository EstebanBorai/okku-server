use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, FromRow, Serialize)]
pub struct Image {
    pub id: Uuid,
    pub height: i16,
    pub width: i16,
    pub mime: String,
    pub url: String,
    pub filename: String,
    pub size: i32,
    pub bytes: Vec<u8>,
}

#[derive(Clone, FromRow, Serialize)]
pub struct ImageResource {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub height: i16,
    pub width: i16,
    pub mime: String,
    pub filename: String,
    pub url: String,
    pub size: i32,
}

impl ImageResource {
    pub fn from_image(img: Image, owner_id: Uuid) -> Self {
        Self {
            owner_id,
            id: img.id,
            height: img.height,
            width: img.width,
            mime: img.mime,
            filename: img.filename,
            url: img.url,
            size: img.size,
        }
    }
}
