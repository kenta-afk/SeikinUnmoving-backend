use crate::state::UserServiceState;
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use userservice::{SignInCommand, UserService};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct SignInResponse {
    pub jwt: String,
    pub refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/api/user/signin",
    tag = "user",
    request_body = SignInRequest,
    responses(
        (status = 200, description = "User successfully signed in", body = SignInResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn signin<T>(
    State(UserServiceState(service)): State<UserServiceState<T>>,
    jar: CookieJar,
    Json(payload): Json<SignInRequest>,
) -> Result<(CookieJar, Json<SignInResponse>), StatusCode>
where
    T: UserService,
{
    let command = SignInCommand {
        email: payload.email,
        password: payload.password,
    };

    match service.signin(command).await {
        Ok(dto) => {
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
                Json(SignInResponse {
                    jwt: dto.jwt,
                    refresh_token: dto.refresh_token,
                }),
            ))
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
