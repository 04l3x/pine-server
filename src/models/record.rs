use git::Repo;

use crate::error::{BackendError, Result};
use crate::utils::database::Pool;
use async_graphql::{Enum, InputObject, SimpleObject};
use sqlx;
use sqlx::{
	postgres::{PgQueryResult, PgRow},
	FromRow, Row, Type,
};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Enum, Type)]
pub enum Visibility {
	Public,
	Private,
}

#[derive(Clone, Debug, FromRow, SimpleObject)]
pub struct Record {
	id: Uuid,
	owner_id: Uuid,
	name: String,
	description: Option<String>,
	visibility: Visibility,
}

//Database accions separate in other impl block for better legibility
impl Record {
	//async fn read_all_by_owner(owner_id: Uuid, pool: &Pool) {}
	//async fn read_all_private() {

	async fn read_path_info(
		pool: &Pool,
		repo_name: String,
		username: String,
	) -> sqlx::Result<PgRow> {
		sqlx::query(
			"
			SELECT
				rec.id AS id, users.id AS owner_id
			FROM
				git.record AS rec
			JOIN
				public.users AS users
			ON
				rec.owner_id = users.id
			WHERE
				name = $1 AND username = $2;
			",
		)
		.bind(repo_name)
		.bind(username)
		.fetch_one(pool)
		.await
	}

	async fn read_all_public(pool: &Pool) -> sqlx::Result<Vec<Record>> {
		sqlx::query_as::<_, Record>(
			"
			SELECT * FROM git.record WHERE visibility = 'Public';
		",
		)
		.fetch_all(pool)
		.await
	}

	async fn insert(self, pool: &Pool) -> sqlx::Result<PgQueryResult> {
		println!("{:?}", self.visibility);
		sqlx::query(
			"
			INSERT INTO git.record 
			(id, owner_id, name, description, visibility) 
			VALUES ($1, $2, $3, $4, $5);
		",
		)
		.bind(self.id)
		.bind(self.owner_id)
		.bind(self.name)
		.bind(self.description)
		.bind(self.visibility)
		.execute(pool)
		.await
	}

	async fn read_all(pool: &Pool) -> sqlx::Result<Vec<Record>> {
		sqlx::query_as::<_, Record>(
			"
			SELECT * FROM git.record;
		",
		)
		.fetch_all(pool)
		.await
	}

	async fn read_by_id(id: Uuid, pool: &Pool) -> sqlx::Result<Record> {
		sqlx::query_as::<_, Record>(
			"
			SELECT * FROM git.record WHERE id = $1;
		",
		)
		.bind(id)
		.fetch_one(pool)
		.await
	}

	async fn delete_by_id(id: Uuid, pool: &Pool) -> sqlx::Result<PgQueryResult> {
		sqlx::query(
			"
			Delete git.record WHERE id = $1;
		",
		)
		.bind(id)
		.execute(pool)
		.await
	}
}

impl Record {
	fn new(
		owner_id: Uuid,
		name: String,
		description: Option<String>,
		visibility: Visibility,
	) -> Record {
		Record {
			id: Uuid::new_v4(),
			owner_id,
			name,
			description,
			visibility,
		}
	}

	fn new_from_request(owner_id: Uuid, request: NewRepository) -> Record {
		Record {
			id: Uuid::new_v4(),
			owner_id,
			name: request.name,
			description: request.description,
			visibility: request.visibility,
		}
	}

	pub async fn initialize(pool: &Pool, owner_id: Uuid, request: NewRepository) -> Result<()> {
		let new_repo = Record::new_from_request(owner_id.clone(), request.clone());
		let repo_id = new_repo.id;

		match new_repo.insert(pool).await {
			Ok(_) => {
				Repo::new_bare(repo_id, owner_id);
				Ok(())
			}
			Err(e) => Err(Box::new(e)),
		}
	}

	pub async fn get_all_public(pool: &Pool) -> Option<Vec<Record>> {
		match Record::read_all_public(pool).await {
			Ok(res) => res.into(),
			Err(_) => None,
		}
	}

	pub async fn repo_path<'a>(
		pool: &Pool,
		repo_name: String,
		username: String,
	) -> Result<PathBuf> {
		match Record::read_path_info(pool, repo_name, username).await {
			Ok(pg_row) => {
				let root = std::env::var("GIT_ROOT_DIR").expect("no git root dir var");
				let owner = pg_row.get::<Uuid, _>("owner_id");
				let repo = pg_row.get::<Uuid, _>("id");
				let path = format!("{}/{}/{}.git", root, owner, repo);
				Ok(PathBuf::from(path))
			}
			Err(e) => Err(Box::new(e)),
		}
	}
}

#[derive(Clone, InputObject)]
pub struct NewRepository {
	name: String,
	description: Option<String>,
	visibility: Visibility,
}
