use crate::state::JwtConfig;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::decode;
use serde::{Deserialize, Serialize};
use userservice::UserId;

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: UserId,
    exp: usize,
    iat: usize,
}

pub struct AuthenticatedUser(pub UserId);

impl<S> FromRequestParts<S> for AuthenticatedUser
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

        let jwt_token = jar.get("jwt").ok_or(StatusCode::UNAUTHORIZED)?.value();

        let token_data =
            decode::<JwtClaims>(jwt_token, &jwt_config.decoding_key, &jwt_config.validation)
                .map_err(|e| {
                    tracing::warn!("JWT decode error: {}", e);
                    StatusCode::UNAUTHORIZED
                })?;

        tracing::debug!("User authenticated: {:?}", token_data.claims.sub);

        Ok(AuthenticatedUser(token_data.claims.sub))
    }
}
