use crate::error::{Error, Result};

use super::{User, UserRepository};

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
        if name.len() > 40 {
            return Err(Error::Validation(
                "Invalid \"name\" for user. The name must have a maximum of 40 characters"
                    .to_string(),
            ));
        }

        let user = self.user_repository.create(name).await?;

        Ok(user)
    }

    pub async fn find_all(&self) -> Result<Vec<User>> {
        self.user_repository.find_all().await
    }

    pub async fn find_by_name(&self, name: &str) -> Result<User> {
        self.user_repository.find_by_name(name).await
    }
}
