use async_graphql::{Context, Object};

use crate::models;
use crate::utils::database::Pool;

pub struct Queries;

#[Object]
impl Queries {
	async fn all_record(&self, ctx: &Context<'_>) -> Option<Vec<models::record::Record>> {
		let pool = ctx.data::<Pool>().expect("error pool ctx");
		let res = models::record::Record::get_all(pool).await;
		println!("all record gql");
		println!("{:?}", res);
		match res {
			Some(res) => res.into(),
			None => None,
		}
	}
}
