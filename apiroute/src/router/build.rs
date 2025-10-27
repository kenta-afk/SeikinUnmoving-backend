use axum::{routing::get, Router};

pub fn build_router() -> Router {
    let health_route = Router::new().route("/health", get(crate::router::health::health::health));

    Router::new().merge(health_route)
}