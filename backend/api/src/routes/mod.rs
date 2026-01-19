pub mod health;
pub mod user;

use worker::Router;

pub fn register_routes(router: Router<'_, ()>) -> Router<'_, ()> {
    let router = health::register(router);
    let router = user::register(router);

    router
}
