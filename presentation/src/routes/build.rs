use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::check::health::health,
        crate::routes::user::signup,
        crate::routes::user::signin,
        crate::routes::user::get_user,
    ),
    components(
        schemas(
            crate::routes::user::SignUpRequest,
            crate::routes::user::SignUpResponse,
            crate::routes::user::SignInRequest,
            crate::routes::user::SignInResponse,
            crate::routes::user::GetUserRequest,
            crate::routes::user::GetUserResponse,
        )
    ),
    tags(
        (name = "user", description = "User management endpoints"),
        (name = "health", description = "Health check endpoints")
    )
)]
struct ApiDoc;

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
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(health_route)
        .merge(user_route)
        .with_state(app_state)
}
