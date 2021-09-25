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
	#[sqlx(rename = "owner_id")]
	owner: Uuid,
	name: String,
	description: Option<String>,
	visibility: Visibility,
}

//Database accions separate in other impl block for better legibility
impl Record {
	//async fn read_all_by_owner(owner_id: Uuid, pool: &Pool) {}
	//async fn read_all_public() {}
	//async fn read_all_private() {
	//async fn update(id: Uuid)
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
		.bind(self.owner)
		.bind(self.name)
		.bind(self.description)
		.bind(self.visibility)
		.execute(pool)
		.await
	}

	async fn select_all(pool: &Pool) -> sqlx::Result<Vec<Record>> {
		sqlx::query_as::<_, Record>(
			"
			SELECT * FROM git.record;
		",
		)
		.fetch_all(pool)
		.await
	}

	async fn select_by_id(id: Uuid, pool: &Pool) -> sqlx::Result<Record> {
		sqlx::query_as::<_, Record>(
			"
			SELECT * FROM git.record WHERE id = $1;
		",
		)
		.bind(id)
		.fetch_one(pool)
		.await
	}

	async fn delete(id: Uuid, pool: &Pool) -> sqlx::Result<PgQueryResult> {
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
		owner: Uuid,
		name: String,
		description: Option<String>,
		visibility: Visibility,
	) -> Record {
		Record {
			id: Uuid::new_v4(),
			owner,
			name,
			description,
			visibility,
		}
	}

	fn new_from_request(owner: Uuid, request: RecordRequest) -> Record {
		Record {
			id: Uuid::new_v4(),
			owner,
			name: request.name,
			description: request.description,
			visibility: request.visibility,
		}
	}

	pub async fn create(pool: &Pool, owner: Uuid, request: RecordRequest) -> Result<(), ()> {
		let res = Record::new_from_request(owner, request).insert(pool).await;
		println!("{:?}", res);
		match res {
			Ok(_) => Ok(()),
			Err(_) => Err(()),
		}
	}

	pub async fn get_all(pool: &Pool) -> Option<Vec<Record>> {
		match Record::select_all(pool).await {
			Ok(res) => res.into(),
			Err(_) => None,
		}
	}
}

#[derive(InputObject)]
pub struct RecordRequest {
	name: String,
	description: Option<String>,
	visibility: Visibility,
}
