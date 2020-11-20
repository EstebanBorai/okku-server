#[macro_use]
extern crate log;

use anyhow::Result;
use std::env;

mod database;
mod model;
mod proto;
mod server;
mod utils;
mod client;
mod hub;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()
        .ok()
        .expect("Unable to find .env file. Create one based on the .env.sample");

    env_logger::init();

    let port = env::var("PORT")
        .expect("Missing PORT environment variable")
        .parse::<u16>()
        .expect("Invalid PORT value, expected u16");

    database::init_db()
        .await
        .expect("Unable to initialize database");

    let server = server::Server::new(port);

    info!(
        "{}",
        format!("Server listening on: ws://127.0.0.1:{}", port)
    );

    server.run().await;

    Ok(())
}
