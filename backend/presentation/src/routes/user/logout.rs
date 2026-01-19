use crate::{extractors::jwt::AuthenticatedUser, state::UserServiceState};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Serialize;
use userservice::{LogoutCommand, UserService};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LogoutResponse {
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/user/logout",
    tag = "user",
    responses(
        (status = 200, description = "User successfully logged out", body = LogoutResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn logout<T>(
    State(UserServiceState(service)): State<UserServiceState<T>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    jar: CookieJar,
) -> Result<(CookieJar, Json<LogoutResponse>), StatusCode>
where
    T: UserService,
{
    let command = LogoutCommand { user_id };

    match service.logout(command).await {
        Ok(_) => {
            // クッキーを削除
            let jar = jar
                .remove(Cookie::from("jwt"))
                .remove(Cookie::from("refresh_token"));

            tracing::info!("User logged out successfully: {:?}", user_id);

            Ok((
                jar,
                Json(LogoutResponse {
                    message: "Successfully logged out".to_string(),
                }),
            ))
        }
        Err(e) => {
            tracing::error!("Logout failed: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
