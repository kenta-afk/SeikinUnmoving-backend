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
