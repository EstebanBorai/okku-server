use crate::model::AvatarMIMEType;
use crate::server::http_response::HttpResponse;
use crate::service::InjectedServices;
use anyhow::{Error, Result as AnyhowResult};
use bytes::BufMut;
use futures::TryStreamExt;
use std::string::ToString;
use uuid::Uuid;
use warp::filters::multipart::{FormData, Part};
use warp::http::StatusCode;
use warp::reject::Rejection;

pub async fn upload_avatar(
    services: InjectedServices,
    uid: Uuid,
    form: FormData,
) -> Result<impl warp::Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|error| {
        eprintln!("Form Read Error: {}", error);
        warp::reject::reject()
    })?;

    if let Some(p) = parts.into_iter().find(|part| part.name() == "image") {
        let content_type = p.content_type();
        let mime_type = get_mime_type(content_type.unwrap()).unwrap();
        let file_bytes = part_bytes(p).await.unwrap();

        return match services
            .user_service
            .set_avatar(&uid, mime_type, file_bytes)
            .await
        {
            Ok(avatar) => Ok(HttpResponse::<Vec<u8>>::send_file(
                avatar.image,
                &avatar.mime_type.to_string(),
            )),
            Err(error) => Ok(HttpResponse::<String>::new(
                error.to_string().as_str(),
                StatusCode::BAD_REQUEST,
            )
            .into()),
        };
    }

    Err(warp::reject::reject())
}

pub async fn download_avatar(
    services: InjectedServices,
    uid: Uuid,
) -> Result<impl warp::Reply, Rejection> {
    match services.user_service.download_avatar(&uid).await {
        Ok(avatar) => Ok(HttpResponse::<Vec<u8>>::send_file(
            avatar.image,
            &avatar.mime_type.to_string(),
        )),
        Err(error) => Ok(HttpResponse::<String>::new(
            error.to_string().as_str(),
            StatusCode::BAD_REQUEST,
        )
        .into()),
    }
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
