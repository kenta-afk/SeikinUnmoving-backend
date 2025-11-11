mod routes;

use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let router = Router::new();
    let router = routes::register_routes(router);

    router.run(req, env).await
}
