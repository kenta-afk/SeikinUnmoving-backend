mod routes;
mod repositories;

pub use repositories::{
    client_repository_d1::ClientRepositoryD1,
    user_repository_d1::UserRepositoryD1,
    video_repository_d1::VideoRepositoryD1,
};

use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    // リクエストのOriginヘッダーを先に取得
    let origin = req.headers().get("Origin")?.unwrap_or_default();
    let allowed_origin = if origin.ends_with(".seikin-frontend.pages.dev") || origin.ends_with(".seikinunmoving.pages.dev") {
        origin
    } else {
        "*".to_string()
    };

    let router = Router::new();
    let router = routes::register_routes(router);

    // CORSヘッダーを追加
    let mut response = router.run(req, env).await?;
    
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", &allowed_origin)?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization")?;
    headers.set("Access-Control-Allow-Credentials", "true")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    
    Ok(response)
}
