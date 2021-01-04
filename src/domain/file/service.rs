use rand::{distributions::Alphanumeric, Rng};
use std::env;
use std::str::FromStr;
use tree_magic::from_u8 as mime_from_bytes;
use url::Url;
use uuid::Uuid;

use crate::error::{Error, Result};

use super::{File, FileMimeType, FileRepository, MimeType};

/// Max size for image file. 1 GB in bytes.
const MAX_IMAGE_SIZE: usize = 1_000_000_000;

pub struct FileService<R>
where
    R: FileRepository,
{
    file_repository: R,
    server_port: u16,
    server_host: String,
}

impl<R> FileService<R>
where
    R: FileRepository,
{
    pub fn new(file_repository: R) -> Self {
        let host = env::var("HOST").expect("Missing \"HOST\" environment variable");
        let port = env::var("PORT").expect("Missing \"PORT\" environment variable");
        let port = port
            .parse::<u16>()
            .expect("\"PORT\" environment variable must be a valid number");

        Self {
            file_repository,
            server_host: host,
            server_port: port,
        }
    }

    pub async fn upload(&self, bytes: &[u8], user_id: &Uuid) -> Result<File> {
        if bytes.len() > MAX_IMAGE_SIZE {
            return Err(Error::FileTooLarge(bytes.len()));
        }

        let mime = self.get_mime_type(bytes)?;
        let filename = self.make_file_name(&mime);
        let url = self.make_file_url(&filename)?;
        let url = url.to_string();

        self.file_repository
            .create(&filename, &mime, bytes, bytes.len(), &url, user_id)
            .await
    }

    pub async fn download(&self, filename: &str) -> Result<Vec<u8>> {
        self.file_repository.find_by_filename(filename).await
    }

    fn make_file_name(&self, mime: &MimeType) -> String {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        format!("{}.{}", filename, mime.get_ext())
    }

    fn make_file_url(&self, filename: &str) -> Result<Url> {
        let server_url = format!(
            "http://{host}:{port}/api/v1/files/{filename}",
            host = self.server_host,
            port = self.server_port,
            filename = filename,
        );

        Url::parse(&server_url).map_err(Error::from)
    }

    fn get_mime_type(&self, bytes: &[u8]) -> Result<MimeType> {
        let mime = mime_from_bytes(bytes);
        let mime = MimeType::from_str(&mime)?;

        Ok(mime)
    }
}
