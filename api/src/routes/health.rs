use worker::*;

pub fn register(router: Router<'_, ()>) -> Router<'_, ()> {
    router.get_async("/health", health_handler)
}

async fn health_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("OK")
}
