use serde::{Deserialize, Serialize};

// SignUp用のリクエスト型
#[derive(Deserialize)]
pub struct SignUpRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

// SignUp用のレスポンス型
#[derive(Serialize)]
pub struct SignUpResponse {
    pub jwt: String,
    pub refresh_token: String,
}
