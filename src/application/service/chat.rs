use domain::chat::ChatService;

use crate::domain;

pub fn make_chat_service() -> ChatService {
  ChatService::new()
}
