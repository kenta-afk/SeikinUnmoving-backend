use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn build_router<US>(app_state: AppState<US>) -> Router
where
    US: userservice::UserService + Clone,
{
    let health_route = Router::new().route("/health", get(crate::routes::check::health::health));

    let user_route = Router::new()
        .route("/user/signup", post(crate::routes::user::signup))
        .route("/user/signin", post(crate::routes::user::signin))
        .route("/user", post(crate::routes::user::get_user));

    Router::new()
        .merge(health_route)
        .merge(user_route)
        .with_state(app_state)
}
