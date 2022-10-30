use thiserror::Error as ThisError;

pub use db::Db;
pub use db::init_db;
pub use social::create;
pub use social::find_by_id;
pub use social::Social;
pub use social::SocialCreateDto;

mod db;
mod social;

#[derive(ThisError, Debug)]
pub enum RepoError {
    #[error("Entity not found - {0}[{1}]")]
    EntityNotFound(&'static str, String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
