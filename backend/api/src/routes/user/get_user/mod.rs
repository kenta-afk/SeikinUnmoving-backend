pub mod types;

use worker::*;
use userservice::{GetUserCommand, UserService};
use crate::routes::user::common::create_user_service;
use crate::routes::user::extractor::extract_user_id_from_jwt;
use types::GetUserResponse;

pub async fn get_user(req: Request, ctx: RouteContext<()>) -> Result<Response> {
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

    // ユーザー情報を取得
    let command = GetUserCommand { user_id };
    let result = match user_service.get_user(command).await {
        Ok(dto) => dto,
        Err(e) => {
            return Response::error(format!("Get user error: {}", e), 404);
        }
    };

    // レスポンスを返す
    let response = GetUserResponse {
        user_id: result.user_id.to_string(),
        email: result.email,
        name: result.name,
        seikin_similarity: result.seikin_similarity,
    };

    Response::from_json(&response)
}

pub fn register(router: Router<'_, ()>) -> Router<'_, ()> {
    router.get_async("/user/me", get_user)
}
