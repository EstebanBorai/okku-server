use lazy_static::lazy_static;
use regex::Regex;
use sqlx::postgres::Postgres;
use sqlx::Transaction;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::profile::{Profile, ProfileRepository, ProfileService};
use crate::domain::secret::{SecretRepository, SecretService};
use crate::error::{Error, Result};
use crate::infrastructure::repository::base::BaseRepository;

use super::{User, UserRepository};

lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new("^[a-z0-9.]{7,20}$").unwrap();
}

pub struct UserService<R, S, T>
where
    R: UserRepository + BaseRepository,
    S: ProfileRepository,
    T: SecretRepository,
{
    user_repository: R,
    profile_service: Arc<ProfileService<S>>,
    secret_service: Arc<SecretService<T>>,
}

impl<R, S, T> UserService<R, S, T>
where
    R: UserRepository + BaseRepository,
    S: ProfileRepository,
    T: SecretRepository,
{
    pub fn new(
        user_repository: R,
        profile_service: Arc<ProfileService<S>>,
        secret_service: Arc<SecretService<T>>,
    ) -> Self {
        Self {
            user_repository,
            profile_service,
            secret_service,
        }
    }

    pub async fn register(&self, name: &str, email: &str, pwd: &[u8]) -> Result<User> {
        let mut tx = self.user_repository.begin_tx().await?;

        let user = self.create_tx(&mut tx, name).await?;

        self.profile_service
            .create_tx(&mut tx, &user.id, email)
            .await?;

        self.secret_service
            .create_tx(&mut tx, &user.id, pwd)
            .await?;

        tx.commit().await?;

        Ok(user)
    }

    pub async fn create_tx(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        name: &str,
    ) -> Result<User> {
        if !USERNAME_REGEX.is_match(name) {
            return Err(Error::InvalidUsername(name.to_string()));
        }

        let user = self.user_repository.create_tx(tx, name).await?;

        Ok(user)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<User> {
        self.user_repository.find_by_name(name).await
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<User> {
        self.user_repository.find_one(id).await
    }

    pub async fn fetch_profile(&self, id: &Uuid) -> Result<Profile> {
        self.profile_service.find_by_user_id(id).await
    }
}
