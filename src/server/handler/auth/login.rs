use http_auth_basic::Credentials;
use serde::Serialize;
use warp::http::StatusCode;

use crate::application::service::Services;
use crate::server::utils::Response;

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn login(
    authorization_header: String,
    services: Services,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    let credentials = match Credentials::from_header(authorization_header) {
        Ok(creds) => creds,
        Err(e) => return Err(Response::reject_with(e.into(), StatusCode::FORBIDDEN)),
    };

    // at this point, "user_id" is the same value as the "User.name"
    // "user_id" is defined for the Basic Authentication as the username/email
    // instead of the resource ID
    let user = match services
        .user_service
        .find_by_name(credentials.user_id.as_str())
        .await
    {
        Ok(user) => user,
        Err(e) => return Err(Response::reject_with(e, StatusCode::FORBIDDEN)),
    };

    match services
        .auth_service
        .authenticate(credentials.password.as_bytes(), &user.id)
        .await
    {
        Ok(token) => Ok(Response::new(LoginResponse { token })),
        Err(e) => {
            return Err(Response::message(e.message())
                .status_code(StatusCode::FORBIDDEN)
                .reject());
        }
    }
}
