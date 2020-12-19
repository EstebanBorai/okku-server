use futures::TryStreamExt;
use std::string::ToString;
use uuid::Uuid;
use warp::filters::multipart::{FormData, Part};
use warp::reject::Rejection;

use crate::error::MSendError;
use crate::server::http_response::HttpResponse;
use crate::service::{Claims, InjectedServices};

pub async fn upload_avatar(
    _: Claims,
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
        let mime_type = services
            .image_service
            .get_mime_type(content_type.unwrap())
            .unwrap();
        let file_bytes = services.image_service.part_bytes(p).await.unwrap();

        return match services
            .user_service
            .set_avatar(&uid, mime_type, file_bytes)
            .await
        {
            Ok(avatar) => Ok(HttpResponse::<Vec<u8>>::send_file(
                avatar.image,
                &avatar.mime_type.to_string(),
            )),
            Err(e) => Ok(e.into_response()),
        };
    }

    Err(warp::reject::reject())
}

pub async fn download_avatar(
    _: Claims,
    services: InjectedServices,
    uid: Uuid,
) -> Result<impl warp::Reply, Rejection> {
    match services.user_service.download_avatar(&uid).await {
        Ok(avatar) => Ok(HttpResponse::<Vec<u8>>::send_file(
            avatar.image,
            &avatar.mime_type.to_string(),
        )),
        Err(e) => Ok(e.into_response()),
    }
}
