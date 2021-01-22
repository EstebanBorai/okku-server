use async_trait::async_trait;
use uuid::Uuid;

use crate::error::Result;

use super::Chat;
use super::{Message, MessageKind};

#[async_trait]
pub trait ChatRepository {
    async fn create_chat(&self) -> Result<Chat>;
    async fn fetch_user_chats(&self, user_id: &Uuid) -> Result<Vec<Chat>>;
    async fn append_to_chat_history(
        &self,
        chat_id: &Uuid,
        author_id: &Uuid,
        content: String,
        kind: MessageKind,
        file_id: Option<Uuid>,
    ) -> Result<()>;
    async fn retrieve_chat_history(&self, chat_id: &Uuid) -> Result<Vec<Message>>;
}
