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
    pub image: Vec<u8>,
}
