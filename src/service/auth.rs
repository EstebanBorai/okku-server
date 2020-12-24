use argon2::{self, Config};
use http_auth_basic::Credentials;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use warp::http::StatusCode;
use warp::hyper::Body;

use crate::database::DbPool;
use crate::error::AppError;
use crate::model::{secret::Secret, user::User};

use super::user::UserService;

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").unwrap();
}

#[derive(Clone)]
pub struct AuthService {
    db_conn: DbPool,
    user_service: Arc<UserService>,
}

#[derive(Deserialize)]
pub struct UserRegister {
    pub name: String,
    pub password: String,
}

/// JWT Claims for a User token
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: u128,
}

/// Token payload sent to the client when authentication
/// is successful on either signup or login processes
#[derive(Deserialize, Serialize)]
pub struct Token {
    pub token: String,
}

impl AuthService {
    pub fn new(db_conn: DbPool, user_service: Arc<UserService>) -> Self {
        Self {
            db_conn,
            user_service,
        }
    }

    pub async fn signup(&self, user_register: UserRegister) -> Result<User, AppError> {
        let username = user_register.name.clone();
        let password = user_register.password.clone();

        if self.user_service.get_user_by_name(&username).await.is_ok() {
            return Err(AppError::UsernameTaken(username));
        }

        let user = self
            .user_service
            .create_user(&username)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let password_hash = &self.make_hash(password.as_bytes())?;

        self.store_user_secret(&user, password_hash).await?;

        Ok(user)
    }

    pub async fn login(&self, authorization_header: &str) -> Result<Token, AppError> {
        let credentials = Credentials::from_header(authorization_header.to_string())
            .map_err(|e| AppError::InvalidBasicAuthHeader(e.to_string()))?;
        let (user, secret) = self
            .user_service
            .get_user_and_secret(&credentials.user_id)
            .await?;

        if self.verify_hash(&secret.hash, credentials.password.as_bytes()) {
            return Ok(self.sign_jwt_token(&user)?);
        }

        Err(AppError::InvalidCredentials)
    }

    /// Creates a hash for the prodided password
    pub fn make_hash(&self, password: &[u8]) -> Result<String, AppError> {
        let conf = Config::default();
        let salt = thread_rng().gen::<[u8; 32]>();
        let hash = argon2::hash_encoded(password, &salt, &conf);

        hash.map_err(|e| AppError::UnexpectedServerError(e.to_string()))
    }

    async fn store_user_secret(&self, user: &User, hash: &str) -> Result<Secret, AppError> {
        let secret: Secret =
            sqlx::query_as("INSERT INTO secrets(hash, user_id) VALUES ($1, $2) RETURNING *")
                .bind(hash)
                .bind(user.id)
                .fetch_one(&self.db_conn)
                .await?;

        Ok(secret)
    }

    /// Given a `hash` and the `password` validates
    /// the `hash` is compatible with the `password`
    pub fn verify_hash(&self, hash: &str, password: &[u8]) -> bool {
        argon2::verify_encoded(hash, password).unwrap_or(false)
    }

    /// Signs a JWT token with the provided claims (`Claims`)
    pub fn sign_jwt_token(&self, user: &User) -> Result<Token, AppError> {
        let one_day_ms = Duration::from_secs(60 * 60 * 24).as_millis();
        let claims = Claims {
            user_id: user.id,
            exp: AuthService::timestamp_now() + one_day_ms,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
        )
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(Token { token })
    }

    /// Verifies a JWT and retrieve the `Claims` stored on
    /// it if valid
    pub fn verify_jwt_token(token: &str) -> Result<Claims, AppError> {
        let decode_result = decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::UnexpectedServerError(e.to_string()))?;

        Ok(decode_result.claims)
    }

    pub fn timestamp_now() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }
}

impl warp::reply::Reply for Claims {
    fn into_response(self) -> warp::reply::Response {
        let builder = warp::http::Response::builder().status(StatusCode::OK);

        builder
            .body(Body::from(serde_json::to_string(&self).unwrap()))
            .unwrap()
    }
}
