use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::chat::Chat;

#[derive(Debug, FromRow)]
pub struct ChatsUsersDTO {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ChatsUsersDTO {
    pub fn as_chat(dto: &ChatsUsersDTO) -> Chat {
        Chat { id: dto.chat_id }
    }
}
