use std::env;
use std::fs;
use std::path::PathBuf;

pub struct Config;

impl Config {
    pub async fn run() {
        Config::set_environment_variables();
    }

    fn set_environment_variables() {
        let file = fs::read_to_string(PathBuf::from(".env")).unwrap();
        for line in file.lines() {
            let var: Vec<&str> = line.split('=').collect();
            env::set_var(var[0], var[1]);
        }
    }
}

#[cfg(test)]
mod basics {
    #[test]
    fn minimal_works_with_repo() {
        todo!();
    }
}