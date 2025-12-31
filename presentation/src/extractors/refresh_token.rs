use crate::state::JwtConfig;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::decode;
use serde::{Deserialize, Serialize};
use userservice::UserId;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenClaims {
    sub: UserId,
    jti: Uuid,
    exp: i64,
    iat: i64,
}

pub struct RefreshTokenExtractor {
    pub user_id: UserId,
    pub jti: Uuid,
}

impl<S> FromRequestParts<S> for RefreshTokenExtractor
where
    S: Send + Sync,
    JwtConfig: axum::extract::FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jwt_config = JwtConfig::from_ref(state);

        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let refresh_token = jar
            .get("refresh_token")
            .ok_or(StatusCode::UNAUTHORIZED)?
            .value();

        let token_data = decode::<RefreshTokenClaims>(
            refresh_token,
            &jwt_config.decoding_key,
            &jwt_config.validation,
        )
        .map_err(|e| {
            tracing::warn!("Refresh token decode error: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

        tracing::debug!(
            "Refresh token extracted for user: {:?}",
            token_data.claims.sub
        );

        Ok(RefreshTokenExtractor {
            user_id: token_data.claims.sub,
            jti: token_data.claims.jti,
        })
    }
}
