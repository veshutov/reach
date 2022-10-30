use crate::model::RepoError;
use crate::security::SecurityError;
use crate::web::WebError;

mod model;
mod security;
mod web;

#[tokio::main]
async fn main() {
    let db = model::init_db()
        .await
        .expect("failed to initialize db");
    web::start_web(db)
        .await
        .expect("failed to start web server");
}

pub enum AppError {
    Repo(RepoError),
    Security(SecurityError),
    Web(WebError)
}
