use serde::Serialize;
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::application::service::Services;
use crate::domain::auth::Claims;
use crate::domain::chat::Chat;
use crate::server::utils::Response;

#[derive(Serialize)]
pub struct RetrieveUserChatsResponse {
    chats: Vec<Chat>,
}

pub async fn retrieve_user_chats(
    claims: Claims,
    services: Services,
) -> Result<impl warp::Reply, Rejection> {
    match services
        .chat_service
        .retrieve_user_chats(&claims.user_id)
        .await
    {
        Ok(chats) => {
            Ok(Response::new(RetrieveUserChatsResponse { chats }).status_code(StatusCode::OK))
        }
        Err(e) => Err(Response::message(e.message())
            .status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .reject()),
    }
}
