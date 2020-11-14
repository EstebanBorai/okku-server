use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::env;
use std::str::FromStr;
use std::time::Duration;
use std::fs::read_to_string;
use tokio_postgres::{Config, Error, NoTls};

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

type Result<T> = std::result::Result<T, dyn std::error::Error>;

pub type DbConn = Connection<PgConnectionManager<NoTls>>;
pub type DbPool = Pool<PgConnectionManager<NoTls>>;

/// Builds a Postgres connection pool and defines configurations such
/// as minimum and maximum open connections, and connection timeout
///
/// This function relies on environment variables from the .env file:
///
/// * `POSTGRES_USER`
/// * `POSTGRES_PASSWORD`
/// * `POSTGRES_DB`
pub fn create_pool() -> std::result::Result<DbPool, mobc::Error<Error>> {
    let db_username = env::var("POSTGRES_USER").expect("Missing POSTGRES_USER env variable");
    let db_password =
        env::var("POSTGRES_PASSWORD").expect("Missing POSTGRES_PASSWORD env variable");
    let database = env::var("POSTGRES_DB").expect("Missing POSTGRES_DB env variable");

    let config = Config::from_str(&format!(
        "postgres://{username}:{password}@127.0.0.1:5432/{database}",
        username = db_username,
        password = db_password,
        database = database
    ))?;

    let manager = PgConnectionManager::new(config, NoTls);

    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

/// Gathers a database connection from the database pool
pub async fn get_db_conn(db_pool: &DbPool) -> Result<DbConn> {
    db_pool.get().await?
}

pub async fn init_db(db_pool: &DbPool) -> Result<()> {
    let init_file = read_to_string("./init.sql")?;
    let conn = get_db_conn(db_pool).await?;

    conn.batch_execute(init_file.as_str())
        .await
        .unwrap();

    Ok(())
}
