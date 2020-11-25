use crate::database::get_db_conn;
use crate::server::http_response::HttpResponse;
use anyhow::{Error, Result as AnyhowResult};
use bytes::BufMut;
use futures::TryStreamExt;
use std::string::ToString;
use uuid::Uuid;
use warp::filters::multipart::{FormData, Part};
use warp::http::StatusCode;
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

pub async fn replace_avatar(uid: Uuid, form: FormData) -> Result<impl warp::Reply, Rejection> {
    let db_conn = get_db_conn().await.unwrap();

    if db_conn
        .query_one("SELECT COUNT(1) FROM avatars WHERE user_id = $1", &[&uid])
        .await
        .is_err()
    {
        return Ok(HttpResponse::<String>::new(
            "No avatar found for user",
            StatusCode::BAD_REQUEST,
        ));
    }

    let parts: Vec<Part> = form.try_collect().await.map_err(|error| {
        eprintln!("Form Read Error: {}", error);
        warp::reject::reject()
    })?;

    if let Some(p) = parts.into_iter().find(|part| part.name() == "image") {
        let content_type = p.content_type();
        let mime_type = get_mime_type(content_type.unwrap()).unwrap();
        let file_bytes = part_bytes(p).await.unwrap();

        db_conn
            .query_one(
                r#"
                UPDATE avatars
                SET
                    image = $1,
                    mime_type = $2
                WHERE
                    user_id = $3
                RETURNING *"#,
                &[&file_bytes.as_slice(), &mime_type.to_string(), &uid],
            )
            .await
            .unwrap();

        return Ok(HttpResponse::<String>::new(
            "Avatar updated successfully",
            StatusCode::OK,
        ));
    }

    Err(warp::reject::reject())
}

pub async fn download_avatar(uid: Uuid) -> Result<impl warp::Reply, Rejection> {
    let dbconn = get_db_conn().await.unwrap();
    let results = dbconn
        .query_one(
            "SELECT image, mime_type FROM avatars WHERE user_id = $1",
            &[&uid],
        )
        .await
        .unwrap();

    println!("{:?}", results);

    let image_bytes: &[u8] = results.get(0);

    println!("{:?}", image_bytes);

    Ok(image_bytes.clone().to_owned())
}

fn get_mime_type(content_type: &str) -> AnyhowResult<AvatarMIMEType> {
    match content_type {
        "image/png" => Ok(AvatarMIMEType::Png),
        "image/jpeg" => Ok(AvatarMIMEType::Jpeg),
        _ => Err(Error::msg(format!(
            "Unsupported Content-Type for \"image\": {}",
            content_type
        ))),
    }
}

async fn part_bytes(part: Part) -> AnyhowResult<Vec<u8>> {
    let stream = part.stream();

    match stream
        .try_fold(Vec::new(), |mut vec, data| {
            vec.put(data);
            async move { Ok(vec) }
        })
        .await
        .map_err(|error| Err(Error::msg(error.to_string())))
    {
        Ok(bytes) => Ok(bytes),
        Err(error) => Err(error.unwrap()),
    }
}
