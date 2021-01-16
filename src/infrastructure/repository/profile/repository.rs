use async_trait::async_trait;
use futures::TryStreamExt;
use sqlx::Row;
use uuid::Uuid;

use crate::domain::profile::{Profile, ProfileRepository};
use crate::domain::user::User;
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
    async fn create(&self, user: &User) -> Result<Profile> {
        let dto: ProfileDTO =
            sqlx::query_as("INSERT INTO profiles (user_id) VALUES ($1, $2, $3) RETURNING *")
                .bind(user.id)
                .fetch_one(self.db_pool)
                .await?;

        Ok(ProfileDTO::into_profile(&dto, user, None))
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Profile> {
        let mut rows = sqlx::query(
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
        .fetch(self.db_pool);

        if let Some(row) = rows.try_next().await? {
            return Ok(Profile {
                id: row.try_get("profile_id")?,
                user: User {
                    id: row.try_get("user_id")?,
                    name: row.try_get("name")?,
                },
                first_name: row.try_get("first_name")?,
                email: row.try_get("email")?,
                birthday: row.try_get("birthday")?,
                bio: row.try_get("bio")?,
                surname: row.try_get("surname")?,
                avatar: None,
                contacts: None,
            });
        }

        Err(Error::UserNotFound)
    }
}
