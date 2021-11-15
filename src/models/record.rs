use crate::graphql::info::{Info, InfoFactory};
use crate::utils::database::Pool;
use async_graphql::{Enum, InputObject, SimpleObject};
use error::Result;
use git::Repo;
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

#[derive(Clone, Debug, FromRow)]
pub struct Record {
	id: Uuid,
	owner_id: Uuid,
	name: String,
	description: Option<String>,
	visibility: Visibility,
}

#[derive(Clone, Debug, FromRow, SimpleObject)]
pub struct RecordInfo {
	id: Uuid,
	owner_id: Uuid,
	name: String,
	description: Option<String>,
	visibility: Visibility,
	#[sqlx(rename = "username")]
	owner_name: String,
	#[sqlx(rename = "avatar")]
	owner_avatar: String,
}

//Database accions separate in other impl block for better legibility
impl Record {
	async fn size_of_public_record(pool: &Pool) -> sqlx::Result<i64> {
		match sqlx::query(
			"
			SELECT count(id) FROM git.record WHERE visibility = 'Public';
		",
		)
		.fetch_one(pool)
		.await
		{
			Ok(pg_row) => Ok(pg_row.get::<i64, _>("count")),
			Err(e) => Err(e),
		}
	}

	async fn size_of_public_record_filter_by_name(pool: &Pool, query: String) -> sqlx::Result<i64> {
		match sqlx::query(
			"
			SELECT count(id) FROM git.record 
			WHERE name ~ $1 AND visibility = 'Public';
		",
		)
		.bind(query)
		.fetch_one(pool)
		.await
		{
			Ok(pg_row) => Ok(pg_row.get::<i64, _>("count")),
			Err(e) => Err(e),
		}
	}

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
			INNER JOIN
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

	async fn insert(self, pool: &Pool) -> sqlx::Result<PgQueryResult> {
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

	async fn read_public_by_page(pool: &Pool, page: i32) -> sqlx::Result<Vec<RecordInfo>> {
		sqlx::query_as::<_, RecordInfo>(
			"
			SELECT
				record.id,
				owner_id,
				name,
				description,
				visibility,
				username,
				avatar
			FROM git.record
			INNER JOIN users ON record.owner_id = users.id
			WHERE visibility = 'Public'
			ORDER BY id LIMIT 15 OFFSET $1;
		",
		)
		.bind((page - 1) * 15)
		.fetch_all(pool)
		.await
	}

	async fn read_public_by_page_filter_by_name(
		pool: &Pool,
		page: i32,
		query: String,
	) -> sqlx::Result<Vec<RecordInfo>> {
		sqlx::query_as::<_, RecordInfo>(
			"
			SELECT
				record.id,
				owner_id,
				name,
				description,
				visibility,
				username,
				avatar
			FROM git.record
			INNER JOIN users ON record.owner_id = users.id
			WHERE name ~ $1 AND visibility = 'Public'
			ORDER BY id LIMIT 15 OFFSET $2;
		",
		)
		.bind(query)
		.bind((page - 1) * 15)
		.fetch_all(pool)
		.await
	}

	async fn read_public_by_id(pool: &Pool, id: Uuid) -> sqlx::Result<RecordInfo> {
		sqlx::query_as::<_, RecordInfo>(
			"
			SELECT
				record.id,
				owner_id,
				name,
				description,
				visibility,
				username,
				avatar
			FROM git.record
			INNER JOIN users ON record.owner_id = users.id
			WHERE record.id = $1 AND visibility = 'Public';
		",
		)
		.bind(id)
		.fetch_one(pool)
		.await
	}

	async fn read_owned_by_id(pool: &Pool, id: Uuid, owner_id: Uuid) -> sqlx::Result<RecordInfo> {
		sqlx::query_as::<_, RecordInfo>(
			"
			SELECT
				record.id,
				owner_id,
				name,
				description,
				visibility,
				username,
				avatar
			FROM git.record
			INNER JOIN users ON record.owner_id = users.id
			WHERE record.id = $1 AND owner_id = $2 ;
		",
		)
		.bind(id)
		.bind(owner_id)
		.fetch_one(pool)
		.await
	}
}

impl Record {
	fn _new(
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

	pub async fn initialize(pool: &Pool, owner_id: Uuid, request: NewRepository) -> Result<Uuid> {
		let new_repo = Record::new_from_request(owner_id.clone(), request.clone());
		let repo_id = new_repo.id;

		match new_repo.insert(pool).await {
			Ok(_) => {
				Repo::new_bare(repo_id.clone(), owner_id);
				Ok(repo_id)
			}
			Err(e) => Err(Box::new(e)),
		}
	}

	pub async fn public_record_paginated_with_name_filter(
		pool: &Pool,
		page: i32,
		query: String,
	) -> Result<Records> {
		match Record::read_public_by_page_filter_by_name(pool, page, query.clone()).await {
			Ok(results) => match Record::size_of_public_record_filter_by_name(pool, query).await {
				Ok(count) => {
					let mut builder = RecordsBuilder::new();

					builder.set_values(
						InfoFactory::default().info(count as i32, page, 15),
						Some(results),
					);

					Ok(builder.build())
				}
				Err(e) => Err(Box::new(e)),
			},
			Err(e) => Err(Box::new(e)),
		}
	}

	pub async fn public_record_paginated(pool: &Pool, page: i32) -> Result<Records> {
		match Record::read_public_by_page(pool, page).await {
			Ok(results) => match Record::size_of_public_record(pool).await {
				Ok(count) => {
					let mut builder = RecordsBuilder::new();

					builder.set_values(
						InfoFactory::default().info(count as i32, page, 15),
						Some(results),
					);

					Ok(builder.build())
				}
				Err(e) => Err(Box::new(e)),
			},
			Err(e) => Err(Box::new(e)),
		}
	}

	pub async fn public_record_info_by_id(pool: &Pool, id: Uuid) -> Result<RecordInfo> {
		match Record::read_public_by_id(pool, id).await {
			Ok(result) => Ok(result),
			Err(e) => Err(Box::new(e)),
		}
	}

	//pub async fn get_owned_record_info_by_id(pool: &Pool, id: Uuid) -> Result<RecordInfo> {	}

	pub async fn repo_path(pool: &Pool, repo_name: String, username: String) -> Result<PathBuf> {
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

#[derive(Clone, Debug, SimpleObject)]
pub struct Records {
	info: Info,
	results: Option<Vec<RecordInfo>>,
}

impl Default for Records {
	fn default() -> Self {
		Self {
			info: Info::default(),
			results: None,
		}
	}
}

#[derive(Debug)]
pub struct RecordsBuilder {
	records: Records,
}

impl RecordsBuilder {
	pub fn new() -> RecordsBuilder {
		RecordsBuilder {
			records: Records::default(),
		}
	}

	fn set_info(&mut self, info: Info) {
		self.records.info = info;
	}

	fn set_results(&mut self, results: Option<Vec<RecordInfo>>) {
		self.records.results = results;
	}

	fn set_values(&mut self, info: Info, results: Option<Vec<RecordInfo>>) {
		self.set_info(info);
		self.set_results(results);
	}

	pub fn build(self) -> Records {
		self.records
	}
}

#[derive(Clone, InputObject)]
pub struct NewRepository {
	name: String,
	description: Option<String>,
	visibility: Visibility,
}

#[derive(Clone, InputObject)]
pub struct RecordFilter {
	id: Option<Uuid>,
	name: Option<String>,
	description: Option<String>,
	visibility: Option<Visibility>,
}
