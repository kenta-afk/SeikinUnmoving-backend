pub mod health;
pub mod user;
pub mod game;
pub mod video;

use worker::{Router, Request, Response, Result, RouteContext};

// OPTIONSリクエストを処理（CORSプリフライト）
async fn handle_options(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let headers = worker::Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://c1356b39.seikin-frontend.pages.dev")?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization")?;
    headers.set("Access-Control-Allow-Credentials", "true")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    
    Ok(Response::empty()?.with_headers(headers))
}

pub fn register_routes(router: Router<'_, ()>) -> Router<'_, ()> {
    let router = health::register(router);
    let router = user::register(router);
    let router = game::register(router);
    let router = video::register(router);

    // すべてのルートでOPTIONSメソッドを処理
    router.options_async("/*catchall", handle_options)
}
