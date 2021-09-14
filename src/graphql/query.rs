use async_graphql::{Context, Object};

use crate::models;

pub struct Queries;

#[Object]
impl Queries {
    async fn record_id(&self, ctx: &Context<'_>) -> uuid::Uuid {
        let res = models::record::Record::id().await;
        res.expect("")
    }

    /*async fn record(&self, ctx: &Context<'_>, name: String) -> FieldResult<models::record::Record> {
        //let res = models::record::Record::id().await;
        Ok(models::record::Record{
            id: uuid::Uuid::new_v4(),
            name
        })
    }*/

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
