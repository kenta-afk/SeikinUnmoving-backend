use crate::{extractors::RefreshTokenExtractor, state::UserServiceState};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Serialize;
use userservice::{RefreshCommand, UserService};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct RefreshResponse {
    pub jwt: String,
    pub refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/refresh",
    tag = "user",
    responses(
        (status = 200, description = "User token successfully refreshed", body = RefreshResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn refresh<T>(
    State(UserServiceState(service)): State<UserServiceState<T>>,
    RefreshTokenExtractor { user_id, jti }: RefreshTokenExtractor,
    jar: CookieJar,
) -> Result<(CookieJar, Json<RefreshResponse>), StatusCode>
where
    T: UserService,
{
    let command = RefreshCommand { user_id, jti };

    match service.refresh_token(command).await {
        Ok(dto) => {
            tracing::info!("Refresh token successful for user: {:?}", user_id);

            let jwt_cookie = Cookie::build(("jwt", dto.jwt.clone()))
                .path("/api")
                .http_only(true)
                .same_site(axum_extra::extract::cookie::SameSite::Lax)
                .build();

            let refresh_cookie = Cookie::build(("refresh_token", dto.refresh_token.clone()))
                .path("/refresh")
                .http_only(true)
                .same_site(axum_extra::extract::cookie::SameSite::Lax)
                .build();

            let jar = jar.add(jwt_cookie).add(refresh_cookie);

            Ok((
                jar,
                Json(RefreshResponse {
                    jwt: dto.jwt,
                    refresh_token: dto.refresh_token,
                }),
            ))
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
