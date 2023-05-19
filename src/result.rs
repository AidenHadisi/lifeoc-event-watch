use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Serialize, Deserialize)]
/// Error type for this crate.
pub enum Error {
    #[error("Parser error: {0}")]
    /// Error returned by the parser.
    ParserError(String),

    #[error("API error: {0}")]
    /// Error returned by the API.
    ApiError(String),

    #[error("Internal server error: {0}")]
    /// Internal server error.
    InternalServerError(String),
}
