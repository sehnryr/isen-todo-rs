#[cfg(feature = "server")]
use crate::repository::error::Error as RepositoryError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[cfg(feature = "server")]
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
}
