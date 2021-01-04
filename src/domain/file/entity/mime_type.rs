use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::string::ToString;

use crate::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub enum MimeType {
    GIF,
    JPEG,
}

pub trait FileMimeType {
    fn get_ext(&self) -> String;
}

impl FileMimeType for MimeType {
    fn get_ext(&self) -> String {
        match self {
            Self::GIF => String::from("gif"),
            Self::JPEG => String::from("jpeg"),
        }
    }
}

impl FromStr for MimeType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mime = match s {
            "image/gif" => Self::GIF,
            "image/jpeg" => Self::JPEG,
            _ => return Err(Error::UnrecognizedMIME(s.to_string())),
        };

        Ok(mime)
    }
}

impl ToString for MimeType {
    fn to_string(&self) -> String {
        match self {
            Self::GIF => String::from("image/gif"),
            Self::JPEG => String::from("image/jpeg"),
        }
    }
}
