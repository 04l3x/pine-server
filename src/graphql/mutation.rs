use async_graphql::{Context, Object, Result};

use crate::auth;
use crate::utils::database::Pool;
use crate::models::record;

pub struct Mutations;

#[Object]
impl Mutations {
    async fn sign_up(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<auth::Session> {
        Ok(auth::Session::new("token".to_string()))
    }

    async fn sign_in(&self, username: String, password: String) -> Result<auth::Session> {
        Ok(auth::Session::new("token".to_string()))
    }

    async fn create_repo(&self, ctx: &Context<'_>, request: record::RecordRequest) -> bool {
        let pool = ctx.data::<Pool>().expect("error pool ctx");
        match record::Record::create(pool, uuid::Uuid::new_v4(), request).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
