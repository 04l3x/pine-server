use crate::models::record::{Record, /*RecordFilter, */ Records};
use crate::utils::database::Pool;
use async_graphql::{Context, Object};
use error::{ApiError, Result};
//use git::{RepoTree, RepoFullTree/*, RepoFullInfo*/};
use crate::auth::session::Token;

pub struct Queries;

#[Object]
impl Queries {
	async fn public_record(
		&self,
		ctx: &Context<'_>,
		page: Option<usize>,
		query: Option<String>,
	) -> Result<Records> {
		let pool = ctx.data::<Pool>().expect("error pool ctx");

		match query {
			Some(query) => match page {
				Some(page) => {
					Record::public_record_paginated_with_name_filter(pool, page, query).await
				}
				None => Record::public_record_paginated_with_name_filter(pool, 1, query).await,
			},
			None => match page {
				Some(page) => Record::public_record_paginated(pool, page).await,
				None => Record::public_record_paginated(pool, 1).await,
			},
		}
	}

	async fn is_logged(&self, ctx: &Context<'_>) -> Result<bool> {
		match ctx.data::<Token>() {
			Ok(token) => Ok(token.is_valid()),
			Err(_) => Err(Box::new(ApiError::Unauthenticated)),
		}
	}

	//async fn public_record(
	//	&self,
	//	ctx: &Context<'_>,
	//	page: Option<usize>,
	//	query: Option<String>, //Option<Filter>
	//) -> Result<Records> {
	//	let pool = ctx.data::<Pool>().expect("error pool ctx");

	//	match query {
	//		Some(query) => match page {
	//			Some(page) => Ok(RecordsBuilder::new().build()),
	//			None => Ok(RecordsBuilder::new().build()),
	//		},
	//		None => match page {
	//			Some(page) => Record::public_record_paginated(pool, page).await,
	//			None => Record::public_record_paginated(pool, 1).await,
	//		},
	//	}
	//}

	async fn debug_tree(
		&self,
		ctx: &Context<'_>,
		username: String,
		repo_name: String,
	) -> Result<bool> {
		let pool = ctx.data::<Pool>().expect("error pool ctx");

		match Record::repo_path(pool, repo_name, username).await {
			Ok(path) => {
				git::repository::Repository::debug_tree(path);
				Ok(true)
			}
			Err(e) => Err(e),
		}
	}
}
