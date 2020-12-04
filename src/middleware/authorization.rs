use crate::server::http_response::HttpResponse;
use crate::service::Claims;
use jsonwebtoken::{decode, DecodingKey, Validation};
use lazy_static::lazy_static;
use std::env;
use warp::http::StatusCode;
use warp::{reject, Filter, Rejection};

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").unwrap();
}

pub fn with_authorization() -> impl Filter<Extract = (Claims,), Error = Rejection> + Copy + Clone {
    warp::header::<String>("authorization").and_then(|authorizaton_header: String| async move {
        if let Ok(token) = decode::<Claims>(
            &authorizaton_header,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        ) {
            info!("Received request from {}", token.claims.user_id);
            Ok(token.claims)
        } else {
            Err(warp::reject::custom(HttpResponse::<String>::new(
                "Forbidden",
                StatusCode::FORBIDDEN,
            )))
        }
    })
}
