use sqlx::postgres::PgPoolOptions;
use std::env;
use std::path::PathBuf;
use std::fs;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub async fn default_pool() ->  Pool {	
	PgPoolOptions::new()
		.max_connections(5)
		.connect( self::uri().as_str() )
		.await
		.expect("db error")
}

pub async fn pool_with_options( max_connections: u32 ) -> Pool {	
	PgPoolOptions::new()
		.max_connections( max_connections )
		.connect( self::uri().as_str() )
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

pub async fn initial_migrations() {
	let pool = pool_with_options(1).await;

	sqlx::query( 
		fs::read_to_string(
			PathBuf::from("sql/schema.up.sql")
		).unwrap()
		.as_str()
	)
	.execute(&pool)
	.await
	.unwrap();

	for file in fs::read_dir( PathBuf::from("sql") ).unwrap() {
		match &file {
			Ok(file) => {
				if file.path().to_str().expect("").contains(".up.sql") 
					&& !file.path().to_str().expect("").contains("schema.up.sql") {

					sqlx::query( 
						fs::read_to_string(
							file.path()
						).unwrap()
						.as_str()
					)
					.execute(&pool)
					.await
					.unwrap();
				}
			},
			Err(e) => panic!("{:?}",e),
		}
	}
}


