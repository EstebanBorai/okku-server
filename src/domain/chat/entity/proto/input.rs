use serde::{Deserialize, Serialize};

use crate::domain::chat::InputProtoMessageDTO;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", content = "inner")]
pub enum Input {
    #[serde(rename = "message")]
    Message(IncomingMessage),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncomingMessage {
    pub message: InputProtoMessageDTO,
}
