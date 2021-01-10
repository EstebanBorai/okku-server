use serde::Deserialize;
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::application::service::Claims;
use crate::application::service::Services;
use crate::server::utils::Response;

#[derive(Deserialize)]
pub struct CreateProfileRequest {
    first_name: String,
    email: String,
}

pub async fn upgrade(
    claims: Claims,
    body: CreateProfileRequest,
    services: Services,
) -> Result<impl warp::Reply, Rejection> {
    let user = services
        .user_service
        .find_one(&claims.user_id)
        .await
        .map_err(|e| {
            Response::message(e.message())
                .status_code(StatusCode::BAD_REQUEST)
                .reject()
        })?;

    let profile = services
        .profile_service
        .create(&user, &body.first_name, &body.email)
        .await
        .map_err(|e| {
            Response::message(e.message())
                .status_code(StatusCode::BAD_REQUEST)
                .reject()
        })?;

    Ok(Response::new(profile).status_code(StatusCode::CREATED))
}
