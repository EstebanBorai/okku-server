use regex::Regex;
use uuid::Uuid;

use crate::domain::user::{User, UserRepository, UserService};
use crate::error::{Error, Result};

use super::{Profile, ProfileRepository};

const EMAIL_REGEX: &str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;

pub struct ProfileService<R>
where
    R: ProfileRepository,
{
    profile_repository: R,
}

impl<R> ProfileService<R>
where
    R: ProfileRepository,
{
    pub fn new(profile_repository: R) -> Self {
        Self { profile_repository }
    }

    pub async fn create(&self, user: &User, first_name: &str, email: &str) -> Result<Profile> {
        let email_regex = Regex::new(EMAIL_REGEX).unwrap();

        if !email_regex.is_match(email) {
            return Err(Error::InvalidEmailAddress(email.to_string()));
        }

        self.profile_repository.create(user).await
    }

    pub async fn find_by_user_id(&self, id: &Uuid) -> Result<Profile> {
        self.profile_repository.find_by_user_id(id).await
    }
}
