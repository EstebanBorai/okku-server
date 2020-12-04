use crate::server::http_response::HttpResponse;
use crate::service::{InjectedServices, Token, UserRegister};
use anyhow::Result;
use warp::http::StatusCode;

pub async fn signup(
    user_register: UserRegister,
    services: InjectedServices,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    match services.auth_service.signup(user_register).await {
        Ok(user) => {
            let token = services.auth_service.sign_jwt_token(&user).unwrap();

            Ok(HttpResponse::<Token>::with_payload(token, StatusCode::OK))
        }
        Err(error) => Ok(HttpResponse::new(
            &error.to_string(),
            StatusCode::BAD_REQUEST,
        ).into()),
    }
}

pub async fn login(
    auth_header_value: String,
    services: InjectedServices,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    match services.auth_service.login(&auth_header_value).await {
        Ok(token) => Ok(HttpResponse::<Token>::with_payload(token, StatusCode::OK)),
        Err(error) => Ok(HttpResponse::new(
            &error.to_string(),
            StatusCode::BAD_REQUEST,
        ).into()),
    }
}
