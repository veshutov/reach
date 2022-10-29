use std::error::Error;
use std::sync::Arc;

use axum::{Extension, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};

use crate::model::Db;
use crate::security::authorize;
use crate::web::social::{create_social, get_social_by_id};

mod social;

#[derive(Clone)]
pub struct State {
    db: Db,
}

pub async fn start_web(db: Db) -> hyper::Result<()> {
    let shared_state = Arc::new(State { db });
    let app = Router::new()
        .route("/auth", post(authorize))
        .route("/socials/:id", get(get_social_by_id))
        .route("/socials", post(create_social))
        .layer(Extension(shared_state));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
    where E: Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}