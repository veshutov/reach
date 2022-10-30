use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::Path;

use crate::{AppError, RepoError, WebError};
use crate::model::{create, find_by_id, Social, SocialCreateDto};
use crate::security::Claims;
use crate::web::State;

pub async fn create_social(
    Extension(state): Extension<Arc<State>>,
    claims: Claims,
    body: Json<SocialCreateDto>,
) -> Result<Json<Social>, AppError> {
    create(&state.db, &claims, body.0)
        .await
        .map_err(to_app_error)
        .map(Json::from)
}

pub async fn get_social_by_id(
    Extension(state): Extension<Arc<State>>,
    _: Claims,
    path: Path<(String, String)>,
) -> Result<Json<Social>, AppError> {
    let id = path.1.parse()
        .map_err(|_| AppError::Web(WebError::InvalidRequest("invalid social id".to_string())))?;
    find_by_id(&state.db, id)
        .await
        .map_err(to_app_error)
        .map(Json::from)
}

fn to_app_error(err: RepoError) -> AppError {
    AppError::Repo(err)
}
