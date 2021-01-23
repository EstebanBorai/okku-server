use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingMessageDTO {
    pub chat_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub kind: String,
    pub file_id: Option<Uuid>,
}
