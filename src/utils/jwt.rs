use crate::config;
use crate::http::error::Error;
use crate::http::Result;
use axum::async_trait;
use axum::extract::{FromRequest, RequestParts, TypedHeader};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use chrono::{Duration, Utc};
use config::JWT_SECRET;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: i64,
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let exp = Utc::now() + Duration::hours(48);

        Claims {
            user_id,
            exp: exp.timestamp(),
        }
    }

    pub fn to_jwt(&self) -> String {
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(&JWT_SECRET.as_bytes()),
        );

        token.expect("failed to generate token")
    }

    pub fn verify(token: &str) -> Result<Claims> {
        Ok(jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret(&JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)?)
    }
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = Error;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| Error::InvalidToken)?;
        let data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(&JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| Error::InvalidToken)?;

        Ok(data.claims)
    }
}
