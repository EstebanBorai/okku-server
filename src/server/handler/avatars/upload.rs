use futures::TryStreamExt;
use serde::Serialize;
use uuid::Uuid;
use warp::filters::multipart::{FormData, Part};
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::application::service::Claims;
use crate::application::service::Services;
use crate::server::utils::File;
use crate::server::utils::Response;

#[derive(Serialize)]
pub struct AvatarUploadResponse {
    id: Uuid,
    url: String,
}

pub async fn upload(
    claims: Claims,
    services: Services,
    form: FormData,
) -> Result<impl warp::Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        Response::message(format!("Unable to read form, {}", e.to_string()))
            .status_code(StatusCode::BAD_REQUEST)
            .reject()
    })?;

    if let Some(p) = parts.into_iter().find(|part| part.name() == "avatar") {
        let part_bytes = File::from_part(p).await.map_err(|e| {
            Response::message(e.message())
                .status_code(StatusCode::BAD_REQUEST)
                .reject()
        })?;

        let avatar = services
            .avatar_service
            .create(part_bytes.bytes().as_slice(), &claims.user_id)
            .await
            .map_err(|e| {
                Response::message(e.message())
                    .status_code(StatusCode::BAD_REQUEST)
                    .reject()
            })?;

        return Ok(Response::new(AvatarUploadResponse {
            id: avatar.id,
            url: avatar.file.url.to_string(),
        }));
    }

    Err(
        Response::message(String::from("The form field \"avatar\" is required"))
            .status_code(StatusCode::BAD_REQUEST)
            .reject(),
    )
}
