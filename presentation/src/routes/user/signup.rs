use crate::state::UserServiceState;
use axum::{Json, extract::State, http::StatusCode};
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
    path = "/user/signup",
    tag = "user",
    request_body = SignUpRequest,
    responses(
        (status = 200, description = "User successfully signed up", body = SignUpResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn signup<T>(
    State(UserServiceState(service)): State<UserServiceState<T>>,
    Json(payload): Json<SignUpRequest>,
) -> Result<Json<SignUpResponse>, StatusCode>
where
    T: UserService,
{
    let command = SignUpCommand {
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };

    match service.signup(command).await {
        Ok(dto) => Ok(Json(SignUpResponse {
            jwt: dto.jwt,
            refresh_token: dto.refresh_token,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
