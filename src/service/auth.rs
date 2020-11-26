use crate::database::{DbConn, Row};
use crate::model::{Secret, User};
use crate::service::UserService;
use anyhow::{Error, Result};
use argon2::{self, Config};
use http_auth_basic::Credentials;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").unwrap();
}

#[derive(Clone)]
pub struct AuthService {
    db_conn: Arc<DbConn>,
    user_service: Arc<UserService>,
}

#[derive(Deserialize)]
pub struct UserRegister {
    pub name: String,
    pub password: String,
}

/// JWT Claims for a User token
#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub user_id: Uuid,
}

/// Token payload sent to the client when authentication
/// is successful on either signup or login processes
#[derive(Deserialize, Serialize)]
pub struct Token {
    pub token: String,
}

impl AuthService {
    pub fn new(db_conn: Arc<DbConn>, user_service: UserService) -> Self {
        Self {
            db_conn,
            user_service: Arc::new(user_service),
        }
    }

    pub async fn signup(&self, user_register: UserRegister) -> Result<User> {
        let username = user_register.name.clone();
        let password = user_register.password.clone();

        if self.user_service.get_user_by_name(&username).await.is_ok() {
            return Err(Error::msg(format!("Username {} is taken", &username)));
        }

        let user = self
            .user_service
            .create_user(&username)
            .await
            .map_err(Error::from)?;

        let password_hash = &self.make_hash(password.as_bytes())?;

        self.store_user_secret(&user, password_hash).await?;

        Ok(user)
    }

    pub async fn login(&self, authorization_header: &str) -> Result<Token> {
        let credentials = Credentials::from_header(authorization_header.to_string())
            .map_err(|err| Error::msg(err.to_string()))?;
        let (user, secret) = self
            .user_service
            .get_user_with_secret(&credentials.user_id)
            .await?;

        if self.verify_hash(&secret.hash, credentials.password.as_bytes()) {
            return Ok(self.sign_jwt_token(&user)?);
        }

        Err(Error::msg("Invalid username/password"))
    }

    /// Creates a hash for the prodided password
    pub fn make_hash(&self, password: &[u8]) -> Result<String> {
        let conf = Config::default();
        let salt = thread_rng().gen::<[u8; 32]>();

        Ok(argon2::hash_encoded(password, &salt, &conf).map_err(Error::from)?)
    }

    async fn store_user_secret(&self, user: &User, hash: &str) -> Result<Secret> {
        let rows: Row = self
            .db_conn
            .query_one(
                "INSERT INTO secrets(hash, user_id) VALUES ($1, $2)",
                &[&hash, &user.id],
            )
            .await
            .map_err(Error::from)?;

        Ok(Secret {
            id: rows.get(0),
            hash: rows.get(1),
            user_id: rows.get(2),
        })
    }

    /// Given a `hash` and the `password` validates
    /// the `hash` is compatible with the `password`
    pub fn verify_hash(&self, hash: &str, password: &[u8]) -> bool {
        argon2::verify_encoded(hash, password).unwrap_or(false)
    }

    /// Signs a JWT token with the provided claims (`Claims`)
    pub fn sign_jwt_token(&self, user: &User) -> Result<Token> {
        let claims = Claims { user_id: user.id };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
        )
        .map_err(Error::from)?;

        Ok(Token { token })
    }

    /// Verifies a `Token` to have a valid sign and not to
    /// be out dated. If the provided `Token` is valid, returns
    /// the `Claims` for the token
    pub fn verify_jwt_token(&self, client_token: &Token) -> Result<Claims> {
        let token_data = decode::<Claims>(
            &client_token.token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(Error::from)?;

        Ok(token_data.claims)
    }
}
