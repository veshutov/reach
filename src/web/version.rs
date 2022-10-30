use std::collections::HashMap;

use axum::async_trait;
use axum::extract::{FromRequest, Path, RequestParts};

use crate::{AppError, WebError};

enum Version {
    V1,
}

#[async_trait]
impl<S> FromRequest<S> for Version
    where
        S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<S>) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> = req.extract()
            .await
            .map_err(|_| AppError::Web(WebError::UnsupportedApiVersion))?;

        let version = params.get("version")
            .ok_or_else(|| AppError::Web(WebError::UnsupportedApiVersion))?;

        match version.as_str() {
            "v1" => Ok(Version::V1),
            _ => Err(AppError::Web(WebError::UnsupportedApiVersion))
        }
    }
}
