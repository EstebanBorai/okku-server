use async_trait::async_trait;
use futures::TryStreamExt;
use sqlx::Row;
use uuid::Uuid;

use crate::domain::chat::{Chat, ChatRepository, HistoryMessageDTO, IncomingMessageDTO};
use crate::error::{Error, Result};
use crate::infrastructure::database::DbPool;

use super::{ChatDTO, ChatsUsersDTO};

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
        let chat: ChatDTO =
            sqlx::query_as("INSERT INTO chats (id) VALUES (uuid_generate_v4()) RETURNING *")
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

    async fn append_to_chat_history(&self, incoming_message: &IncomingMessageDTO) -> Result<()> {
        let _: HistoryMessageDTO = sqlx::query_as(
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
        .bind(&incoming_message.content)
        .bind(&incoming_message.kind)
        .bind(incoming_message.author_id)
        .bind(incoming_message.chat_id)
        .bind(incoming_message.file_id)
        .fetch_one(self.db_pool)
        .await
        .map_err(Error::from)?;

        Ok(())
    }

    async fn retrieve_chat_history(&self, chat_id: &Uuid) -> Result<Vec<HistoryMessageDTO>> {
        let messages: Vec<HistoryMessageDTO> =
            sqlx::query_as("SELECT * FROM messages WHERE chat_id = $1")
                .bind(chat_id)
                .fetch_all(self.db_pool)
                .await
                .map_err(Error::from)?;

        return Ok(messages);
    }

    async fn append_user_to_chat(&self, chat_id: &Uuid, user_id: &Uuid) -> Result<Uuid> {
        let mut rows = sqlx::query(
            r#"INSERT INTO chats_users (
                chat_id,
                user_id
            ) VALUES (
                $1,
                $2
            ) RETURNING *"#,
        )
        .bind(chat_id)
        .bind(user_id)
        .fetch(self.db_pool);

        if let Some(row) = rows.try_next().await? {
            let relation_id: Uuid = row.try_get("id")?;

            return Ok(relation_id);
        }

        Err(Error::RelateChatWithUserError(
            chat_id.to_owned(),
            user_id.to_owned(),
        ))
    }
}
