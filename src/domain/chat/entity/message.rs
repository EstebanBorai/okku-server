use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::chat::Chat;
use crate::domain::file::File;
use crate::domain::user::User;

use super::MessageKind;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: Uuid,
    pub chat: Chat,
    pub author: User,
    pub content: String,
    pub kind: MessageKind,
    pub file: Option<File>,
    pub created_at: DateTime<Utc>,
}
