use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::application::service::secret::SecretService;
use crate::error::{Error, Result};

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").unwrap();
}

pub struct AuthService {
    secret_service: Arc<SecretService>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: u128,
}

impl AuthService {
    pub fn new(secret_service: Arc<SecretService>) -> Self {
        Self { secret_service }
    }

    pub async fn authenticate(&self, pwd: &[u8], user_id: &Uuid) -> Result<String> {
        let is_valid = self.secret_service.validate(pwd, user_id).await?;

        if is_valid {
            return Ok(self.sign_token(user_id)?);
        }

        Err(Error::InvalidCredentials)
    }

    pub async fn verify_token(&self, token: &str) -> Result<Claims> {
        let decode_result = decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| Error::JWTError(e.to_string()))?;

        Ok(decode_result.claims)
    }

    fn sign_token(&self, user_id: &Uuid) -> Result<String> {
        let claims = Claims {
            user_id: user_id.clone(),
            exp: self.unix_now()? + 86400000_u128,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
        )
        .map_err(|e| Error::JWTError(e.to_string()))
    }

    fn unix_now(&self) -> Result<u128> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(Error::from)?;

        Ok(now.as_millis())
    }
}

pub fn make_auth_service(secret_service: Arc<SecretService>) -> AuthService {
    AuthService::new(secret_service)
}
