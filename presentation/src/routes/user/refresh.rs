use crate::{extractors::RefreshTokenExtractor, state::UserServiceState};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Serialize;
use userservice::{RefreshCommand, UserService};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct RefreshResponse {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub seikin_similarity: f64,
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
            let jwt_cookie = Cookie::build(("jwt", dto.jwt.clone()))
                .path("/api")
                .http_only(true)
                .same_site(axum_extra::extract::cookie::SameSite::Lax)
                .build();

            let refresh_cookie = Cookie::build(("refresh_token", dto.refresh_token.clone()))
                .path("/")
                .http_only(true)
                .same_site(axum_extra::extract::cookie::SameSite::Lax)
                .build();

            let jar = jar.add(jwt_cookie).add(refresh_cookie);

            Ok((
                jar,
                Json(RefreshResponse {
                    user_id: dto.user_id.to_string(),
                    email: dto.email,
                    name: dto.name,
                    seikin_similarity: dto.seikin_similarity,
                }),
            ))
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
