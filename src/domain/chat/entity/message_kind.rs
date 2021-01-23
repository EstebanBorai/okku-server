use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::string::ToString;

use crate::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageKind {
    Text,
}

impl ToString for MessageKind {
    fn to_string(&self) -> String {
        match self {
            MessageKind::Text => String::from("text"),
        }
    }
}

impl FromStr for MessageKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(MessageKind::Text),
            _ => Err(Error::ParseMessageKindError(format!(
                "Invalid kind provided: {}",
                s
            ))),
        }
    }
}
