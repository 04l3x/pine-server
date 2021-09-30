pub mod session;
pub mod user;

use crate::error::Result;

use crate::utils::database::Pool;
use session::Token;

pub async fn sign_in(credentials: user::Credentials, pool: &Pool) -> Result<Token> {
	match user::User::authenticate(credentials, pool).await {
		Ok(uuid) => Ok(Token::generate(uuid)),
		Err(e) => Err(e),
	}
}

pub async fn sign_up(form: user::SignUpForm, pool: &Pool) -> Result<Token> {
	match user::User::register(form, pool).await {
		Ok(uuid) => Ok(Token::generate(uuid)),
		Err(e) => Err(e),
	}
}
