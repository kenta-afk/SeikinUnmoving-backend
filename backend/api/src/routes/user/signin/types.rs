use serde::{Deserialize, Serialize};

// SignIn用のリクエスト型
#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

// SignIn用のレスポンス型
#[derive(Serialize)]
pub struct SignInResponse {
    pub jwt: String,
    pub refresh_token: String,
}
