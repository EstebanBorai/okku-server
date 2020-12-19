use lazy_static::lazy_static;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::env;
use std::fs::canonicalize;
use std::fs::read_to_string;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, NoTls};

use crate::error::{AppError, Result};

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

pub type DbConn = Connection<PgConnectionManager<NoTls>>;
pub type DbPool = Pool<PgConnectionManager<NoTls>>;

pub type Row = tokio_postgres::Row;

lazy_static! {
    static ref DB_POOL: DbPool = create_pool().unwrap();
}

/// Builds a Postgres connection pool and defines configurations such
/// as minimum and maximum open connections, and connection timeout
///
/// This function relies on environment variables from the .env file:
///
/// * `POSTGRES_USER`
/// * `POSTGRES_PASSWORD`
/// * `POSTGRES_DB`
pub fn create_pool() -> Result<DbPool> {
    let db_username = env::var("POSTGRES_USER").expect("Missing POSTGRES_USER env variable");
    let db_password =
        env::var("POSTGRES_PASSWORD").expect("Missing POSTGRES_PASSWORD env variable");
    let database = env::var("POSTGRES_DB").expect("Missing POSTGRES_DB env variable");

    let config = Config::from_str(&format!(
        "postgres://{username}:{password}@127.0.0.1:5432/{database}",
        username = db_username,
        password = db_password,
        database = database
    ))
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let manager = PgConnectionManager::new(config, NoTls);

    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

/// Gathers a database connection from the database pool
///
/// Read more on `tokio_postgres` for details on the API
/// avaiable on the underlying `DbConn` type
///
/// [tokio_postgres](https://docs.rs/tokio-postgres/0.5.5/tokio_postgres/index.html)
pub async fn get_db_conn() -> Result<DbConn> {
    Ok(DB_POOL
        .get()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?)
}

/// Initializes the database.
/// First reads the "init.sql" query available
/// on the `src/database/init.sql` file.
///
/// Then gets a connection from the Database Connection Pool
/// and executes the `init.sql` query.
pub async fn init_db() -> Result<()> {
    let init_query = read_to_string(
        canonicalize("./src/database/init.sql")
            .map_err(|e| AppError::DatabaseError(e.to_string()))?,
    )
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let conn = get_db_conn()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    conn.batch_execute(init_query.as_str())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}
