use serde::Serialize;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::application::service::Services;
use crate::domain::auth::Claims;
use crate::domain::chat::Message;
use crate::server::utils::Response;

#[derive(Serialize)]
pub struct FetchChatMessagesResponse {
    messages: Vec<Message>,
}

pub async fn fetch_chat_messages(
    _: Claims,
    services: Services,
    chat_id: Uuid,
) -> Result<impl warp::Reply, Rejection> {
    match services
        .hub_service
        .chat_provider
        .fetch_chat_messages(&chat_id)
        .await
    {
        Ok(messages) => {
            Ok(Response::new(FetchChatMessagesResponse { messages }).status_code(StatusCode::OK))
        }
        Err(e) => Err(Response::message(e.to_string())
            .status_code(StatusCode::BAD_REQUEST)
            .reject()),
    }
}
