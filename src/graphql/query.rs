use async_graphql::{Context, Object};

use crate::models;
use crate::utils::database::Pool;

pub struct Queries;

#[Object]
impl Queries {
	async fn all_public_record(&self, ctx: &Context<'_>) -> Option<Vec<models::record::Record>> {
		let pool = ctx.data::<Pool>().expect("error pool ctx");
		models::record::Record::get_all_public(pool).await
	}
}
