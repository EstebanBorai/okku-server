use tokio::sync::broadcast::Sender;

use domain::chat::{ChatService, Parcel};

use crate::domain;

pub fn make_chat_service(tx: Sender<Parcel>) -> ChatService {
    ChatService::new(tx)
}
