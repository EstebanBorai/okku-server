use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::chat::dto::InputProtoMessageDTO;
use crate::domain::chat::entity::{Chat, Message, Output};
use crate::domain::chat::{ChatRepository, MessagesRepository};
use crate::error::{Error, Result};

pub struct ChatProvider {
    chats: RwLock<HashMap<Uuid, Chat>>,
    chat_repository: ChatRepository,
    messages_repository: MessagesRepository,
}

impl ChatProvider {
    pub fn new(chat_repository: ChatRepository, messages_repository: MessagesRepository) -> Self {
        Self {
            chats: RwLock::new(HashMap::new()),
            chat_repository,
            messages_repository,
        }
    }

    pub async fn create_chat(&self, participants_ids: Vec<Uuid>) -> Result<Chat> {
        if participants_ids.len() < 2 {
            return Err(Error::ChatNotEnoughParticipants(
                participants_ids.len() as u8
            ));
        }

        let chat = self.chat_repository.create(participants_ids).await?;

        self.chats.write().await.insert(chat.id, chat.clone());

        Ok(chat)
    }

    pub async fn fetch_chats(&self, user_id: &Uuid) -> Result<Vec<Chat>> {
        self.chat_repository.fetch_user_chats(user_id).await
    }

    pub async fn handle_incoming_message(
        &self,
        incoming_message: InputProtoMessageDTO,
    ) -> Result<Message> {
        let (chat, incoming_message) = self.validate_incoming_message(incoming_message).await?;

        self.messages_repository
            .create(chat, incoming_message)
            .await
    }

    async fn validate_incoming_message(
        &self,
        incoming_message: InputProtoMessageDTO,
    ) -> Result<(Chat, InputProtoMessageDTO)> {
        if let Some(chat) = self.chats.read().await.get(&incoming_message.chat_id) {
            if chat
                .participants_ids
                .iter()
                .any(|participant_id| *participant_id == incoming_message.author_id)
            {
                return Ok((chat.to_owned(), incoming_message));
            }
        }

        if let Ok(chat) = self
            .chat_repository
            .find_by_id(&incoming_message.chat_id)
            .await
        {
            if chat
                .participants_ids
                .iter()
                .any(|participant_id| *participant_id == incoming_message.author_id)
            {
                return Ok((chat.to_owned(), incoming_message));
            }

            return Err(Error::UserDoesntBelongToChat(
                incoming_message.author_id,
                incoming_message.chat_id,
            ));
        }

        Err(Error::ChatNotFound)
    }

    pub async fn find_chat(&self, chat_id: &Uuid) -> Result<Chat> {
        if let Some(chat) = self.chats.read().await.get(&chat_id) {
            return Ok(chat.clone());
        }

        let chat = self.chat_repository.find_by_id(chat_id).await?;

        match self.chats.write().await.insert(chat.id, chat) {
            Some(chat) => Ok(chat),
            None => Err(Error::ChatNotFound),
        }
    }
}
