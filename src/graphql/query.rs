use crate::models::{
	record::{Record, /*RecordFilter, */ Records},
	repository,
};
use crate::utils::database::Pool;
use async_graphql::{Context, Object};
use error::{ApiError, GitError, Result};
//use git::{RepoTree, RepoFullTree/*, RepoFullInfo*/};
use crate::auth::session::Token;
use git::tree::Tree;
use uuid::Uuid;

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

	async fn repo_tree(&self, ctx: &Context<'_>, id: Uuid) -> Result<Tree> {
		let pool = ctx.data::<Pool>().expect("error pool ctx");

		match repository::Repository::path_by_uuid(pool, &id).await {
			Ok(path) => repository::Repository::full_tree(path),
			Err(_) => Err(Box::new(GitError::Other)),
		}
	}
}
