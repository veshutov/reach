use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;

use axum::{Extension, Json, Router};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post};

use crate::model::{Db, Social, SocialDao, SocialPatch};
use crate::security::{UserCtx, utx_from_token};

#[derive(Clone)]
pub struct State {
    db: Db,
}

pub async fn start_web(db: Db) -> hyper::Result<()> {
    let shared_state = Arc::new(State { db });
    let app = Router::new()
        .route("/socials/:id", get(get_social_by_id))
        .route("/socials", post(create_social))
        .layer(Extension(shared_state));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
}


async fn create_social(Extension(state): Extension<Arc<State>>, body: Json<SocialPatch>) -> Result<Json<Social>, (StatusCode, String)> {
    let user_ctx = utx_from_token("123").await.unwrap();
    SocialDao::create(&state.db, &user_ctx, body.0)
        .await
        .map_err(internal_error)
        .map(Json::from)
}

async fn get_social_by_id(Extension(state): Extension<Arc<State>>, path: Path<String>) -> Result<Json<Social>, (StatusCode, String)> {
    SocialDao::find_by_id(&state.db, path.parse().unwrap())
        .await
        .map_err(internal_error)
        .map(Json::from)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
    where E: Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}