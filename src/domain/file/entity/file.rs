use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::MimeType;

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub id: Uuid,
    pub filename: String,
    pub mime: MimeType,
    pub bytes: Vec<u8>,
    pub size: usize,
    pub url: Url,
}
