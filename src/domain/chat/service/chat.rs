use futures::StreamExt;
use std::collections::HashMap;
use std::default::Default;
use std::time::Duration;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::RwLock;
use tokio::time::delay_for;
use uuid::Uuid;

use crate::domain::chat::dto::InputProtoMessageDTO;
use crate::domain::chat::entity::{Chat, IncomingMessage, Input, Message, Output, Proto};
use crate::domain::user::User;

pub struct ChatService {
    chats: RwLock<HashMap<Uuid, Chat>>,
}

impl Default for ChatService {
    fn default() -> Self {
        Self {
            chats: RwLock::new(HashMap::new()),
        }
    }
}

impl ChatService {
    pub async fn create_chat(&self, participants_ids: Vec<Uuid>) {
        let chat = Chat::new_with_participants(participants_ids);

        self.chats.write().await.insert(chat.id, chat);
    }

    pub async fn validate_incoming_message(
        &self,
        incoming_message: InputProtoMessageDTO,
    ) -> Result<Message, ()> {
        if let Some(chat) = self.chats.read().await.get(&incoming_message.chat_id) {
            if chat
                .participants_ids
                .iter()
                .any(|participant_id| *participant_id == incoming_message.author_id)
            {
                return Ok(Message {
                    id: Uuid::new_v4(),
                    author: User {
                        id: incoming_message.author_id,
                        name: String::from("fetch_this_from_database"),
                    },
                    chat: chat.to_owned(),
                    body: incoming_message.body,
                    created_at: incoming_message.created_at,
                });
            }

            return Err(());
        }

        Err(())
    }

    pub async fn find_chat(&self, chat_id: &Uuid) -> Result<Chat, ()> {
        if let Some(chat) = self.chats.read().await.get(&chat_id) {
            return Ok(chat.clone());
        }

        Err(())
    }
}
