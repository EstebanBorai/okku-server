#[macro_use]
extern crate log;

use std::env;

mod database;
mod client;
mod error;
mod hub;
mod model;
mod proto;
mod server;

#[tokio::main]
async fn main() {
    dotenv::dotenv()
        .ok()
        .expect("Unable to find .env file. Create one based on the .env.sample");

        env_logger::init();

    let port = env::var("PORT")
        .expect("Missing PORT environment variable")
        .parse::<u16>()
        .expect("Invalid PORT value, expected u16");

    let db_pool = database::create_pool().expect("Unable to create database pool");

    database::init_db(&db_pool).await.expect("Unable to initialize database");

    let server = server::Server::new(port);

    info!(
        "{}",
        format!("Server listening on: ws://127.0.0.1:{}", port)
    );

    server.run().await;
}
