use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct MessageDTO {
    pub id: Uuid,
    pub content: String,
    pub kind: String,
    pub author_id: Uuid,
    pub chat_id: Uuid,
    pub file_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
