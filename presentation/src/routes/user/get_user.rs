use crate::{extractors::AuthenticatedUser, state::UserServiceState};
use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use userservice::{GetUserCommand, UserService};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct GetUserResponse {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub seikin_similarity: f64,
}

#[utoipa::path(
    post,
    path = "/api/user",
    tag = "user",
    responses(
        (status = 200, description = "User found", body = GetUserResponse),
        (status = 400, description = "Bad request"),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user<T>(
    State(UserServiceState(service)): State<UserServiceState<T>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<Json<GetUserResponse>, StatusCode>
where
    T: UserService,
{
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
