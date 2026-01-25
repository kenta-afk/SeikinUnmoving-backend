use crate::state::UserServiceState;
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use userservice::{SignUpCommand, UserService};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct SignUpRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct SignUpResponse {
    pub jwt: String,
    pub refresh_token: String,
}

/// Sign up a new user
#[utoipa::path(
    post,
    path = "/api/user/signup",
    tag = "user",
    request_body = SignUpRequest,
    responses(
        (status = 200, description = "User successfully signed up", body = SignUpResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn signup<T>(
    State(UserServiceState(service)): State<UserServiceState<T>>,
    jar: CookieJar,
    Json(payload): Json<SignUpRequest>,
) -> Result<(CookieJar, Json<SignUpResponse>), StatusCode>
where
    T: UserService,
{
    let command = SignUpCommand {
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };

    match service.signup(command).await {
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
                Json(SignUpResponse {
                    jwt: dto.jwt,
                    refresh_token: dto.refresh_token,
                }),
            ))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
