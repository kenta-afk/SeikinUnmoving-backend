pub mod types;

use worker::*;
use userservice::{RefreshCommand, UserService};
use crate::routes::user::common::create_user_service;
use crate::routes::user::extractor::extract_refresh_token_claims;
use types::{RefreshRequest, RefreshResponse};

pub async fn refresh(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // リクエストボディをパース
    let body: RefreshRequest = match req.json().await {
        Ok(b) => b,
        Err(e) => {
            return Response::error(format!("Invalid request body: {}", e), 400);
        }
    };

    // リフレッシュトークンからuser_idとjtiを抽出
    let (user_id, jti) = match extract_refresh_token_claims(&body.refresh_token) {
        Ok(claims) => claims,
        Err(e) => {
            return Response::error(format!("Invalid refresh token: {}", e), 401);
        }
    };

    // UserServiceを作成
    let user_service = match create_user_service(&ctx) {
        Ok(service) => service,
        Err(e) => {
            return Response::error(format!("Service initialization error: {}", e), 500);
        }
    };

    // トークンをリフレッシュ
    let command = RefreshCommand { user_id, jti };
    let result = match user_service.refresh_token(command).await {
        Ok(dto) => dto,
        Err(e) => {
            return Response::error(format!("Refresh token error: {}", e), 401);
        }
    };

    // レスポンスを返す
    let response = RefreshResponse {
        user_id: result.user_id.to_string(),
        email: result.email,
        name: result.name,
        seikin_similarity: result.seikin_similarity,
        jwt: result.jwt,
        refresh_token: result.refresh_token,
    };

    Response::from_json(&response)
}

pub fn register(router: Router<'_, ()>) -> Router<'_, ()> {
    router.post_async("/user/refresh", refresh)
}
