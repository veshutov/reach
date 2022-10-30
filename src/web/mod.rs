use std::net::SocketAddr;
use std::sync::Arc;

use axum::{BoxError, Extension, Router};
use axum::async_trait;
use axum::body::HttpBody;
use axum::extract::{FromRequest, RequestParts};
use axum::extract::rejection::JsonRejection;
use axum::routing::{get, post};
use serde::de::DeserializeOwned;
use thiserror::Error as ThisError;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::AppError;
use crate::model::Db;
use crate::security::authorize;
use crate::web::social::{create_social, get_social_by_id};

mod social;
mod version;
mod error;

pub async fn start_web(db: Db) -> hyper::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("tower_http=debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let shared_state = Arc::new(State { db });
    let app = Router::new()
        .route("/api/:version/auth", post(authorize))
        .route("/api/:version/socials/:id", get(get_social_by_id))
        .route("/api/:version/socials", post(create_social))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(shared_state));

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await
}

pub struct State {
    db: Db,
}

#[derive(ThisError, Debug)]
pub enum WebError {
    #[error("unsupported api version")]
    UnsupportedApiVersion,

    #[error("{0}")]
    InvalidRequest(String),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

pub struct ReqJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ReqJson<T>
    where
        T: DeserializeOwned,
        S: Send + Sync,
        S: HttpBody + Send,
        S::Data: Send,
        S::Error: Into<BoxError>,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<S>) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => {
                Err(AppError::Web(WebError::JsonExtractorRejection(rejection)))
            }
        }
    }
}
