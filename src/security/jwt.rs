use axum::async_trait;
use axum::extract::{FromRequest, RequestParts, TypedHeader};
use axum::Json;
use headers::Authorization;
use headers::authorization::Bearer;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;

use crate::{AppError, SecurityError};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub company: String,
    pub exp: usize,
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub async fn authorize(body: Json<AuthRequest>) -> Result<Json<AuthResponse>, AppError> {
    //todo should compare with values from db
    if body.client_id != "client_id" || body.client_secret != "client_secret" {
        return Err(AppError::Security(SecurityError::InvalidCredentials));
    }

    //todo should fetch Claims from db
    let claims = Claims {
        sub: 11,
        company: "tomoru".to_owned(),
        exp: 2000000000,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AppError::Security(SecurityError::FailedToEncodeClaims))?;

    return Ok(Json::from(AuthResponse {
        access_token: token,
        token_type: "Bearer".to_owned(),
    }));
}

#[async_trait]
impl<S> FromRequest<S> for Claims
    where
        S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<S>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = req
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Security(SecurityError::InvalidAuthToken))?;

        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AppError::Security(SecurityError::InvalidAuthToken))?;

        //todo should also check exp
        Ok(token_data.claims)
    }
}
