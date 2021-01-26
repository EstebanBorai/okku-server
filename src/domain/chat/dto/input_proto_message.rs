use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InputProtoMessageDTO {
    pub author_id: Uuid,
    pub chat_id: Uuid,
    pub body: String,
    pub created_at: DateTime<Utc>,
}
