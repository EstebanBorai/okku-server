use crate::database::{get_db_conn, Row};
use crate::model::User;
use crate::server::http_response::HttpResponse;
use anyhow::{Error, Result};
use argon2::{self, Config};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use uuid::Uuid;
use warp::http::StatusCode;

#[derive(Deserialize)]
pub struct UserRegister {
    pub name: String,
    pub password: String,
}

pub async fn signup(
    user_register: UserRegister,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    match get_db_conn().await {
        Ok(db_conn) => {
            if let Ok(_) = db_conn
                .query_one(
                    "SELECT * FROM users WHERE users.name = $1 LIMIT 1",
                    &[&user_register.name],
                )
                .await
            {
                return Ok(HttpResponse::new(
                    "Username is taken",
                    StatusCode::BAD_REQUEST,
                ));
            }

            let user_insert_rows: Row = db_conn
                .query_one(
                    "INSERT INTO users(name) VALUES ($1) RETURNING *",
                    &[&user_register.name],
                )
                .await
                .unwrap();

            let user_id: Uuid = user_insert_rows.get(0);
            let hash = make_hash(user_register.password.as_bytes()).unwrap();

            db_conn
                .query(
                    "INSERT INTO secrets(hash, user_id) VALUES ($1, $2)",
                    &[&hash, &user_id],
                )
                .await
                .unwrap();

            let created_user = User {
                id: user_id,
                name: user_insert_rows.get(1),
            };

            Ok(HttpResponse::with_payload(
                created_user,
                StatusCode::CREATED,
            ))
        }
        Err(err) => Ok(HttpResponse::new(
            &format!("An error ocurred!\n{}", err.to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn login() -> Result<impl warp::Reply, std::convert::Infallible> {
    tokio::time::delay_for(std::time::Duration::from_secs(10)).await;
    Ok(format!("I waited {} seconds!", 10))
}

fn make_hash(password: &[u8]) -> Result<String> {
    let conf = Config::default();
    let salt = thread_rng().gen::<[u8; 32]>();

    match argon2::hash_encoded(password, &salt, &conf) {
        Ok(hash) => Ok(hash),
        Err(err) => Err(Error::msg(err.to_string())),
    }
}
