use worker::*;
use userservice::{SignInCommand, UserService};
use crate::routes::user::common::create_user_service;
use crate::routes::user::signin_types::{SignInRequest, SignInResponse};

pub async fn signin(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // リクエストボディをパース
    let body: SignInRequest = match req.json().await {
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

    // サインインコマンドを実行
    let command = SignInCommand {
        email: body.email,
        password: body.password,
    };

    let result = match user_service.signin(command).await {
        Ok(dto) => dto,
        Err(e) => {
            return Response::error(format!("Signin error: {}", e), 401);
        }
    };

    // レスポンスを返す
    let response = SignInResponse {
        jwt: result.jwt,
        refresh_token: result.refresh_token,
    };

    Response::from_json(&response)
}

pub fn register(router: Router<'_, ()>) -> Router<'_, ()> {
    router.post_async("/api/signin", signin)
}
