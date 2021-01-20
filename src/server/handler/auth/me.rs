use serde::Serialize;
use warp::http::StatusCode;

use crate::application::service::Services;
use crate::domain::auth::Claims;
use crate::server::utils::Response;

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn me(
    claims: Claims,
    services: Services,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    match services.user_service.fetch_profile(&claims.user_id).await {
        Ok(profile) => Ok(Response::new(profile).status_code(StatusCode::OK)),
        Err(e) => Err(Response::reject_with(e, StatusCode::FORBIDDEN)),
    }
}
