use async_graphql::{Enum, InputObject, SimpleObject};

use crate::database::Pool;

use sqlx;
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Type, Enum, Copy, Eq, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, FromRow, SimpleObject)]
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
    async fn insert(self, pool: &Pool) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
        println!("{:?}", self.visibility);
        sqlx::query(
            "
			INSERT INTO git.record 
			(id, owner_id, name, description, visibility) 
			VALUES ($1, $2, $3, $4, $5);
		",
        )
        .bind(self.id)
        .bind(Uuid::parse_str("c2891e06-ff21-4717-a32b-b9db295db885").unwrap()) //FIXME: change default uuid for current user uuid
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

    async fn select_by_id(id: Uuid, pool: &Pool) -> sqlx::Result<sqlx::postgres::PgRow> {
        sqlx::query(
            "
			SELECT * FROM git.record WHERE id = $1;
		",
        )
        .bind(id)
        .fetch_one(pool)
        .await
    }

    async fn delete(id: Uuid, pool: &Pool) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
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

    pub async fn create(pool: &Pool, request: RecordRequest) -> Result<(), ()> {
        let res = Record::from(request).insert(pool).await;
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

impl From<RecordRequest> for Record {
    fn from(request: RecordRequest) -> Self {
        Record::new(
            Uuid::new_v4(),
            request.name,
            request.description,
            request.visibility,
        )
    }
}

#[derive(InputObject)]
pub struct RecordRequest {
    name: String,
    description: Option<String>,
    visibility: Visibility,
}
