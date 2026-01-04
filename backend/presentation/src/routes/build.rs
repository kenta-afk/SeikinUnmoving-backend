use crate::state::AppState;
use axum::{
    Router,
    http::{Method, header},
    routing::{get, post},
};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::check::health::health,
        crate::routes::user::signup,
        crate::routes::user::signin,
        crate::routes::user::get_user,
        crate::routes::user::refresh,
        crate::routes::user::logout,
    ),
    components(
        schemas(
            crate::routes::user::SignUpRequest,
            crate::routes::user::SignUpResponse,
            crate::routes::user::SignInRequest,
            crate::routes::user::SignInResponse,
            crate::routes::user::GetUserResponse,
            crate::routes::user::RefreshResponse,
            crate::routes::user::LogoutResponse,
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
        .route("/user/logout", post(crate::routes::user::logout))
        .route("/refresh", post(crate::routes::user::refresh))
        .route("/api/user", post(crate::routes::user::get_user));

    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:8081"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::COOKIE, header::AUTHORIZATION])
        .allow_credentials(true);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(health_route)
        .merge(user_route)
        .layer(cors)
        .with_state(app_state)
}
