use crate::domain::models::error::DbError;
use thiserror::Error;

mod application;
mod domain;
mod infrastructure;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    Database(#[from] DbError),
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

#[allow(dead_code)]
fn main() {
    println!("Hello, world!");
}
