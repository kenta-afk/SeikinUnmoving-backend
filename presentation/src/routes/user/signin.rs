use crate::state::UserServiceState;
use axum::{Json, extract::State, http::StatusCode};
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

/// Sign in an existing user
#[utoipa::path(
    post,
    path = "/user/signin",
    tag = "user",
    request_body = SignInRequest,
    responses(
        (status = 200, description = "User successfully signed in", body = SignInResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn signin<T>(
    State(UserServiceState(service)): State<UserServiceState<T>>,
    Json(payload): Json<SignInRequest>,
) -> Result<Json<SignInResponse>, StatusCode>
where
    T: UserService,
{
    let command = SignInCommand {
        email: payload.email,
        password: payload.password,
    };

    match service.signin(command).await {
        Ok(dto) => Ok(Json(SignInResponse {
            jwt: dto.jwt,
            refresh_token: dto.refresh_token,
        })),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
