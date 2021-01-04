use sqlx::FromRow;
use std::convert::TryInto;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

use crate::domain::file::{File, MimeType};
use crate::error::Error;

#[derive(FromRow)]
pub struct FileDTO {
    pub id: Uuid,
    pub filename: String,
    pub bytes: Vec<u8>,
    pub mime: String,
    pub size: i32,
    pub url: String,
    pub user_id: Uuid,
}

impl TryInto<File> for FileDTO {
    type Error = Error;

    fn try_into(self: FileDTO) -> Result<File, Self::Error> {
        let mime = MimeType::from_str(&self.mime)?;
        let url = Url::from_str(&self.url).map_err(Error::from)?;

        Ok(File {
            id: self.id,
            filename: self.filename,
            bytes: self.bytes,
            mime,
            size: self.size as usize,
            url,
        })
    }
}
