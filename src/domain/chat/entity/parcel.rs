use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Kind {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "ping")]
    Ping,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parcel {
    pub kind: Kind,
    pub data: Option<Vec<u8>>,
    pub client_id: Option<Uuid>,
}

impl Parcel {
    pub fn ping() -> Self {
        Self {
            kind: Kind::Ping,
            data: None,
            client_id: None,
        }
    }

    pub fn message(client_id: &Uuid, bytes: &[u8]) -> Self {
        Self {
            kind: Kind::Message,
            data: Some(bytes.to_vec()),
            client_id: Some(client_id.to_owned()),
        }
    }
}
