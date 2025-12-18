use crate::state::UserServiceState;
use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use userservice::{GetUserCommand, SignInCommand, SignUpCommand, UserId, UserService};
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

#[derive(Deserialize, ToSchema)]
pub struct GetUserRequest {
    pub user_id: String,
}

#[derive(Serialize, ToSchema)]
pub struct GetUserResponse {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub seikin_similarity: f64,
}

/// Get user information
#[utoipa::path(
    post,
    path = "/user",
    tag = "user",
    request_body = GetUserRequest,
    responses(
        (status = 200, description = "User found", body = GetUserResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user<T>(
    State(UserServiceState(service)): State<UserServiceState<T>>,
    Json(payload): Json<GetUserRequest>,
) -> Result<Json<GetUserResponse>, StatusCode>
where
    T: UserService,
{
    let user_id = payload
        .user_id
        .parse::<UserId>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let command = GetUserCommand { user_id };

    match service.get_user(command).await {
        Ok(dto) => Ok(Json(GetUserResponse {
            user_id: dto.user_id.to_string(),
            email: dto.email,
            name: dto.name,
            seikin_similarity: dto.seikin_similarity,
        })),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
