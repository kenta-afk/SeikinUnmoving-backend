pub mod common;
pub mod extractor;
pub mod signup;
pub mod signin;
pub mod get_user;
pub mod logout;
pub mod refresh;

use worker::Router;

pub fn register(router: Router<'_, ()>) -> Router<'_, ()> {
    let router = signup::register(router);
    let router = signin::register(router);
    let router = get_user::register(router);
    let router = logout::register(router);
    let router = refresh::register(router);

    router
}
