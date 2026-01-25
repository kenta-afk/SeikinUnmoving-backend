use crate::state::JwtConfig;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, header, request::Parts},
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::decode;
use serde::{Deserialize, Serialize};
use userservice::UserId;

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: UserId,
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

        // Authorizationヘッダーから取得を試みる
        let jwt_token = if let Some(auth_header) = parts.headers.get(header::AUTHORIZATION) {
            let auth_str = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())
            } else {
                None
            }
        } else {
            None
        };

        // Authorizationヘッダーにない場合はクッキーから取得
        let jwt_token = match jwt_token {
            Some(token) => token,
            None => {
                let jar = CookieJar::from_request_parts(parts, state)
                    .await
                    .map_err(|_| StatusCode::UNAUTHORIZED)?;
                jar.get("jwt")
                    .ok_or(StatusCode::UNAUTHORIZED)?
                    .value()
                    .to_string()
            }
        };

        let token_data =
            decode::<JwtClaims>(&jwt_token, &jwt_config.decoding_key, &jwt_config.validation)
                .map_err(|e| {
                    tracing::warn!("JWT decode error: {}", e);
                    StatusCode::UNAUTHORIZED
                })?;

        tracing::debug!("User authenticated: {:?}", token_data.claims.sub);

        Ok(AuthenticatedUser(token_data.claims.sub))
    }
}
