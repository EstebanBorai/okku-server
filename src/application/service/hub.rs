use crate::domain::chat::{ChatRepository, HubService};
use crate::infrastructure::database::DbPool;

pub fn make_hub_service(db_pool: &'static DbPool) -> HubService {
    let chat_repository = ChatRepository::new(db_pool);

    HubService::new(chat_repository)
}
