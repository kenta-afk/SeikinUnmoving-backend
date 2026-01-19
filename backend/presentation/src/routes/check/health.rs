/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = String)
    )
)]
pub async fn health() -> &'static str {
    "OK"
}
