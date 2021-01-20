use serde::{Deserialize, Serialize};
use warp::http::StatusCode;

use crate::application::service::Services;
use crate::domain::user::User;
use crate::server::utils::Response;

#[derive(Deserialize)]
pub struct SignupPayload {
    email: String,
    name: String,
    password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    token: String,
    user: User,
}

pub async fn signup(
    body: SignupPayload,
    services: Services,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    let user = match services
        .user_service
        .register(&body.name, &body.email, body.password.as_bytes())
        .await
    {
        Ok(user) => user,
        Err(e) => {
            return Err(Response::message(e.message())
                .status_code(StatusCode::BAD_REQUEST)
                .reject());
        }
    };

    match services
        .auth_service
        .authenticate(body.password.as_bytes(), &user.id)
        .await
    {
        Ok(token) => {
            Ok(Response::new(SignupResponse { token, user }).status_code(StatusCode::CREATED))
        }
        Err(e) => Err(Response::message(e.message())
            .status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .reject()),
    }
}
