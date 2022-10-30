use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};

use serde_json::json;

use crate::{AppError, WebError};
use crate::model::RepoError;
use crate::security::SecurityError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Repo(repo_error) => {
                match repo_error {
                    RepoError::EntityNotFound(_, _) => (StatusCode::NOT_FOUND, repo_error.to_string()),
                    RepoError::SqlxError(_) => (StatusCode::INTERNAL_SERVER_ERROR, repo_error.to_string()),
                    RepoError::IOError(_) => (StatusCode::INTERNAL_SERVER_ERROR, repo_error.to_string())
                }
            }
            AppError::Security(security_error) => {
                match security_error {
                    SecurityError::InvalidAuthToken => (StatusCode::UNAUTHORIZED, security_error.to_string()),
                    SecurityError::InvalidCredentials => (StatusCode::UNAUTHORIZED, security_error.to_string()),
                    SecurityError::FailedToEncodeClaims => (StatusCode::INTERNAL_SERVER_ERROR, security_error.to_string())
                }
            }
            AppError::Web(web_error) => {
                match web_error {
                    WebError::UnsupportedApiVersion => (StatusCode::BAD_REQUEST, web_error.to_string()),
                    WebError::InvalidRequest(_) => (StatusCode::BAD_REQUEST, web_error.to_string())
                }
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
