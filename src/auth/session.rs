use serde::{Deserialize, Serialize};
use uuid::Uuid;
//use jsonwebtoken::errors::ErrorKind;
use async_graphql::Object;
use jsonwebtoken::{
	/*decode,*/ encode, Algorithm, /*DecodingKey,*/ EncodingKey,
	Header, /*Validation*/
};
use std::env;

pub struct Token(String);

#[Object]
impl Token {
	async fn token(&self) -> String {
		self.0.clone()
	}
}

impl Token {
	pub fn generate(user_id: Uuid) -> Token {
		Token(
			encode(
				&Header::new(Algorithm::HS512),
				&Claims::new(user_id.to_string()),
				&EncodingKey::from_secret(env::var("SECRET_KEY").unwrap().as_bytes()),
			)
			.unwrap(),
		)
	}
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
	exp: usize,
	sub: String, //subject of the jwt
}

impl Claims {
	fn new(sub: String) -> Claims {
		Claims {
			exp: 60 * 60 * 12,
			sub,
		}
	}
}
