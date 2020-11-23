use crate::server::http_response::HttpResponse;
use crate::database::{DbConn, Row, get_db_conn};
use anyhow::{Result as AnyhowResult, Error};
use base64::{encode_config, Config};
use bytes::BufMut;
use futures::TryStreamExt;
use std::string::ToString;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::filters::multipart::{FormData, Part};
use warp::reject::Rejection;

enum AvatarMIMEType {
    Png,
    Jpeg,
}

impl ToString for AvatarMIMEType {
    fn to_string(&self) -> String {
        match self {
            AvatarMIMEType::Png => String::from("image/png"),
            AvatarMIMEType::Jpeg => String::from("image/jpeg"),
        }
    }
}

pub async fn upload_avatar(uid: Uuid, form: FormData) -> Result<impl warp::Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|error| {
        eprintln!("Form Read Error: {}", error);
        warp::reject::reject()
    })?;

    if let Some(p) = parts.into_iter().find(|part| part.name() == "image") {
        let content_type = p.content_type();
        let mime_type = get_mime_type(content_type.unwrap()).unwrap();
        let file_bytes = part_bytes(p).await.unwrap();
        let db_conn = get_db_conn().await.unwrap();

        db_conn
            .query_one(
                r#"
                INSERT INTO avatars (image, user_id, mime_type)
                    VALUES($1, $2, $3)
                    RETURNING
                    *"#,
                &[&file_bytes.as_slice(), &uid, &mime_type.to_string()],
            )
            .await
            .unwrap();

        return Ok(HttpResponse::<String>::new(
            &format!("Hello {} with {}", uid, mime_type.to_string()),
            StatusCode::CREATED,
        ));
    }

    Err(warp::reject::reject())
}

fn get_mime_type(content_type: &str) -> AnyhowResult<AvatarMIMEType> {
    match content_type {
        "image/png" => Ok(AvatarMIMEType::Png),
        "image/jpeg" => Ok(AvatarMIMEType::Jpeg),
        _ => Err(Error::msg(format!("Unsupported Content-Type for \"image\": {}", content_type)))
    }
}

async fn part_bytes(part: Part) -> AnyhowResult<Vec<u8>> {
    let stream = part.stream();

    match stream.try_fold(Vec::new(), |mut vec, data| {
        vec.put(data);
        async move { Ok(vec) }
    })
    .await
    .map_err(|error| {
        Err(Error::msg(error.to_string()))
    }) {
        Ok(bytes) => Ok(bytes),
        Err(error) => Err(error.unwrap())
    }
}
