mod auth;
mod config;
mod database;
mod graphql;
mod models;
mod server;

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    config::Config::run().await;
    env_logger::init();
    server::Server::start().await
}
