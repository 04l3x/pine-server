use sqlx::postgres::PgPoolOptions;
use std::env;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub async fn default_pool() -> Pool {
	PgPoolOptions::new()
		.max_connections(5)
		.connect(uri().as_str())
		.await
		.expect("db error")
}

pub async fn pool_with_options(max_connections: u32) -> Pool {
	PgPoolOptions::new()
		.max_connections(max_connections)
		.connect(uri().as_str())
		.await
		.expect("db error")
}

fn uri() -> String {
	format!(
		"postgres://{}:{}@{}:{}/{}",
		env::var("PSQL_USER").expect("error NOT PSQL_USER VARIABLE"),
		env::var("PSQL_PASSWORD").expect("error NOT PSQL_PASSWORD VARIABLE"),
		env::var("PSQL_HOST").expect("error NOT HOST PSQL_HOST VARIABLE"),
		env::var("PSQL_PORT").expect("error NOT HOST PSQL_PORT VARIABLE"),
		env::var("PSQL_DEFAULT_DATABASE").expect("error NOT PSQL_DEFAULT_DATABASE VARIABLE")
	)
}

