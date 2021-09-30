use crate::error::{AuthError, Result};
use crate::utils::database::Pool;
use argon2::{
	password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
	Algorithm, Argon2, Params, Version,
};
use async_graphql::InputObject;
use sqlx;
use sqlx::{
	postgres::{PgQueryResult, PgRow},
	Row,
};
use uuid::Uuid;

#[derive(InputObject)]
pub struct SignUpForm {
	email: String,
	username: String,
	password: String,
}

#[derive(Debug, InputObject)]
pub struct Credentials {
	username: String,
	password: String,
}

impl Credentials {
	fn generate(username: String, password: String) -> Credentials {
		let salt = SaltString::generate(&mut OsRng);

		Credentials {
			username,
			password: Credentials::encrypt_input(password, &salt),
		}
	}

	fn argon2() -> Argon2<'static> {
		Argon2::new(
			Algorithm::Argon2id,
			Version::V0x13,
			Params::new(15360, 3, 1, None).unwrap(),
		)
	}

	fn encrypt_input(password: String, salt: &SaltString) -> String {
		Credentials::argon2()
			.hash_password(password.as_bytes(), salt)
			.unwrap()
			.to_string()
	}

	fn verify(&self, not_encrypted: String) -> bool {
		let hash = PasswordHash::new(&self.password).unwrap();
		Credentials::argon2()
			.verify_password(not_encrypted.as_bytes(), &hash)
			.is_ok()
	}
}

#[derive(Debug)]
pub struct User {
	id: Uuid,
	email: String,
	credentials: Credentials,
	verified: bool,
}

///database
impl User {
	async fn instert(&self, pool: &Pool) -> sqlx::Result<PgQueryResult> {
		sqlx::query(
			"
			INSERT INTO public.users
			(id, email, username, password, verified)
			VALUES ($1, $2, $3, $4, $5);
		",
		)
		.bind(self.id.clone())
		.bind(self.email.clone())
		.bind(self.credentials.username.clone())
		.bind(self.credentials.password.clone())
		.bind(self.verified)
		.execute(pool)
		.await
	}

	async fn read_by_username(pool: &Pool, username: String) -> sqlx::Result<PgRow> {
		sqlx::query(
			"
			SELECT * FROM public.users WHERE username = $1;	
		",
		)
		.bind(username)
		.fetch_one(pool)
		.await
	}
}

///impl for model behavior and publics signatures
///user's module returns primitives then
///session's module generate or not the session depending the result
impl User {
	fn new(email: String, credentials: Credentials) -> User {
		User {
			id: Uuid::new_v4(),
			email,
			credentials,
			verified: false,
		}
	}

	pub async fn register(form: SignUpForm, pool: &Pool) -> Result<Uuid> {
		let new_user = User::from(form);

		match &new_user.instert(pool).await {
			Ok(_) => Ok(new_user.id),
			Err(e) => match e.as_database_error() {
				Some(e) => {
					if e.message().contains("email") {
						Err(Box::new(AuthError::EmailExists))
					} else if e.message().contains("username") {
						Err(Box::new(AuthError::UsernameExists))
					} else {
						Err(Box::new(AuthError::Other))
					}
				}
				None => Err(Box::new(AuthError::Other)),
			},
		}
	}

	pub async fn authenticate(credentials: Credentials, pool: &Pool) -> Result<Uuid> {
		match User::read_by_username(pool, credentials.username).await {
			Ok(pg_row) => {
				let user = User::from(pg_row);
				if user.credentials.verify(credentials.password) {
					Ok(user.id)
				} else {
					Err(Box::new(AuthError::BadPassword))
				}
			}
			Err(e) => match e {
				sqlx::Error::RowNotFound => Err(Box::new(AuthError::BadUsername)),
				_ => Err(Box::new(AuthError::Other)),
			},
		}
	}
}

impl From<PgRow> for User {
	fn from(pg_row: PgRow) -> User {
		let credentials = Credentials {
			username: pg_row.get("username"),
			password: pg_row.get("password"),
		};

		User {
			id: pg_row.get("id"),
			email: pg_row.get("email"),
			credentials,
			verified: pg_row.get("verified"),
		}
	}
}

impl From<SignUpForm> for User {
	fn from(form: SignUpForm) -> User {
		User::new(
			form.email,
			Credentials::generate(form.username, form.password),
		)
	}
}

#[cfg(test)]
mod authentication {
	#[test]
	fn register() {
		todo!();
	}

	#[test]
	fn authtenticate() {
		todo!();
	}
}
