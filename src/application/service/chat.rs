use tokio::sync::broadcast::Sender;

use crate::domain::chat;
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repository::chat::Repository;

pub type ChatService = chat::ChatService<Repository>;

pub fn make_chat_service(db_pool: &'static DbPool, tx: Sender<chat::Parcel>) -> ChatService {
    ChatService::new(tx, Repository::new(db_pool))
}
