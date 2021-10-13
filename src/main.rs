mod auth;
mod error;
mod graphql;
mod models;
mod server;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	utils::config::Config::run();
	server::Server::start().await
}
