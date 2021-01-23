use async_trait::async_trait;
use uuid::Uuid;

use crate::error::Result;

use super::{Chat, HistoryMessageDTO, IncomingMessageDTO};

#[async_trait]
pub trait ChatRepository {
    async fn create_chat(&self) -> Result<Chat>;
    async fn append_user_to_chat(&self, chat_id: &Uuid, user_id: &Uuid) -> Result<Uuid>;
    async fn fetch_user_chats(&self, user_id: &Uuid) -> Result<Vec<Chat>>;
    async fn append_to_chat_history(&self, incoming_message: &IncomingMessageDTO) -> Result<()>;
    async fn retrieve_chat_history(&self, chat_id: &Uuid) -> Result<Vec<HistoryMessageDTO>>;
}
