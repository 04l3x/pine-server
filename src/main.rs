mod auth;
mod graphql;
mod models;
mod server;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	utils::config::Config::run().await;
	env_logger::init();
	server::Server::start().await
}
