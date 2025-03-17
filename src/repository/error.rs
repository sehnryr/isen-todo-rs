pub(super) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Sql error: {0}")]
    Sql(#[from] sqlx::Error),

    #[error("Username already exists")]
    UsernameAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,
}
