use futures::TryStreamExt;
use uuid::Uuid;
use warp::filters::multipart::{FormData, Part};
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::error::MSendError;
use crate::server::http_response::HttpResponse;
use crate::service::auth::Claims;
use crate::service::InjectedServices;

pub async fn upload_avatar(
    claims: Claims,
    services: InjectedServices,
    uid: Uuid,
    form: FormData,
) -> Result<impl warp::Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|error| {
        eprintln!("Form Read Error: {}", error);
        warp::reject::reject()
    })?;

    
    if let Some(p) = parts.into_iter().find(|part| part.name() == "image") {
        let image = services.avatar_service.from_part(p).await.unwrap();
        let variations = services.avatar_service.make_variations(&image).unwrap();

        return match services.avatar_service.save(claims.user_id, &variations).await {
            Ok(avatar) => Ok(HttpResponse::with_payload(avatar, StatusCode::OK)),
            Err(e) => Ok(e.into_http()),
        }
    }

    Err(warp::reject::reject())
}

pub async fn download_avatar(
    claims: Claims,
    services: InjectedServices,
    uid: Uuid,
) -> Result<impl warp::Reply, Rejection> {
    match services.avatar_service.find(claims.user_id).await {
        Ok(avatar) =>  Ok(HttpResponse::with_payload(avatar, StatusCode::OK)),
        Err(e) => Ok(e.into_http()),
    }
}
