use crate::error::{AuthError, Result};
use async_graphql::Object;
use chrono::{DateTime, Duration, Timelike, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{
	decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

type IsValid = bool;

#[derive(Clone)]
pub struct Token(pub String);

#[Object]
impl Token {
	async fn token(&self) -> String {
		self.0.clone()
	}
}

impl Token {
	pub fn generate(user_id: Uuid, duration_min: i64) -> Token {
		let iat = Utc::now();
		let exp = iat + chrono::Duration::minutes(duration_min);
		Token(
			encode(
				&Header::new(Algorithm::HS512),
				&Claims::new(user_id.to_string(), iat, exp),
				&EncodingKey::from_secret(env::var("SECRET_KEY").unwrap().as_bytes()),
			)
			.unwrap(),
		)
	}

	pub fn is_valid(&self) -> IsValid {
		match self.decode() {
			Ok(_) => true,
			Err(_) => false,
		}
	}

	pub fn get_sub_uuid(&self) -> Uuid {
		assert!(self.is_valid());
		self.get_claims().unwrap().sub_uuid()
	}

	fn decode(&self) -> Result<TokenData<Claims>> {
		match decode::<Claims>(
			&self.0,
			&DecodingKey::from_secret(env::var("SECRET_KEY").unwrap().as_bytes()),
			&Validation::new(Algorithm::HS512),
		) {
			Ok(token_data) => Ok(token_data),
			Err(err) => match *err.kind() {
				ErrorKind::InvalidToken => Err(Box::new(AuthError::InvalidToken)),
				_ => Err(Box::new(AuthError::Other)),
			},
		}
	}

	fn get_claims(&self) -> Result<Claims> {
		match self.decode() {
			Ok(token) => Ok(token.claims),
			Err(e) => Err(e),
		}
	}
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
	sub: String,
	#[serde(with = "jwt_numeric_date")]
	iat: DateTime<Utc>,
	#[serde(with = "jwt_numeric_date")]
	exp: DateTime<Utc>,
}

impl Claims {
	fn new(sub: String, iat: DateTime<Utc>, exp: DateTime<Utc>) -> Claims {
		let iat = iat
			.date()
			.and_hms_milli(iat.hour(), iat.minute(), iat.second(), 0);
		let exp = exp
			.date()
			.and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0);

		Claims { exp, sub, iat }
	}

	fn sub_uuid(&self) -> Uuid {
		Uuid::parse_str(&self.sub).expect("error sub to uuid")
	}
}

mod jwt_numeric_date {
	use chrono::{DateTime, TimeZone, Utc};
	use serde::{self, Deserialize, Deserializer, Serializer};

	pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let timestamp = date.timestamp();
		serializer.serialize_i64(timestamp)
	}

	pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
	where
		D: Deserializer<'de>,
	{
		Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
			.single() // If there are multiple or no valid DateTimes from timestamp, return None
			.ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
	}
}
