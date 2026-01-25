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
        crate::routes::game::start_game,
        crate::routes::game::update_position,
        crate::routes::game::get_game_status,
        crate::routes::game::end_game,
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
        (name = "game", description = "Game endpoints"),
        (name = "health", description = "Health check endpoints")
    )
)]
struct ApiDoc;

pub fn build_router<US, GS>(app_state: AppState<US, GS>) -> Router
where
    US: userservice::UserService + Clone + Send + Sync,
    GS: gameservice::GameService + Clone + Send + Sync,
{
    let health_route = Router::new().route("/health", get(crate::routes::check::health::health));

    let user_route = Router::new()
        .route("/api/user/signup", post(crate::routes::user::signup))
        .route("/api/user/signin", post(crate::routes::user::signin))
        .route("/api/user/logout", post(crate::routes::user::logout))
        .route("/api/user/me", get(crate::routes::user::get_user))
        .route("/api/user", post(crate::routes::user::get_user))
        .route("/refresh", post(crate::routes::user::refresh));

    // ゲームルート
    let game_route = Router::new()
        .route("/api/game/start", post(crate::routes::game::start_game))
        .route(
            "/api/game/update-position",
            post(crate::routes::game::update_position),
        )
        .route(
            "/api/game/status/{session_id}",
            get(crate::routes::game::get_game_status),
        )
        .route(
            "/api/game/end/{session_id}",
            post(crate::routes::game::end_game),
        );

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
        .merge(game_route)
        .layer(cors)
        .with_state(app_state)
}
