use serde::Serialize;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::application::service::Services;
use crate::domain::auth::Claims;
use crate::domain::chat::HistoryMessageDTO;
use crate::server::utils::Response;

#[derive(Serialize)]
pub struct ChatHistoryResponse {
    messages: Vec<HistoryMessageDTO>,
}

pub async fn retrieve_chat_history(
    _: Claims,
    services: Services,
    chat_id: Uuid,
) -> Result<impl warp::Reply, Rejection> {
    match services.chat_service.retrieve_chat_history(&chat_id).await {
        Ok(messages) => {
            Ok(Response::new(ChatHistoryResponse { messages }).status_code(StatusCode::OK))
        }
        Err(e) => Err(Response::message(e.message())
            .status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .reject()),
    }
}
