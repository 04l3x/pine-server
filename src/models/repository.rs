//use crate::error::{Result, BackendError};
use git2;
use std::{env, path::Path};
use uuid::Uuid;

pub struct Repository;

impl Repository {
	pub fn new_bare(name: String, owner_id: Uuid) -> git2::Repository {
		let path = format!(
			"{}/{}/{}.git",
			env::var("GIT_ROOT_DIR").expect("not git root var"),
			owner_id.to_string(),
			name
		);
		git2::Repository::init_bare(&Path::new(&path)).unwrap()
	}
}
