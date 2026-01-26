use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("database error: {0}")]
    Generic(String),
    #[error("not found")]
    NotFound,
}
