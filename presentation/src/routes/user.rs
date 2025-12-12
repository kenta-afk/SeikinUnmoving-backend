use crate::state::UserServiceState;
use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use userservice::{GetUserCommand, SignInCommand, SignUpCommand, UserId, UserService};

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignUpResponse {
    pub jwt: String,
    pub refresh_token: String,
}

/// Example handler that uses UserServiceState extracted from AppState
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

#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignInResponse {
    pub jwt: String,
    pub refresh_token: String,
}

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

#[derive(Deserialize)]
pub struct GetUserRequest {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct GetUserResponse {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub seikin_similarity: f64,
}

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
