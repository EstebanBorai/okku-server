use sqlx::Row;
use uuid::Uuid;

use crate::database::DbPool;
use crate::error::AppError;
use crate::model::{avatar::Avatar, secret::Secret, user::User};

#[derive(Clone)]
pub struct UserService {
    db_pool: DbPool,
}

impl UserService {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    /// Get `User` with the provided `name`
    pub async fn get_user_by_name(&self, name: &str) -> Result<User, AppError> {
        let user: User = sqlx::query_as("SELECT * FROM users WHERE users.name = $1 LIMIT 1")
            .bind(name)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(user)
    }

    /// Creates a new `User`
    pub async fn create_user(&self, name: &str) -> Result<User, AppError> {
        let user: User = sqlx::query_as("INSERT INTO users(name) VALUES ($1) RETURNING *")
            .bind(name)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(user)
    }

    pub async fn get_user_and_secret(&self, name: &str) -> Result<(User, Secret), AppError> {
        let rows = sqlx::query(
            r#"
            SELECT
              users.id,
              users.name,
              secrets.id AS secret_id,
              secrets.hash
            FROM
              users
            LEFT JOIN secrets ON secrets.user_id = users.id
            WHERE
              users.name = $1"#,
        )
        .bind(name)
        .fetch_one(&self.db_pool)
        .await?;

        Ok((
            User::new(rows.get(0), rows.get(1)),
            Secret {
                id: rows.get(2),
                hash: rows.get(3),
                user_id: rows.get(0),
            },
        ))
    }

    pub async fn set_avatar(
        &self,
        uid: &Uuid,
        content_type: &str,
        file_bytes: Vec<u8>,
    ) -> Result<Avatar, AppError> {
        let has_avatar = sqlx::query("SELECT id FROM avatars WHERE user_id = $1")
            .bind(uid)
            .fetch_one(&self.db_pool)
            .await
            .is_err();

        if has_avatar {
            let avatar: Avatar = sqlx::query_as(
                r#"
            INSERT INTO avatars 
                (image, user_id, mime_type)
            VALUES ($1, $2, $3)
            RETURNING *"#,
            )
            .bind(&file_bytes.as_slice())
            .bind(&uid)
            .bind(&content_type.to_string())
            .fetch_one(&self.db_pool)
            .await?;

            return Ok(avatar);
        }

        // if the provided user already has an avatar, we replace the current
        // avatar with the one provided by the `file_bytes`
        let avatar: Avatar = sqlx::query_as(
            r#"
            UPDATE avatars
            SET
                image = $1,
                mime_type = $2
            WHERE
                user_id = $3
            RETURNING *"#,
        )
        .bind(&file_bytes.as_slice())
        .bind(&content_type.to_string())
        .bind(&uid)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(avatar)
    }

    pub async fn download_avatar(&self, user_id: &Uuid) -> Result<Avatar, AppError> {
        let avatar: Avatar = sqlx::query_as(
            r#"SELECT id, image, mime_type
            FROM avatars
            WHERE user_id = $1"#,
        )
        .bind(&user_id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(avatar)
    }
}
