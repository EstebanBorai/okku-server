use crate::database::{DbConn, Row};
use crate::model::{Avatar, AvatarMIMEType, Secret, User};
use anyhow::{Error, Result};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    db_conn: Arc<DbConn>,
}

impl UserService {
    pub fn new(db_conn: Arc<DbConn>) -> Self {
        Self { db_conn }
    }

    /// Get `User` with the provided `name`
    pub async fn get_user_by_name(&self, name: &str) -> Result<User> {
        let result = self
            .db_conn
            .query_one(
                "SELECT * FROM users WHERE users.name = $1 LIMIT 1",
                &[&name],
            )
            .await
            .map_err(Error::from)?;

        Ok(User::new(result.get(0), result.get(1)))
    }

    /// Creates a new `User`
    pub async fn create_user(&self, name: &str) -> Result<User> {
        let rows: Row = self
            .db_conn
            .query_one("INSERT INTO users(name) VALUES ($1) RETURNING *", &[&name])
            .await
            .map_err(Error::from)?;

        Ok(User::new(rows.get(0), rows.get(1)))
    }

    pub async fn get_user_with_secret(&self, name: &str) -> Result<(User, Secret)> {
        let rows: Row = self
            .db_conn
            .query_one(
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
                &[&name],
            )
            .await
            .map_err(Error::from)?;

        Ok((
            User::new(rows.get(0), rows.get(1)),
            Secret {
                id: rows.get(2),
                hash: rows.get(3),
                user_id: rows.get(4),
            },
        ))
    }

    pub async fn set_avatar(
        &self,
        uid: &Uuid,
        content_type: AvatarMIMEType,
        file_bytes: Vec<u8>,
    ) -> Result<Avatar> {
        let has_avatar = self
            .db_conn
            .query_one("SELECT COUNT(1) FROM avatars WHERE user_id = $1", &[&uid])
            .await
            .is_ok();

        if has_avatar {
            // if the provided user already has an avatar, we replace the current
            // avatar with the one provided by the `file_bytes`
            let rows: Row = self
                .db_conn
                .query_one(
                    r#"
                UPDATE avatars
                SET
                    image = $1,
                    mime_type = $2
                WHERE
                    user_id = $3
                RETURNING *"#,
                    &[&file_bytes.as_slice(), &content_type.to_string(), &uid],
                )
                .await
                .map_err(Error::from)?;

            return Ok(Avatar {
                id: rows.get(0),
                image: rows.get(1),
                mime_type: rows.get(2),
            });
        }

        let rows: Row = self
            .db_conn
            .query_one(
                r#"
                INSERT INTO avatars (image, user_id, mime_type)
                VALUES($1, $2, $3)
                RETURNING *"#,
                &[&file_bytes.as_slice(), &uid, &content_type.to_string()],
            )
            .await
            .map_err(Error::from)?;

        Ok(Avatar {
            id: rows.get(0),
            image: rows.get(1),
            mime_type: rows.get(2),
        })
    }

    pub async fn download_avatar(&self, user_id: &Uuid) -> Result<Avatar> {
        let rows: Row = self
            .db_conn
            .query_one(
                "SELECT id, image, mime_type FROM avatars WHERE user_id = $1",
                &[&user_id],
            )
            .await
            .map_err(Error::from)?;

        Ok(Avatar {
            id: rows.get(0),
            image: rows.get(1),
            mime_type: rows.get(2),
        })
    }
}
