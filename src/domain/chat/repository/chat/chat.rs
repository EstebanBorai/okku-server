use sqlx::postgres::Postgres;
use sqlx::Transaction;
use uuid::Uuid;

use crate::domain::chat::entity::Chat;
use crate::error::Result;
use crate::infrastructure::database::DbPool;

use super::dto::{ChatDTO, ChatsUsersDTO};

pub struct ChatRepository {
    db_pool: &'static DbPool,
}

impl ChatRepository {
    pub fn new(db_pool: &'static DbPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, participants_ids: Vec<Uuid>) -> Result<Chat> {
        let mut tx = self.db_pool.begin().await?;

        let chat: ChatDTO =
            sqlx::query_as("INSERT INTO chats (id) VALUES (uuid_generate_v4()) RETURNING *")
                .fetch_one(&mut tx)
                .await?;

        let _: ChatsUsersDTO = sqlx::query_as(
            ChatRepository::make_insert_chats_users_query(&chat.id, &participants_ids).as_str(),
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(Chat {
            id: chat.id,
            messages: Vec::new(),
            participants_ids,
        })
    }

    pub async fn fetch_chats_by_participant_id(participant_id: &Uuid) -> Result<Vec<Chat>> {
        todo!();
        // let chats: Vec<ChatDTO> =             sqlx::query_as("SELECT * FROM chats WHERE (uuid_generate_v4()) RETURNING *")
        // .fetch_one(&mut tx)
        // .await?;
        // WITH user_chats AS (
        //     SELECT
        //         chats_users.chat_id
        //     FROM
        //         chats_users
        //     WHERE
        //         chats_users.user_id = 'a63ddc44-f19c-4c32-a926-56c1eb09750f'::uuid
        //     GROUP BY
        //         chats_users.chat_id
        // )
        // SELECT
        //     *
        // FROM
        //     user_chats
        // INNER JOIN chats_user ON chats_users.chat_id = user_chats.chat_id
        // WHERE
        //     user_chats.chat_id = chats_users.user_id;
    }

    /// Creates a SQL query to insert multiple relationships of
    /// chats(id) and users(id) with the provided `participants_ids`
    ///
    /// Notice: This function concatenates strings _as is_, theres no
    /// SQL injection validation this must be improved in the future and
    /// is implemented as a POC.
    fn make_insert_chats_users_query(chat_id: &Uuid, participants_ids: &Vec<Uuid>) -> String {
        let mut query = String::from("INSERT INTO chats_users (chat_id, user_id) VALUES ");

        for (idx, participant) in participants_ids.iter().enumerate() {
            if idx == participants_ids.len() - 1 {
                query.push_str(&format!(
                    "('{chat_id}', '{user_id}')",
                    chat_id = chat_id,
                    user_id = participant
                ));
            } else {
                query.push_str(&format!(
                    "('{chat_id}', '{user_id}'), ",
                    chat_id = chat_id,
                    user_id = participant
                ));
            }
        }

        query.push_str(" RETURNING *");

        String::from(query)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    const INSERT_CHATS_USERS_QUERY_EXPECT: &str = "INSERT INTO chats_users (chat_id, user_id) VALUES (\'225ceb5c-b595-482c-a190-118e0b72de6b\', \'d41ea14c-bd86-4207-bbaa-19e12d9eb777\'), (\'225ceb5c-b595-482c-a190-118e0b72de6b\', \'2b2f4989-00c9-47d2-8534-6cee0be6462b\') RETURNING *";

    #[test]
    fn it_makes_insert_chats_users_query() {
        let chat_id = Uuid::from_str("225ceb5c-b595-482c-a190-118e0b72de6b").unwrap();
        let participants_ids = vec![
            Uuid::from_str("d41ea14c-bd86-4207-bbaa-19e12d9eb777").unwrap(),
            Uuid::from_str("2b2f4989-00c9-47d2-8534-6cee0be6462b").unwrap(),
        ];
        let query = ChatRepository::make_insert_chats_users_query(&chat_id, &participants_ids);

        assert_eq!(query, INSERT_CHATS_USERS_QUERY_EXPECT.to_string());
    }
}
