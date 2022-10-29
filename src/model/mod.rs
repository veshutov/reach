use thiserror::Error as ThisError;

pub use db::Db;
pub use db::init_db;
pub use social::Social;
pub use social::SocialDao;
pub use social::SocialPatch;

mod db;
mod social;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity Not Found - {0}[{1}]")]
    EntityNotFound(&'static str, String),
    
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}