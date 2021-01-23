use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::application::service::Services;
use crate::domain::auth::Claims;
use crate::domain::chat::Chat;
use crate::domain::user::User;
use crate::server::utils::Response;

#[derive(Deserialize)]
pub struct CreateChatPayload {
    participants: Vec<User>,
}

#[derive(Serialize)]
pub struct CreateChatResponse {
    chat: Chat,
}

pub async fn create(
    _: Claims,
    services: Services,
    payload: CreateChatPayload,
) -> Result<impl warp::Reply, Rejection> {
    if payload.participants.len() < 2 {
        return Err(
            Response::message(String::from("Chat must have at least 2 participants"))
                .status_code(StatusCode::BAD_REQUEST)
                .reject(),
        );
    }

    match services
        .chat_service
        .create_chat(payload.participants)
        .await
    {
        Ok(chat) => Ok(Response::new(CreateChatResponse { chat }).status_code(StatusCode::CREATED)),
        Err(e) => Err(Response::message(e.message())
            .status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .reject()),
    }
}
