use futures::TryStreamExt;
use serde::Serialize;
use uuid::Uuid;
use warp::filters::multipart::{FormData, Part};
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::application::service::Services;
use crate::domain::auth::Claims;
use crate::server::utils::File;
use crate::server::utils::Response;

#[derive(Serialize)]
pub struct FileUploadResponse {
    id: Uuid,
    filename: String,
    mime: String,
    size: usize,
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

    if let Some(p) = parts.into_iter().find(|part| part.name() == "file") {
        let part_bytes = File::from_part(p).await.map_err(|e| {
            Response::message(e.message())
                .status_code(StatusCode::BAD_REQUEST)
                .reject()
        })?;
        let file = services
            .file_service
            .upload(part_bytes.bytes().as_slice(), &claims.user_id)
            .await
            .map_err(|e| {
                Response::message(e.message())
                    .status_code(StatusCode::BAD_REQUEST)
                    .reject()
            })?;

        return Ok(Response::new(FileUploadResponse {
            id: file.id,
            filename: file.filename,
            mime: file.mime.to_string(),
            size: file.size,
            url: file.url.to_string(),
        }));
    }

    Err(
        Response::message(String::from("The form field \"file\" is required"))
            .status_code(StatusCode::BAD_REQUEST)
            .reject(),
    )
}
