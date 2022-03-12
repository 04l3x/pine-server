use crate::utils::database::Pool;
use error::Result;
use git::{repository::Repository as Repo, tree::Tree};
use sqlx;
use sqlx::{postgres::PgRow, Row};
use std::path::PathBuf;
use uuid::Uuid;

pub struct Repository {}

impl Repository {
	pub fn full_tree(path: PathBuf) -> Result<Tree> {
		Repo::get_tree(path)
	}
}

impl Repository {
	async fn read_path_info(pool: &Pool, id: &Uuid) -> sqlx::Result<PgRow> {
		sqlx::query(
			"
			SELECT
				id AS repo_id, owner_id
			FROM
				git.record
			WHERE
				id = $1;
			",
		)
		.bind(id)
		.fetch_one(pool)
		.await
	}

	pub async fn path_by_uuid(pool: &Pool, id: &Uuid) -> Result<PathBuf> {
		match Repository::read_path_info(pool, id).await {
			Ok(pg_row) => {
				let root = std::env::var("GIT_ROOT_DIR").expect("no git root dir var");
				let owner = pg_row.get::<Uuid, _>("owner_id");
				let repo = pg_row.get::<Uuid, _>("repo_id");
				let path = format!("{}/{}/{}.git", root, owner, repo);
				Ok(PathBuf::from(path))
			}
			Err(e) => Err(Box::new(e)),
		}
	}
}
