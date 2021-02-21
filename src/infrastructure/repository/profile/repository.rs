use async_trait::async_trait;
use futures::TryStreamExt;
use sqlx::postgres::Postgres;
use sqlx::{Row, Transaction};
use uuid::Uuid;

use crate::domain::profile::{Profile, ProfileRepository};
use crate::error::{Error, Result};
use crate::infrastructure::database::DbPool;

use super::ProfileDTO;

pub struct Repository {
    db_pool: &'static DbPool,
}

impl Repository {
    pub fn new(db_pool: &'static DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl ProfileRepository for Repository {
    async fn create_tx<'a>(
        &'a self,
        tx: &mut Transaction<'static, Postgres>,
        user_id: &Uuid,
        email: &str,
    ) -> Result<Profile> {
        let dto: ProfileDTO =
            sqlx::query_as("INSERT INTO profiles (user_id, email) VALUES ($1, $2) RETURNING *")
                .bind(user_id)
                .bind(email)
                .fetch_one(tx)
                .await?;

        Ok(ProfileDTO::as_profile(&dto, None))
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Profile> {
        let rows = sqlx::query(
            r#"
            SELECT
                users.id AS user_id,
                users.name,
                profiles.id AS profile_id,
                profiles.first_name,
                profiles.email,
                profiles.surname,
                profiles.birthday,
                profiles.bio
            FROM profiles
            LEFT JOIN users ON users.id = $1"#,
        )
        .bind(user_id)
        .fetch_optional(self.db_pool)
        .await?;

        match rows {
            Some(rows) => Ok(Profile {
                id: rows.try_get("profile_id")?,
                first_name: rows.try_get("first_name")?,
                email: rows.try_get("email")?,
                birthday: rows.try_get("birthday")?,
                bio: rows.try_get("bio")?,
                surname: rows.try_get("surname")?,
                avatar: None,
                contacts: None,
            }),
            None => Err(Error::UserNotFound),
        }
    }
}
