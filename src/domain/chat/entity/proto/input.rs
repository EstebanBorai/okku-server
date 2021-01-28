use serde::{Deserialize, Serialize};

use crate::domain::chat::InputProtoMessageDTO;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Input(pub InputProtoMessageDTO);
