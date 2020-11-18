pub async fn signup() -> Result<impl warp::Reply, std::convert::Infallible> {
  tokio::time::delay_for(std::time::Duration::from_secs(10)).await;
  Ok(format!("I waited {} seconds!", 10))
}

pub async fn login() -> Result<impl warp::Reply, std::convert::Infallible> {
  tokio::time::delay_for(std::time::Duration::from_secs(10)).await;
  Ok(format!("I waited {} seconds!", 10))
}
