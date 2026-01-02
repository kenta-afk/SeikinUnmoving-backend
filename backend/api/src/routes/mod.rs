pub mod health;

use worker::Router;

pub fn register_routes(router: Router<'_, ()>) -> Router<'_, ()> {
    let router = health::register(router);

    router
}
