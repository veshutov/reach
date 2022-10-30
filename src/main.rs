use crate::model::{init_db, RepoError};
use crate::security::SecurityError;
use crate::web::{start_web, WebError};

mod model;
mod security;
mod web;

#[tokio::main]
async fn main() {
    let db = init_db()
        .await
        .expect("failed to initialize db");
    start_web(db)
        .await
        .expect("failed to start web server");
}

pub enum AppError {
    Repo(RepoError),
    Security(SecurityError),
    Web(WebError)
}
