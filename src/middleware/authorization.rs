use crate::server::http_response::HttpResponse;
use crate::service::Claims;
use jsonwebtoken::{decode, DecodingKey, Validation};
use lazy_static::lazy_static;
use serde::Serialize;
use std::env;
use warp::http::StatusCode;
use warp::{reject::Reject, Filter, Rejection};

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").unwrap();
}

#[derive(Clone, Debug, Serialize)]
struct ForbiddenError {
    pub error: String,
    pub message: String,
}

impl Reject for HttpResponse<ForbiddenError> {}

pub fn with_authorization() -> impl Filter<Extract = (Claims,), Error = Rejection> + Copy + Clone {
    warp::header::<String>("authorization").and_then(|authorizaton_header: String| async move {
        let token: Vec<&str> = authorizaton_header.split(" ").into_iter().collect();

        if token.get(0).is_none() || token.len() != 2 {
            return Err(warp::reject::custom(
                HttpResponse::<String>::new(
                    "Invalid authorization header provided",
                    StatusCode::BAD_REQUEST,
                ),
            ));
        }

        if token.get(0).is_some() {
            let schema = *token.get(0).unwrap();

            if schema.to_lowercase() != "bearer" {
                return Err(warp::reject::custom(
                    HttpResponse::<String>::new(
                        "Invalid token schema, expected Bearer",
                        StatusCode::BAD_REQUEST,
                    ),
                ));
            }
        }

        let token = *token.get(1).unwrap();

        let decode_result = decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        );

        match decode_result {
            Ok(token) => {
                info!("Received request from {}", token.claims.user_id);
                Ok(token.claims)
            }
            Err(e) => Err(warp::reject::custom(
                HttpResponse::<ForbiddenError>::with_payload(
                    ForbiddenError {
                        error: e.to_string(),
                        message: "Forbidden".to_string(),
                    },
                    StatusCode::FORBIDDEN,
                ),
            )),
        }
    })
}
