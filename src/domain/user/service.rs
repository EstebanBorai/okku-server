use regex::Regex;
use uuid::Uuid;

use crate::error::{Error, Result};

use super::{User, UserRepository};

const USERNAME_REGEX: &str = r#"^[a-z0-9.]{7,20}$"#;

pub struct UserService<R>
where
    R: UserRepository,
{
    user_repository: R,
}

impl<R> UserService<R>
where
    R: UserRepository,
{
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub async fn create(&self, name: &str) -> Result<User> {
        let username_regex = Regex::new(USERNAME_REGEX).unwrap();

        if !username_regex.is_match(name) {
            return Err(Error::InvalidUsername(name.to_string()));
        }

        let user = self.user_repository.create(name).await?;

        Ok(user)
    }

    pub async fn find_one(&self, id: &Uuid) -> Result<User> {
        self.user_repository.find_one(id).await
    }

    pub async fn find_by_name(&self, name: &str) -> Result<User> {
        self.user_repository.find_by_name(name).await
    }
}
