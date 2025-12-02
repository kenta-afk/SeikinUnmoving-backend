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
    ports::{secret_service::SecretService, uuid_service::UuidService},
    service::{
        get_user::{command::GetUserCommand, dto::GetUserDto, usecase::GetUserUseCase},
        signin::{command::SignInCommand, dto::SignInDto, usecase::SignInUseCase},
        signup::{command::SignUpCommand, dto::SignUpDto, usecase::SignUpUseCase},
    },
};
pub use domain::repositories::{
    client_repository::ClientRepository, user_repository::UserRepository,
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

pub type UserServiceInstance =
    UserService<UserRepositoryImpl, ClientRepositoryImpl, UuidServiceImpl, SecretServiceImpl>;

pub struct UserService<UR, CR, IP, SS>
where
    UR: UserRepository,
    CR: ClientRepository,
    IP: UuidService,
    SS: SecretService,
{
    signup_usecase: SignUpUseCase<UR, CR, IP, SS>,
    signin_usecase: SignInUseCase<UR, CR, IP, SS>,
    get_user_usecase: GetUserUseCase<UR>,
}

impl<UR, CR, IP, SS> UserService<UR, CR, IP, SS>
where
    UR: UserRepository + Clone,
    CR: ClientRepository + Clone,
    IP: UuidService + Clone,
    SS: SecretService + Clone,
{
    pub fn new(user_repo: UR, client_repo: CR, uuid_service: IP, secret_service: SS) -> Self {
        Self {
            signup_usecase: SignUpUseCase::new(
                user_repo.clone(),
                client_repo.clone(),
                uuid_service.clone(),
                secret_service.clone(),
            ),
            signin_usecase: SignInUseCase::new(
                user_repo.clone(),
                client_repo.clone(),
                uuid_service.clone(),
                secret_service.clone(),
            ),
            get_user_usecase: GetUserUseCase::new(user_repo),
        }
    }

    pub async fn signup(&self, command: SignUpCommand) -> Result<SignUpDto, ServiceError> {
        self.signup_usecase.execute(command).await
    }

    pub async fn signin(&self, command: SignInCommand) -> Result<SignInDto, ServiceError> {
        self.signin_usecase.execute(command).await
    }

    pub async fn get_user(&self, command: GetUserCommand) -> Result<GetUserDto, ServiceError> {
        self.get_user_usecase.execute(command).await
    }
}

pub async fn build_service(
    db_url: &str,
    secret_key: &str,
) -> Result<UserServiceInstance, Box<dyn std::error::Error + Send + Sync>> {
    let pool = Pool::<Sqlite>::connect(db_url).await?;

    Ok(UserService::new(
        UserRepositoryImpl::new(pool.clone()),
        ClientRepositoryImpl::new(pool),
        UuidServiceImpl,
        SecretServiceImpl::new(secret_key),
    ))
}
