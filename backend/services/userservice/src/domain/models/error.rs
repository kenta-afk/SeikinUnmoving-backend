use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Generic(String),
    #[error("Not found")]
    NotFound,
    #[error("Constraint violation")]
    ConstraintViolation,
}

// ローカル開発環境用（WASM以外）
#[cfg(not(target_arch = "wasm32"))]
impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        DbError::Generic(err.to_string())
    }
}
