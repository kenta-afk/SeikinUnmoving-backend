mod routes;

use std::env;
use tokio::net::TcpListener;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_path("dev/.env").ok();

    let log_level = match env::var("LOG_LEVEL").as_deref() {
        Ok("INFO") => Level::INFO,
        Ok("DEBUG") => Level::DEBUG,
        Ok("ERROR") => Level::ERROR,
        _ => Level::DEBUG,
    };

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(log_level)
        .with_thread_ids(true)
        .with_thread_names(true)
        .json()
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let _user_service = userservice::build_service(&database_url, &secret_key);

    let host = env::var("APIROUTE").expect("APIROUTE must be set");

    let router = routes::build::build_router();

    let listener = TcpListener::bind(host).await?;

    tracing::info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, router).await?;
    Ok(())
}
