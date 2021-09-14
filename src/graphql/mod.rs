mod mutation;
mod query;

use async_graphql::EmptySubscription;

use crate::database;

pub type Schema = async_graphql::Schema<query::Queries, mutation::Mutations, EmptySubscription>;

pub async fn build_schema() -> Schema {
	Schema::build(query::Queries, mutation::Mutations, EmptySubscription)
        .data(database::default_pool().await)
        .finish()
}

