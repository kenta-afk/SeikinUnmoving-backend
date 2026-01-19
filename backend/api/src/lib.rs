mod routes;
mod repositories;

pub use repositories::{
    client_repository_d1::ClientRepositoryD1,
    user_repository_d1::UserRepositoryD1,
};

use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let router = Router::new();
    let router = routes::register_routes(router);

    // CORSヘッダーを追加
    let mut response = router.run(req, env).await?;
    
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    
    Ok(response)
}
