use crate::model::User;
use anyhow::{Error, Result};
use argon2::{self, Config};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, errors::ErrorKind as JwtErrorKind};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").unwrap();
}

/// JWT Claims for a User Token
#[derive(Deserialize, Serialize)]
pub struct TokenClaims {
    pub user_id: Uuid,
}

impl From<User> for TokenClaims {
    fn from(user: User) -> Self {
        TokenClaims { user_id: user.id }
    }
}

/// MSEND JWT Manager
#[derive(Deserialize, Serialize)]
pub struct Jwt {
    pub token: String,
}

impl Jwt {
    /// Signs a JWT for the provided `User` where the
    /// `TokenClaims` are written for the same `User`
    pub fn from_user(user: &User) -> Result<Self> {
        let user = user.clone().to_owned();
        let token_claims = TokenClaims::from(user);

        Jwt::sign_jwt(&token_claims)
    }

    /// Signs a JWT with the provided `TokenClaims`
    fn sign_jwt(claims: &TokenClaims) -> Result<Self> {
        match encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
        ) {
            Ok(t) => Ok(Jwt { token: t }),
            Err(error) => Err(Error::msg(error.to_string())),
        }
    }

    /// Verifies the provided token to be a valid JWT and to be
    /// signed with the `JWT_SECRET`
    pub fn verify(token: &str) -> Result<TokenClaims> {
        match decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                JwtErrorKind::InvalidToken => Err(Error::msg("Token is invalid")),
                JwtErrorKind::InvalidIssuer => Err(Error::msg("Invalid issuer")),
                _ => Err(Error::msg("An error ocurred verifying the JWT"))
            },
        }
    }
}

/// Creates a hash from the provided password `bytes`
pub fn make_hash(password: &[u8]) -> Result<String> {
    let conf = Config::default();
    let salt = thread_rng().gen::<[u8; 32]>();

    match argon2::hash_encoded(password, &salt, &conf) {
        Ok(hash) => Ok(hash),
        Err(err) => Err(Error::msg(err.to_string())),
    }
}

/// Given a `hash` and the `password` validates
/// the `hash` is compatible with the `password`
pub fn verify_hash(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}
