#[macro_use]
extern crate log;

mod application;
mod domain;
mod error;
mod infrastructure;
mod server;

#[tokio::main]
async fn main() {
    dotenv::dotenv()
        .ok()
        .expect("Unable to find .env file. Create one based on the .env.sample");

    env_logger::init();

    let http_server = server::Http::new(3000_u16);

    http_server.serve().await;
}
