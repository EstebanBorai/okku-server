use serde::Serialize;

use crate::database::get_db_pool;

#[derive(Serialize)]
struct HealthCheck {
    db_ok: bool,
}

/// Executes a health check on system services and
/// retrieve its results
pub async fn check() -> Result<impl warp::Reply, std::convert::Infallible> {
    let pool = get_db_pool().await.expect("Unable to get database pool");
    let row: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("Unable to query database");

    Ok(warp::reply::json(&HealthCheck { db_ok: true }))
}
