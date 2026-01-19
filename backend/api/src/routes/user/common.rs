use worker::*;
use userservice::UserServiceImpl;
use crate::{UserRepositoryD1, ClientRepositoryD1};

// UserServiceを作成するヘルパー関数
pub fn create_user_service(ctx: &RouteContext<()>) -> Result<UserServiceImpl<UserRepositoryD1, ClientRepositoryD1, userservice::infrastructure::adapters::uuid_service::UuidServiceImpl, userservice::infrastructure::adapters::secret_service::SecretServiceImpl>> {
    // D1データベースを取得
    let db = ctx.env.d1("DB")
        .map_err(|e| Error::RustError(format!("Database connection error: {}", e)))?;
    let user_repo = UserRepositoryD1::new(db);
    
    let db2 = ctx.env.d1("DB")
        .map_err(|e| Error::RustError(format!("Database connection error: {}", e)))?;
    let client_repo = ClientRepositoryD1::new(db2);

    // JWT秘密鍵を取得
    let jwt_secret = ctx.env.var("JWT_SECRET")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "default-dev-secret-key-change-in-production".to_string());

    // サービスを作成
    let uuid_service = userservice::infrastructure::adapters::uuid_service::UuidServiceImpl;
    let secret_service = userservice::infrastructure::adapters::secret_service::SecretServiceImpl::new(&jwt_secret);
    
    Ok(UserServiceImpl::new(
        user_repo,
        client_repo,
        uuid_service,
        secret_service,
    ))
}
