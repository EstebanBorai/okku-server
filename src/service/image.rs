use bytes::BufMut;
use futures::TryStreamExt;
use std::string::ToString;
use warp::filters::multipart::Part;

use crate::error::AppError;
use crate::model::AvatarMIMEType;

#[derive(Clone)]
pub struct ImageService;

impl ImageService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_mime_type(&self, content_type: &str) -> Result<AvatarMIMEType, AppError> {
        match content_type {
            "image/png" => Ok(AvatarMIMEType::Png),
            "image/jpeg" => Ok(AvatarMIMEType::Jpeg),
            _ => Err(AppError::UnsupportedImage(content_type.to_string())),
        }
    }

    pub async fn part_bytes(&self, part: Part) -> Result<Vec<u8>, AppError> {
        let stream = part.stream();

        match stream
            .try_fold(Vec::new(), |mut vec, data| {
                vec.put(data);
                async move { Ok(vec) }
            })
            .await
            .map_err(|e| Err(AppError::ReadImageError(e.to_string())))
        {
            Ok(bytes) => Ok(bytes),
            Err(error) => Err(error.unwrap()),
        }
    }
}
