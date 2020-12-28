use bytes::BufMut;
use futures::TryStreamExt;
use image::{load_from_memory, GenericImageView};
use image::imageops::FilterType;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::string::ToString;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use warp::filters::multipart::Part;

use crate::database::DbPool;
use crate::error::{AppError, Result};
use crate::model::image::{Image, ImageResource};

use super::url::UrlService;

#[derive(Clone)]
pub struct ImageService {
    db_conn: DbPool,
    url_service: Arc<UrlService>,
}

impl ImageService {
    pub fn new(db_conn: DbPool, url_service: Arc<UrlService>) -> Self {
        Self {
            db_conn,
            url_service,
        }
    }

    pub async fn from_part(&self, p: Part) -> Result<Image> {
        let mime = self.get_content_type(&p);
        let bytes = self.part_bytes(p).await?;
        let bytes = bytes.clone();
        let size: i32 = bytes.len() as i32;
        let img = load_from_memory(&bytes)?;
        let (height, width) = img.dimensions();
        let filename = self.make_filename(size, &mime)?;
        let url = self
            .url_service
            .create_server_url(&format!("api/v1/images/{}", filename))?
            .to_string();

        Ok(Image {
            id: uuid::Uuid::default(),
            url,
            filename: String::from(filename),
            bytes,
            size,
            mime,
            height: height as i16,
            width: width as i16,
        })
    }

    pub async fn save(&self, image: &Image, owner_id: Uuid) -> Result<Image> {
        sqlx::query_as(
            r#"
        INSERT INTO images (
            height,
            width,
            mime,
            filename,
            url,
            size,
            bytes,
            owner_id
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8
        ) RETURNING *"#,
        )
        .bind(&image.height)
        .bind(&image.width)
        .bind(&image.mime)
        .bind(&image.filename)
        .bind(&image.url)
        .bind(&image.size)
        .bind(&image.bytes.as_slice())
        .bind(&owner_id)
        .fetch_one(&self.db_conn)
        .await
        .map_err(AppError::from)
    }

    pub async fn download(&self, url: String) -> Result<Image> {
        sqlx::query_as("SELECT * FROM images WHERE filename = $1")
            .bind(&url)
            .fetch_one(&self.db_conn)
            .await
            .map_err(AppError::from)
    }

    pub async fn get_info(&self, id: Uuid) -> Result<ImageResource> {
        sqlx::query_as(
            r#"
        SELECT
            id,
            height,
            width,
            mime,
            url,
            filename,
            size,
            owner_id
        FROM
            images
        WHERE
            id = $1"#,
        )
        .bind(&id)
        .fetch_one(&self.db_conn)
        .await
        .map_err(AppError::from)
    }

    pub fn get_content_type<'a>(&self, p: &'a Part) -> String {
        let content_type = p.content_type();
        let content_type = content_type.as_ref().unwrap();

        content_type.to_string()
    }

    pub async fn part_bytes(&self, part: Part) -> Result<Vec<u8>> {
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

    pub fn resize_image(&self, image: &Image, height: u32, width: u32) -> Result<Image> {
        let dynamic = load_from_memory(image.bytes.as_slice())?;
        let dynamic = dynamic.resize(width, height, FilterType::CatmullRom);
        let bytes = dynamic.as_bytes();
        let size = bytes.len() as i32;
        let filename = self.make_filename(size, &image.mime)?;
        let url = self
            .url_service
            .create_server_url(&format!("api/v1/images/{}", filename))?
            .to_string();

        Ok(Image {
            id: uuid::Uuid::default(),
            url,
            filename: String::from(filename),
            bytes: bytes.to_vec(),
            size,
            mime: image.mime.clone(),
            height: height as i16,
            width: width as i16,
        })
    }

    fn make_filename(&self, size: i32, mime: &str) -> Result<String> {
        let mut state = DefaultHasher::new();
        let file_extension = self.extension_from_mime(mime)?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let temporal_name = format!(
            "{}_{}_{}_{}",
            Uuid::new_v4().to_string(),
            size,
            file_extension,
            timestamp
        );

        temporal_name.hash(&mut state);

        Ok(format!("{}.{}", state.finish(), file_extension))
    }

    fn extension_from_mime(&self, mime: &str) -> Result<String> {
        match mime {
            "image/jpeg" => Ok(String::from("jpeg")),
            "image/png" => Ok(String::from("png")),
            _ => Err(AppError::UnsupportedImage(format!(
                "MIME type {} is not supported",
                mime
            ))),
        }
    }
}
