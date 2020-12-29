use argon2::{hash_encoded, verify_encoded, Config as Argon2Config};
use rand::{thread_rng, Rng};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::infrastructure::database::DbPool;
use crate::infrastructure::repository::secret::Repository;

use super::Secret;

pub struct SecretService {
    secret_repository: Repository,
}

impl SecretService {
    pub fn new(secret_repository: Repository) -> Self {
        Self { secret_repository }
    }

    pub async fn create(&self, pwd: &[u8], user_id: &Uuid) -> Result<Secret> {
        let hash = self.make_hash(pwd)?;
        let secret = self.secret_repository.create(&hash, user_id).await?;

        Ok(Secret::from(secret))
    }

    pub async fn validate(&self, pwd: &[u8], user_id: &Uuid) -> Result<bool> {
        let secret = self.secret_repository.find_by_user_id(user_id).await?;

        Ok(self.verify_hash(pwd, &secret.hash))
    }

    fn make_hash(&self, pwd: &[u8]) -> Result<String> {
        let config = Argon2Config::default();
        let salt = thread_rng().gen::<[u8; 32]>();
        let hash = hash_encoded(pwd, &salt, &config);

        hash.map_err(|e| Error::HashError(e.to_string()))
    }

    fn verify_hash(&self, pwd: &[u8], hash: &str) -> bool {
        verify_encoded(hash, pwd).unwrap_or(false)
    }
}

pub fn make_secret_service(db_pool: &'static DbPool) -> SecretService {
    SecretService::new(Repository::new(db_pool))
}
