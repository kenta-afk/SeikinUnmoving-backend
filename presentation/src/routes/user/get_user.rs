use crate::state::UserServiceState;
use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use userservice::{GetUserCommand, UserId, UserService};
use utoipa::ToSchema;

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
