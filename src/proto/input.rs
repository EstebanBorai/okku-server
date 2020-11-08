use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Input {
  #[serde(rename = "join")]
  Join(JoinInput),
  #[serde(rename = "post")]
  Post(PostInput),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinInput {
  pub name: String,
}

#[derive(Debug, Clone)]
pub struct InputParcel {
    pub client_id: Uuid,
    pub input: Input,
}

impl InputParcel {
    pub fn new(client_id: Uuid, input: Input) -> Self {
        InputParcel { client_id, input }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostInput {
    pub body: String,
}
