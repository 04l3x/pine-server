use crate::auth;
use crate::auth::{
	session::Token,
	user::{Credentials, SignUpForm},
};
use crate::models::record;
use crate::utils::database::Pool;
use async_graphql::{Context, Object};
use error::{ApiError, Result};
use uuid::Uuid;

pub struct Mutations;

#[Object]
impl Mutations {
	async fn sign_up(&self, ctx: &Context<'_>, form: SignUpForm) -> Result<Token> {
		let pool = ctx.data::<Pool>().expect("error pool ctx");
		auth::sign_up(form, pool).await
	}

	async fn sign_in(&self, ctx: &Context<'_>, credentials: Credentials) -> Result<Token> {
		let pool = ctx.data::<Pool>().expect("error pool ctx");
		auth::sign_in(credentials, pool).await
	}

	async fn new_repository<'a>(
		&self,
		ctx: &'a Context<'_>,
		request: record::NewRepository,
	) -> Result<Uuid> {
		match ctx.data::<Token>() {
			Ok(token) => {
				if token.is_valid() {
					let pool = ctx.data::<Pool>().expect("error pool ctx");
					match record::Record::initialize(pool, token.get_sub_uuid(), request).await {
						Ok(uuid) => Ok(uuid),
						Err(_) => Err(Box::new(ApiError::Other)),
					}
				} else {
					Err(Box::new(ApiError::BadToken))
				}
			}
			Err(_) => Err(Box::new(ApiError::Unauthenticated)),
		}
	}
}
