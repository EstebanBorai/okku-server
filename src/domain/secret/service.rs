use argon2::{hash_encoded, verify_encoded, Config as Argon2Config};
use rand::{thread_rng, Rng};
use sqlx::postgres::Postgres;
use sqlx::Transaction;
use uuid::Uuid;

use crate::error::{Error, Result};

use super::{Secret, SecretRepository};

pub struct SecretService<R>
where
    R: SecretRepository,
{
    secret_repository: R,
}
impl<R> SecretService<R>
where
    R: SecretRepository,
{
    pub fn new(secret_repository: R) -> Self {
        Self { secret_repository }
    }

    pub async fn create_tx<'a>(
        &'a self,
        tx: &mut Transaction<'static, Postgres>,
        user_id: &Uuid,
        pwd: &[u8],
    ) -> Result<Secret> {
        let hash = self.make_hash(pwd)?;
        let secret = self.secret_repository.create_tx(tx, user_id, &hash).await?;

        Ok(secret.into())
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
