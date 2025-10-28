use axum::{Router, routing::get};

pub fn build_router() -> Router {
    let health_route = Router::new().route("/health", get(crate::router::check::health::health));

    Router::new().merge(health_route)
}
