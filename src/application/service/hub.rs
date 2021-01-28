use std::sync::Arc;

use crate::domain::chat::{ChatRepository, HubService};
use crate::infrastructure::database::DbPool;

use super::UserService;

pub fn make_hub_service(db_pool: &'static DbPool, user_service: Arc<UserService>) -> HubService {
    let chat_repository = ChatRepository::new(db_pool);

    HubService::new(chat_repository, user_service)
}
