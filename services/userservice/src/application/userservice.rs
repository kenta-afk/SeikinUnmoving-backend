use crate::{
    ServiceError,
    application::{
        command::{GetUserCommand, SignInCommand, SignUpCommand},
        dto::{GetUserDto, SignInDto, SignUpDto},
        ports::{
            constant::{JWT_EXPIRATION_SECONDS, REFRESH_TOKEN_EXPIRATION_DAYS},
            secret_service::SecretService,
            uuid_service::UuidService,
        },
    },
    domain::{
        models::{client::Client, jwt::JwtClaims, refresh_token::RefreshClaims, user::User},
        repositories::{client_repository::ClientRepository, user_repository::UserRepository},
    },
};
use async_trait::async_trait;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn signup(&self, command: SignUpCommand) -> Result<SignUpDto, ServiceError>;
    async fn signin(&self, command: SignInCommand) -> Result<SignInDto, ServiceError>;
    async fn get_user(&self, command: GetUserCommand) -> Result<GetUserDto, ServiceError>;
}

pub struct UserServiceImpl<UR, CR, IP, SS>
where
    UR: UserRepository,
    CR: ClientRepository,
    IP: UuidService,
    SS: SecretService,
{
    user_repo: UR,
    client_repo: CR,
    uuid_service: IP,
    secret_service: SS,
}

impl<UR, CR, IP, SS> UserServiceImpl<UR, CR, IP, SS>
where
    UR: UserRepository,
    CR: ClientRepository,
    IP: UuidService,
    SS: SecretService,
{
    pub fn new(user_repo: UR, client_repo: CR, uuid_service: IP, secret_service: SS) -> Self {
        Self {
            user_repo,
            client_repo,
            uuid_service,
            secret_service,
        }
    }
}

#[async_trait]
impl<UR, CR, IP, SS> UserService for UserServiceImpl<UR, CR, IP, SS>
where
    UR: UserRepository,
    CR: ClientRepository,
    IP: UuidService,
    SS: SecretService,
{
    async fn signup(&self, command: SignUpCommand) -> Result<SignUpDto, ServiceError> {
        let user = User::new(
            command.name,
            command.email,
            command.password,
            &self.uuid_service,
            &self.secret_service,
        );

        let jwt_claims = JwtClaims::new(user.id, JWT_EXPIRATION_SECONDS);
        let refresh_token_claims =
            RefreshClaims::new(user.id, &self.uuid_service, REFRESH_TOKEN_EXPIRATION_DAYS);

        let jwt = self.secret_service.create_jwt(&jwt_claims)?;
        let refresh_token = self
            .secret_service
            .create_refresh_token(&refresh_token_claims)?;

        let client = Client::new(
            user.id,
            refresh_token_claims.jti,
            refresh_token_claims.exp,
            &self.uuid_service,
        );

        self.user_repo.create(user).await?;
        self.client_repo.create(client).await?;

        Ok(SignUpDto { jwt, refresh_token })
    }

    async fn signin(&self, command: SignInCommand) -> Result<SignInDto, ServiceError> {
        if !self
            .secret_service
            .verify_password(&command.password, &command.password)
        {
            return Err(ServiceError::PasswordVerificationFailed);
        }

        let user = match self.user_repo.get_by_email(&command.email).await? {
            Some(user) => user,
            None => return Err(ServiceError::UserNotFound),
        };

        let jwt_claims = JwtClaims::new(user.id, JWT_EXPIRATION_SECONDS);
        let refresh_token_claims =
            RefreshClaims::new(user.id, &self.uuid_service, REFRESH_TOKEN_EXPIRATION_DAYS);

        let jwt = self.secret_service.create_jwt(&jwt_claims)?;
        let refresh_token = self
            .secret_service
            .create_refresh_token(&refresh_token_claims)?;

        let mut client = match self.client_repo.get_by_user_id(user.id).await? {
            Some(client) => client,
            None => return Err(ServiceError::ClientNotFound),
        };

        let updated_client = client.update(
            user.id,
            refresh_token_claims.jti,
            refresh_token_claims.exp,
            &self.uuid_service,
        );
        self.client_repo.save(updated_client).await?;

        Ok(SignInDto { jwt, refresh_token })
    }

    async fn get_user(&self, command: GetUserCommand) -> Result<GetUserDto, ServiceError> {
        let user = self
            .user_repo
            .get_by_id(command.user_id)
            .await?
            .ok_or(ServiceError::UserNotFound)?;

        Ok(GetUserDto {
            user_id: user.id,
            email: user.email,
            name: user.name,
            seikin_similarity: user.seikin_similarity,
        })
    }
}
