#[macro_use]
extern crate log;

use std::env;

mod client;
mod error;
mod hub;
mod model;
mod proto;
mod server;

#[tokio::main]
async fn main() {
    // this is temporal and I should change it to dotenv
    // or some config at some point in time
    env::set_var("RUST_LOG", "warn,info,error,debug");
    env_logger::init();

    let server = server::Server::new(8080);

    info!("Serving msend server on: ws://127.0.0.1:8080/");

    server.run().await;
}
