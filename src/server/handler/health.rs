use crate::database::get_db_conn;
use serde::Serialize;

#[derive(Serialize)]
struct HealthCheck {
    db_ok: bool,
}

/// Executes a health check on system services and
/// retrieve its results
pub async fn check() -> Result<impl warp::Reply, std::convert::Infallible> {
    let db_conn = get_db_conn().await.unwrap();

    db_conn.execute("SELECT 1", &[]).await.unwrap();

    Ok(warp::reply::json(&HealthCheck { db_ok: true }))
}
