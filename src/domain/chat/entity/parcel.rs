use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::error::{Error, Result};

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
    pub recipient_id: Option<Uuid>,
    pub sender_id: Option<Uuid>,
}

impl Parcel {
    pub fn ping() -> Self {
        Self {
            kind: Kind::Ping,
            data: Some(Parcel::unix_now().unwrap().to_string().as_bytes().to_vec()),
            recipient_id: None,
            sender_id: None,
        }
    }

    pub fn unix_now() -> Result<u128> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(Error::from)?;

        Ok(now.as_millis())
    }
}
