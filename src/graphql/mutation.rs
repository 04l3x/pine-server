use async_graphql::{Context, Object};

use crate::auth;
use crate::auth::{
	session::Token,
	user::{Credentials, SignUpForm},
};
use crate::models::record;
use crate::utils::database::Pool;

pub struct Mutations;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[Object]
impl Mutations {
	async fn sign_up(&self, ctx: &Context<'_>, form: SignUpForm) -> Result<Token> {
		let pool = ctx.data::<Pool>().expect("error pool ctx");
		match auth::sign_up(form, pool).await {
			Ok(token) => Ok(token),
			Err(e) => Err(e),
		}
	}

	async fn sign_in(&self, ctx: &Context<'_>, credentials: Credentials) -> Result<Token> {
		let pool = ctx.data::<Pool>().expect("error pool ctx");
		match auth::sign_in(credentials, pool).await {
			Ok(token) => Ok(token),
			Err(e) => Err(e),
		}
	}

	async fn create_repo(&self, ctx: &Context<'_>, request: record::RecordRequest) -> bool {
		let pool = ctx.data::<Pool>().expect("error pool ctx");
		match record::Record::create(pool, uuid::Uuid::new_v4(), request).await {
			//FIXME
			Ok(_) => true,
			Err(_) => false,
		}
	}
}
