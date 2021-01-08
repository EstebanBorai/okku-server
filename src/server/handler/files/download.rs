use serde::Serialize;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::application::service::Claims;
use crate::application::service::Services;
use crate::server::utils::Response;

#[derive(Serialize)]
pub struct FileUploadResponse {
    id: Uuid,
    filename: String,
    mime: String,
    size: usize,
    url: String,
}

pub async fn download(
    _: Claims,
    services: Services,
    filename: String,
) -> Result<impl warp::Reply, Rejection> {
    let bytes = services
        .file_service
        .download(&filename)
        .await
        .map_err(|e| {
            Response::message(e.message())
                .status_code(StatusCode::BAD_REQUEST)
                .reject()
        })?;

    Ok(Response::send_file(bytes, "image/jpeg"))
}
