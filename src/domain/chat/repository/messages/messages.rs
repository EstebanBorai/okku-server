use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use sqlx::Row;
use uuid::Uuid;

use crate::domain::chat::dto::InputProtoMessageDTO;
use crate::domain::chat::entity::{Chat, Message};
use crate::domain::user::User;
use crate::error::{Error, Result};
use crate::infrastructure::database::DbPool;

pub struct MessagesRepository {
    db_pool: &'static DbPool,
}

impl MessagesRepository {
    pub fn new(db_pool: &'static DbPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(
        &self,
        chat: Chat,
        input_proto_message: InputProtoMessageDTO,
    ) -> Result<Message> {
        let mut rows = sqlx::query(
            r#"
            WITH message AS (
                INSERT INTO messages (content,
                        kind,
                        author_id,
                        chat_id)
                        VALUES($1,
                            $2,
                            $3,
                            $4)
                    RETURNING
                        *
                )
                SELECT
                    message.id AS message_id,
                    message.content AS message_content,
                    message.created_at AS message_created_at,
                    users.id AS author_id,
                    users. "name" AS author_name
                FROM
                    message
                    INNER JOIN chats ON chats.id = $4
                    INNER JOIN users ON users.id = $3;
                "#,
        )
        .bind(input_proto_message.body)
        .bind("text")
        .bind(input_proto_message.author_id)
        .bind(input_proto_message.chat_id)
        .fetch(self.db_pool);

        while let Some(row) = rows.try_next().await? {
            let message_id: Uuid = row.try_get("message_id")?;
            let message_content: String = row.try_get("message_content")?;
            let message_created_at: DateTime<Utc> = row.try_get("message_created_at")?;
            let author_id: Uuid = row.try_get("author_id")?;
            let author_name: String = row.try_get("author_name")?;

            return Ok(Message {
                id: message_id,
                author: User {
                    id: author_id,
                    name: author_name,
                },
                chat,
                body: message_content,
                created_at: message_created_at,
            });
        }

        Err(Error::UnableToStoreMessage)
    }
}
