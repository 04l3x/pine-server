use git2;
use std::path;

struct Repository;

impl Repository {
    fn get(path: path::PathBuf) -> git2::Repository {
        git2::Repository::open(path).unwrap()
    }
}
