use crate::infrastructure::{
    adapters::{secret_service::SecretServiceImpl, uuid_service::UuidServiceImpl},
    repositories::{
        client::repository::ClientRepositoryImpl, user::repository::UserRepositoryImpl,
    },
};
use thiserror::Error;

mod application;
mod domain;
pub mod infrastructure;

pub use application::{
    command::{
        get_user::GetUserCommand, logout::LogoutCommand, refresh::RefreshCommand,
        signin::SignInCommand, signup::SignUpCommand,
    },
    dto::{
        get_user::GetUserDto, logout::LogoutDto, refresh::RefreshDto, signin::SignInDto,
        signup::SignUpDto,
    },
    ports::{secret_service::SecretService, uuid_service::UuidService},
    userservice::{UserService, UserServiceImpl},
};
pub use domain::{
    models::{
        client::Client,
        error::DbError,
        id::{ClientId, UserId},
        user::User,
    },
    repositories::{
        client::{
            client_repository::ClientRepository, create_client::CreateClient,
            save_client::SaveClient,
        },
        user::{create_user::CreateUser, user_repository::UserRepository},
    },
};

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    Database(#[from] DbError),
    #[error("JWT error: {0}")]
    Jwt(String),
    #[error("password verification failed")]
    PasswordVerificationFailed,
    #[error("user not found")]
    UserNotFound,
    #[error("client not found")]
    ClientNotFound,
}

impl From<String> for ServiceError {
    fn from(s: String) -> Self {
        ServiceError::Jwt(s)
    }
}

pub type ConcreteUserService =
    UserServiceImpl<UserRepositoryImpl, ClientRepositoryImpl, UuidServiceImpl, SecretServiceImpl>;

// 開発環境用のビルド関数
#[cfg(not(target_arch = "wasm32"))]
pub async fn build_service(
    _database_url: &str,
    secret_key: &str,
) -> Result<ConcreteUserService, ServiceError> {
    let user_repo = UserRepositoryImpl::default();
    let client_repo = ClientRepositoryImpl::default();
    let uuid_service = UuidServiceImpl;
    let secret_service = SecretServiceImpl::new(secret_key);

    Ok(UserServiceImpl::new(
        user_repo,
        client_repo,
        uuid_service,
        secret_service,
    ))
}
