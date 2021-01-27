use serde::Deserialize;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::application::service::Services;
use crate::domain::auth::Claims;
use crate::server::utils::Response;

#[derive(Deserialize)]
pub struct CreateChatPayload {
    participants_ids: Vec<Uuid>,
}

pub async fn create_chat(
    claims: Claims,
    services: Services,
    payload: CreateChatPayload,
) -> Result<impl warp::Reply, Rejection> {
    match services
        .hub_service
        .chat_provider
        .create_chat(payload.participants_ids)
        .await
    {
        Ok(chat) => Ok(Response::new(chat).status_code(StatusCode::CREATED)),
        Err(e) => Err(
            Response::message(String::from("Unable to create a new chat"))
                .status_code(StatusCode::BAD_REQUEST)
                .reject(),
        ),
    }
}
