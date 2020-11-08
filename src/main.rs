mod model;
mod proto;
mod client;
mod error;
mod hub;
mod server;

#[tokio::main]
async fn main() {
  env_logger::init();

  let server = server::Server::new(8080);

  server.run().await;
}
