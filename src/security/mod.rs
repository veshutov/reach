use thiserror::Error as ThisError;

pub use jwt::{authorize, Claims};

mod jwt;

#[derive(ThisError, Debug)]
pub enum SecurityError {
    #[error("invalid auth token")]
    InvalidAuthToken,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("failed to encode claims")]
    FailedToEncodeClaims,
}
