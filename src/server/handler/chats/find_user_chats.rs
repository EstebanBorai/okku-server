use serde::Serialize;
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::application::service::Services;
use crate::domain::auth::Claims;
use crate::domain::chat::Chat;
use crate::server::utils::Response;

#[derive(Serialize)]
pub struct FindUserChatsResponse {
    chats: Vec<Chat>,
}

pub async fn find_user_chats(
    claims: Claims,
    services: Services,
) -> Result<impl warp::Reply, Rejection> {
    match services
        .hub_service
        .chat_provider
        .fetch_chats(&claims.user_id)
        .await
    {
        Ok(chats) => Ok(Response::new(FindUserChatsResponse { chats }).status_code(StatusCode::OK)),
        Err(e) => Err(Response::message(e.to_string())
            .status_code(StatusCode::BAD_REQUEST)
            .reject()),
    }
}
