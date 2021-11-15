mod auth;
mod graphql;
mod models;
mod server;
mod utils;

use server::Server;
use utils::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	match Config::run() {
		_ => Server::start().await,
	}
}
