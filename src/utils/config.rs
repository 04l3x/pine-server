use std::env;
use std::fs;
use std::path::PathBuf;

pub struct Config;
//TODO: read options flags from command line
impl Config {
	pub fn run() {
		env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
		env_logger::init();
		//TODO: set rust log for different environments
		//TODO: verify system dependencies when init ?
		Config::set_environment_variables();
	}

	fn set_environment_variables() {
		let file = fs::read_to_string(PathBuf::from(".env")).unwrap();
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
