use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::chat::{Chat, ChatRepository, Message, MessageKind};
use crate::error::{Error, Result};
use crate::infrastructure::database::DbPool;

use super::{ChatDTO, ChatsUsersDTO, MessageDTO};

pub struct Repository {
    db_pool: &'static DbPool,
}

impl Repository {
    pub fn new(db_pool: &'static DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl ChatRepository for Repository {
    async fn create_chat(&self) -> Result<Chat> {
        let chat: ChatDTO = sqlx::query_as("INSERT INTO chats RETURNING *")
            .fetch_one(self.db_pool)
            .await?;

        Ok(ChatDTO::as_chat(&chat))
    }

    async fn fetch_user_chats(&self, user_id: &Uuid) -> Result<Vec<Chat>> {
        let chats: Vec<ChatsUsersDTO> =
            sqlx::query_as("SELECT * FROM chats_users WHERE user_id = $1")
                .bind(user_id)
                .fetch_all(self.db_pool)
                .await?;

        let chats: Vec<Chat> = chats
            .iter()
            .map(|chats_users_dto| ChatsUsersDTO::as_chat(chats_users_dto))
            .collect();

        Ok(chats)
    }

    async fn append_to_chat_history(
        &self,
        chat_id: &Uuid,
        author_id: &Uuid,
        content: String,
        kind: MessageKind,
        file_id: Option<Uuid>,
    ) -> Result<()> {
        let _: MessageDTO = sqlx::query_as(
            r#"INSERT INTO messages (
                content,
                kind,
                author_id,
                chat_id,
                file_id
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5
            ) RETURNING *"#,
        )
        .bind(&content)
        .bind(kind.to_string())
        .bind(author_id)
        .bind(chat_id)
        .bind(file_id)
        .fetch_one(self.db_pool)
        .await
        .map_err(Error::from)?;

        Ok(())
    }

    async fn retrieve_chat_history(
        &self,
        chat_id: &Uuid,
    ) -> Result<Vec<Message>> {
        let messages: Vec<MessageDTO> = sqlx::query_as("SELECT * FROM messages WHERE chat_id = $1")
            .bind(chat_id)
            .fetch_all(self.db_pool)
            .await
            .map_err(Error::from)?;

    }
}
