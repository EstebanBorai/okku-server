use jsonwebtoken::{decode, DecodingKey, Validation};
use lazy_static::lazy_static;
use serde::Serialize;
use std::env;
use warp::http::StatusCode;
use warp::{Filter, Rejection};

use crate::application::service::Claims;
use crate::server::utils::Response;

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").unwrap();
}

#[derive(Clone, Debug, Serialize)]
struct ForbiddenError {
    pub error: String,
    pub message: String,
}

pub fn with_authorization() -> impl Filter<Extract = (Claims,), Error = Rejection> + Copy + Clone {
    warp::header::<String>("authorization").and_then(|authorizaton_header: String| async move {
        let token: Vec<&str> = authorizaton_header.split(" ").into_iter().collect();

        if token.get(0).is_none() || token.len() != 2 {
            return Err(
                Response::message("Invalid authorization header provided".to_string())
                    .status_code(StatusCode::FORBIDDEN)
                    .reject(),
            );
        }

        if token.get(0).is_some() {
            let schema = *token.get(0).unwrap();

            if schema.to_lowercase() != "bearer" {
                return Err(
                    Response::message("Invalid authorization header provided".to_string())
                        .status_code(StatusCode::FORBIDDEN)
                        .reject(),
                );
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
            Err(_) => Err(
                Response::message("Invalid authorization header provided".to_string())
                    .status_code(StatusCode::FORBIDDEN)
                    .reject(),
            ),
        }
    })
}
