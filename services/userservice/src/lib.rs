use crate::{
    domain::models::error::DbError,
    infrastructure::{
        adapters::{secret_service::SecretServiceImpl, uuid_service::UuidServiceImpl},
        repositories::{
            client::repository::ClientRepositoryImpl, user::repository::UserRepositoryImpl,
        },
    },
};
use sqlx::{Pool, Sqlite};
use thiserror::Error;

mod application;
mod domain;
mod infrastructure;

pub use application::{
    command::{GetUserCommand, SignInCommand, SignUpCommand},
    dto::{GetUserDto, SignInDto, SignUpDto},
    ports::{secret_service::SecretService, uuid_service::UuidService},
    userservice::{UserService, UserServiceImpl},
};
pub use domain::{
    models::id::UserId,
    repositories::{
        client::client_repository::ClientRepository, user::user_repository::UserRepository,
    },
};

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    Database(#[from] DbError),
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("password verification failed")]
    PasswordVerificationFailed,
    #[error("user not found")]
    UserNotFound,
    #[error("client not found")]
    ClientNotFound,
}

pub type ConcreteUserService =
    UserServiceImpl<UserRepositoryImpl, ClientRepositoryImpl, UuidServiceImpl, SecretServiceImpl>;

pub async fn build_service(
    db_url: &str,
    secret_key: &str,
) -> Result<ConcreteUserService, Box<dyn std::error::Error + Send + Sync>> {
    let pool = Pool::<Sqlite>::connect(db_url).await?;

    Ok(UserServiceImpl::new(
        UserRepositoryImpl::new(pool.clone()),
        ClientRepositoryImpl::new(pool),
        UuidServiceImpl,
        SecretServiceImpl::new(secret_key),
    ))
}
