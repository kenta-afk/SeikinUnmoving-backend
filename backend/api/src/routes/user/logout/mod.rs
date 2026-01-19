use worker::*;
use userservice::{LogoutCommand, UserService};
use crate::routes::user::common::create_user_service;
use crate::routes::user::extractor::extract_user_id_from_jwt;

pub async fn logout(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // JWTからユーザーIDを取得
    let user_id = match extract_user_id_from_jwt(&req) {
        Ok(id) => id,
        Err(e) => {
            return Response::error(format!("Authorization error: {}", e), 401);
        }
    };

    // UserServiceを作成
    let user_service = match create_user_service(&ctx) {
        Ok(service) => service,
        Err(e) => {
            return Response::error(format!("Service initialization error: {}", e), 500);
        }
    };

    // ログアウト
    let command = LogoutCommand { user_id };
    match user_service.logout(command).await {
        Ok(_) => {},
        Err(e) => {
            return Response::error(format!("Logout error: {}", e), 400);
        }
    };

    Response::ok("Logged out successfully")
}

pub fn register(router: Router<'_, ()>) -> Router<'_, ()> {
    router.post_async("/api/logout", logout)
}
