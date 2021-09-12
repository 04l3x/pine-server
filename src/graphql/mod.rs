use async_graphql::{Context, Object};
use async_graphql::{EmptyMutation, EmptySubscription};

use sqlx::Postgres;

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

type Pool = sqlx::Pool<Postgres>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn repository(&self, ctx: &Context<'_>) -> Option<&'static str> {
        "one repo".into()
    }

	async fn all(&self, ctx: &Context<'_>) -> Option<&'static str> {
		let pool = ctx.data::<Pool>().unwrap();
		"all".into()
	}

	/*async fn characters(&self, ctx: &Context<'_>) -> FieldResult<Vec<Character>> {
	  let pool = ctx.data::<Pool>().unwrap();
	  let query_str = format!("SELECT id, name, kind FROM starwars.characters");
	  let result = sqlx::query(query_str.as_str())
	    .map(|row: PgRow| Character {
	      id: row.get("id"),
	      name: row.get("name"),
	      kind: row.get("kind"),
	    })
	    .fetch_all(pool.get().await.unwrap().deref_mut())
	    .await
	    .unwrap();
	  return Ok(result);
	}*/
}


struct Record {
	repo_name: String,
}

#[Object]
impl Record {
	async fn all(&self, ctx: &Context<'_>) -> Option<&'static str> {
		let pool = ctx.data::<Pool>().unwrap();
		"all".into()
	}
}
