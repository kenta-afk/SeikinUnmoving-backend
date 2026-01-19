use worker::*;
use userservice::UserId;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use uuid::Uuid;

/// JWTからユーザーIDを抽出する
pub fn extract_user_id_from_jwt(req: &Request) -> Result<UserId> {
    let auth_header = req.headers().get("Authorization")
        .map_err(|_| Error::RustError("Missing Authorization header".to_string()))?
        .ok_or_else(|| Error::RustError("Missing Authorization header".to_string()))?;
    
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| Error::RustError("Invalid Authorization header format".to_string()))?;
    
    decode_jwt_claims(token)
}

/// JWTをデコードしてクレームを取得する
fn decode_jwt_claims(token: &str) -> Result<UserId> {
    // トークンをデコードしてユーザーIDを取得
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(Error::RustError("Invalid JWT format".to_string()));
    }
    
    // ペイロード部分をデコード
    let payload = parts[1];
    let decoded = URL_SAFE_NO_PAD.decode(payload)
        .map_err(|e| Error::RustError(format!("Failed to decode JWT: {}", e)))?;
    let json: serde_json::Value = serde_json::from_slice(&decoded)
        .map_err(|e| Error::RustError(format!("Failed to parse JWT payload: {}", e)))?;
    
    let user_id_str = json["sub"]
        .as_str()
        .ok_or_else(|| Error::RustError("Missing sub claim in JWT".to_string()))?;
    
    user_id_str.parse::<UserId>()
        .map_err(|e| Error::RustError(format!("Invalid user ID: {}", e)))
}

/// リフレッシュトークンからuser_idとjtiを抽出する
pub fn extract_refresh_token_claims(token: &str) -> Result<(UserId, Uuid)> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(Error::RustError("Invalid refresh token format".to_string()));
    }
    
    // ペイロード部分をデコード
    let payload = parts[1];
    let decoded = URL_SAFE_NO_PAD.decode(payload)
        .map_err(|e| Error::RustError(format!("Failed to decode refresh token: {}", e)))?;
    let json: serde_json::Value = serde_json::from_slice(&decoded)
        .map_err(|e| Error::RustError(format!("Failed to parse refresh token payload: {}", e)))?;
    
    let user_id_str = json["sub"]
        .as_str()
        .ok_or_else(|| Error::RustError("Missing sub claim in refresh token".to_string()))?;
    let user_id = user_id_str.parse::<UserId>()
        .map_err(|e| Error::RustError(format!("Invalid user ID: {}", e)))?;
    
    let jti_str = json["jti"]
        .as_str()
        .ok_or_else(|| Error::RustError("Missing jti claim in refresh token".to_string()))?;
    let jti = Uuid::parse_str(jti_str)
        .map_err(|e| Error::RustError(format!("Invalid jti: {}", e)))?;
    
    Ok((user_id, jti))
}
