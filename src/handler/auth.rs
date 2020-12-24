use warp::http::StatusCode;

use crate::error::MSendError;
use crate::server::http_response::HttpResponse;
use crate::service::auth::{Token, UserRegister};
use crate::service::InjectedServices;

pub async fn signup(
    user_register: UserRegister,
    services: InjectedServices,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    match services.auth_service.signup(user_register).await {
        Ok(user) => {
            let token = services.auth_service.sign_jwt_token(&user).unwrap();

            Ok(HttpResponse::<Token>::with_payload(token, StatusCode::OK))
        }
        Err(e) => Ok(e.into_http()),
    }
}

pub async fn login(
    auth_header_value: String,
    services: InjectedServices,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    match services.auth_service.login(&auth_header_value).await {
        Ok(token) => Ok(HttpResponse::<Token>::with_payload(token, StatusCode::OK)),
        Err(e) => Ok(e.into_http()),
    }
}
