use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;

use crate::model::{Social, SocialDao, SocialPatch};
use crate::security::Claims;
use crate::web::{internal_error, State};

pub async fn create_social(
    Extension(state): Extension<Arc<State>>,
    claims: Claims,
    body: Json<SocialPatch>,
) -> Result<Json<Social>, (StatusCode, String)> {
    SocialDao::create(&state.db, &claims, body.0)
        .await
        .map_err(internal_error)
        .map(Json::from)
}

pub async fn get_social_by_id(
    Extension(state): Extension<Arc<State>>,
    _: Claims,
    path: Path<String>,
) -> Result<Json<Social>, (StatusCode, String)> {
    SocialDao::find_by_id(&state.db, path.parse().unwrap())
        .await
        .map_err(internal_error)
        .map(Json::from)
}