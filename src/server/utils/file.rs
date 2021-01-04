use bytes::BufMut;
use futures::TryStreamExt;
use std::string::ToString;
use warp::filters::multipart::Part;

use crate::error::{Error, Result};

pub struct File(Vec<u8>);

impl File {
    pub fn bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub async fn from_part(part: Part) -> Result<Self> {
        let stream = part.stream();

        match stream
            .try_fold(Vec::new(), |mut vec, data| {
                vec.put(data);
                async move { Ok(vec) }
            })
            .await
            .map_err(|e| Error::ReadFileError(e.to_string()))
        {
            Ok(bytes) => Ok(Self(bytes)),
            Err(e) => Err(Error::ReadFileError(e.to_string())),
        }
    }
}
