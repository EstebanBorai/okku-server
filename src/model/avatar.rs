use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Serialize)]
pub struct Avatar {
    pub id: Uuid,
    pub image: Vec<u8>,
    pub mime_type: String,
}
