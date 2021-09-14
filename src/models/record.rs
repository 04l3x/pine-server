use async_graphql::{Context, Object};

use async_graphql::{Type, OutputType};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Record {
	id: uuid::Uuid,
	name: String,
}

impl Record {
	pub async fn id() -> Option<Uuid> {
		Some(Uuid::new_v4())
	}

	pub async fn name() -> Option<&'static str> {
		"record_name".into()
	}
}


