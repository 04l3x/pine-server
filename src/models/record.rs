use git::Repo;

use crate::error::{BackendError, Result};
use crate::utils::database::Pool;
use async_graphql::{Enum, InputObject, SimpleObject};
use sqlx;
use sqlx::{
	postgres::{PgQueryResult, PgRow},
	FromRow, Type,
};
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
	//async fn update(id: Uuid)

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
}

#[derive(Clone, InputObject)]
pub struct NewRepository {
	name: String,
	description: Option<String>,
	visibility: Visibility,
}
