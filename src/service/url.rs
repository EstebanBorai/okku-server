use std::env;
use url::Url;

use crate::error::{AppError, Result};

#[derive(Clone)]
pub struct UrlService {
    host: String,
    port: u16,
}

impl UrlService {
    pub fn new() -> Self {
        let host = env::var("HOST").expect("Missing \"HOST\" environment variable");
        let port = env::var("PORT").expect("Missing \"PORT\" environment variable");
        let port = port
            .parse::<u16>()
            .expect("\"PORT\" environment variable must be a valid number");

        Self { host, port }
    }

    pub fn create_server_url(&self, uri: &str) -> Result<Url> {
        let server_url = format!(
            "http://{host}:{port}/{uri}",
            host = self.host,
            port = self.port,
            uri = uri
        );

        Url::parse(&server_url).map_err(AppError::from)
    }
}
