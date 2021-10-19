use async_graphql::{Context, Object};

use crate::models;
use crate::utils::database::Pool;

use uuid::Uuid;

pub struct Queries;

#[Object]
impl Queries {
	async fn all_public_record(&self, ctx: &Context<'_>) -> Option<Vec<models::record::Record>> {
		let pool = ctx.data::<Pool>().expect("error pool ctx");
		models::record::Record::get_all_public(pool).await
	}

	async fn tree(&self, ctx: &Context<'_>, repo_id: Uuid) -> Option<bool> {
		Some(false)
	}

	async fn full_tree(&self, ctx: &Context<'_>, repo_id: Uuid) -> Option<bool> {
		Some(false)
	}

	/*async fn list_all_repos(&self, ctx: &Context<'_>) -> Option<bool> {
		Some(false)
	}

	async fn list_all_with_filter(&self, ctx: &Context<'_>) -> Option<bool> {
		Some(false)
	}*/
}
