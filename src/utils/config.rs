use error::Result;
use std::env;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
	#[structopt(short, long, default_value = "none")]
	migrations: String,

	#[structopt(short, long)]
	data_example: bool,

	#[structopt(short, long, default_value = ".env")]
	env_file: PathBuf,
}

pub struct Config;

impl Config {
	pub fn run() -> Result<()> {
		let args = Args::from_args();
		Config::set_environment_variables(&args.env_file);
		match env::var("ENVIRONMENT") {
			Ok(var) => match var.as_str() {
				"prod" | "production" => {
					env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
				}
				"dev" | "development" => {
					env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
				}
				"test" | "testing" => {
					env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
				}
				_ => {
					env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
				}
			},
			Err(_) => {
				env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
			}
		}
		env_logger::init();
		Ok(())
	}

	fn set_environment_variables(path: &PathBuf) {
		let file = fs::read_to_string(path).unwrap();
		for line in file.lines() {
			let var: Vec<&str> = line.split('=').collect();
			env::set_var(var[0], var[1]);
		}
	}

	pub fn url() -> String {
		format!(
			"{}:{}",
			env::var("HOST").expect("host error"),
			env::var("PORT").expect("port error")
		)
	}

	pub fn client_path() -> String {
		env::var("CLIENT_PATH").unwrap()
	}
}

#[cfg(test)]
mod basics {
	#[test]
	fn minimal_works_with_repo() {
		todo!();
	}
}

/*struct Environment {
	environment: String,
	host: String,
	port: String,
	psql_host: String,
	psql_port: String,
	psql_user: String,
	psql_password: String,
	psql_default_schema: String,
	psql_default_database: String,
	git_root_dir: String,
	git_storage_dir: String,
	client_path: String,
	secret_key: String,
}

impl Default for Environment {
	fn default() -> Self {
		Self {
			environment: String::from("dev"),
			host: String::from("localhost"),
			port: String::from("9000"),
			psql_host: String::from("localhost"),
			psql_port: String::from("5432"),
			psql_user: String::from("dev"),
			psql_password: String::from("password123"),
			psql_default_schema: String::from("git"),
			psql_default_database: String::from("git_server_dev"),
			git_root_dir: String::from("/git"),
			git_storage_dir: String::from("/git/storage"),
			client_path: String::from("../pine-client"),
			secret_key: String::from("a_secure_secret_key"),
		}
	}
}*/
