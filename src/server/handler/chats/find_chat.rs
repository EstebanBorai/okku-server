use uuid::Uuid;
use warp::http::StatusCode;
use warp::reject::Rejection;

use crate::application::service::Services;
use crate::domain::auth::Claims;
use crate::server::utils::Response;

pub async fn find_chat(
    _: Claims,
    services: Services,
    chat_id: Uuid,
) -> Result<impl warp::Reply, Rejection> {
    match services.hub_service.chat_provider.find_chat(&chat_id).await {
        Ok(chat) => Ok(Response::new(chat).status_code(StatusCode::OK)),
        Err(e) => Err(Response::message(e.to_string())
            .status_code(StatusCode::BAD_REQUEST)
            .reject()),
    }
}
