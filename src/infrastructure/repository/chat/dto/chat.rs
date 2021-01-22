use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::chat::Chat;

#[derive(Debug, FromRow)]
pub struct ChatDTO {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ChatDTO {
    pub fn as_chat(dto: &ChatDTO) -> Chat {
        Chat { id: dto.id }
    }
}
