use futures::TryStreamExt;
use uuid::Uuid;
use warp::filters::multipart::{FormData, Part};
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::error::MSendError;
use crate::server::http_response::HttpResponse;
use crate::service::auth::Claims;
use crate::service::image::ImageResource;
use crate::service::InjectedServices;

pub async fn upload(
    claims: Claims,
    services: InjectedServices,
    form: FormData,
) -> Result<impl warp::Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|error| {
        eprintln!("Form Read Error: {}", error);
        warp::reject::reject()
    })?;

    if let Some(p) = parts.into_iter().find(|part| part.name() == "image") {
        let image = services.image_service.from_part(p).await;

        if let Err(e) = image {
            return Ok(e.into_http());
        }

        return match services
            .image_service
            .save(image.unwrap(), claims.user_id)
            .await
        {
            Ok(image) => Ok(HttpResponse::with_payload(
                ImageResource::from_image(image, claims.user_id),
                StatusCode::CREATED,
            )),
            Err(e) => Ok(e.into_http()),
        };
    }

    Err(warp::reject::reject())
}

pub async fn info(
    _: Claims,
    services: InjectedServices,
    id: Uuid,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    match services.image_service.get_info(id).await {
        Ok(info) => Ok(HttpResponse::with_payload(info, StatusCode::OK)),
        Err(e) => Ok(e.into_http()),
    }
}

pub async fn download(
    _: Claims,
    services: InjectedServices,
    url: String,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    match services.image_service.download(url).await {
        Ok(image) => Ok(HttpResponse::send_file(image.image, &image.mime)),
        Err(e) => Ok(e.into_response()),
    }
}
