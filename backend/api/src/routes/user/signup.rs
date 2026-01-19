use worker::*;
use userservice::{SignUpCommand, UserService};
use crate::routes::user::common::create_user_service;
use crate::routes::user::signup_types::{SignUpRequest, SignUpResponse};

pub async fn signup(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // リクエストボディをパース
    let body: SignUpRequest = match req.json().await {
        Ok(b) => b,
        Err(e) => {
            return Response::error(format!("Invalid request body: {}", e), 400);
        }
    };

    // UserServiceを作成
    let user_service = match create_user_service(&ctx) {
        Ok(service) => service,
        Err(e) => {
            return Response::error(format!("Service initialization error: {}", e), 500);
        }
    };

    // サインアップコマンドを実行
    let command = SignUpCommand {
        name: body.name,
        email: body.email,
        password: body.password,
    };

    let result = match user_service.signup(command).await {
        Ok(dto) => dto,
        Err(e) => {
            return Response::error(format!("Signup error: {}", e), 400);
        }
    };

    // レスポンスを返す
    let response = SignUpResponse {
        jwt: result.jwt,
        refresh_token: result.refresh_token,
    };

    Response::from_json(&response)
}

pub fn register(router: Router<'_, ()>) -> Router<'_, ()> {
    router.post_async("/api/signup", signup)
}
