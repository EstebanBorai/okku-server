use sqlx::Row;

use crate::database::DbPool;
use crate::error::Result;
use crate::model::{secret::Secret, user::User};

#[derive(Clone)]
pub struct UserService {
    db_pool: DbPool
}

impl UserService {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    /// Get `User` with the provided `name`
    pub async fn get_user_by_name(&self, name: &str) -> Result<User> {
        let user: User = sqlx::query_as("SELECT * FROM users WHERE users.name = $1 LIMIT 1")
            .bind(name)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(user)
    }

    /// Creates a new `User`
    pub async fn create_user(&self, name: &str) -> Result<User> {
        let user: User = sqlx::query_as("INSERT INTO users(name) VALUES ($1) RETURNING *")
            .bind(name)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(user)
    }

    pub async fn get_user_and_secret(&self, name: &str) -> Result<(User, Secret)> {
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
}
