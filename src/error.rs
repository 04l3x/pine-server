use std::fmt;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum AuthError {
	EmailExists,
	UsernameExists,
	BadUsername,
	BadPassword,
	Other,
}

impl std::error::Error for AuthError {}
impl fmt::Display for AuthError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({:?})", self)
	}
}
