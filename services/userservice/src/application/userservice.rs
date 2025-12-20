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
        repositories::{
            client::client_repository::ClientRepository, user::user_repository::UserRepository,
        },
    },
};

#[async_trait::async_trait]
pub trait UserService: Send + Sync + 'static {
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

impl<UR, CR, IP, SS> Clone for UserServiceImpl<UR, CR, IP, SS>
where
    UR: UserRepository,
    CR: ClientRepository,
    IP: UuidService,
    SS: SecretService,
{
    fn clone(&self) -> Self {
        Self {
            user_repo: self.user_repo.clone(),
            client_repo: self.client_repo.clone(),
            uuid_service: self.uuid_service.clone(),
            secret_service: self.secret_service.clone(),
        }
    }
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

#[async_trait::async_trait]
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

        let jwt_claims = JwtClaims::new(user.id(), JWT_EXPIRATION_SECONDS);
        let refresh_token_claims =
            RefreshClaims::new(user.id(), &self.uuid_service, REFRESH_TOKEN_EXPIRATION_DAYS);

        let jwt = self.secret_service.create_jwt(&jwt_claims)?;
        let refresh_token = self
            .secret_service
            .create_refresh_token(&refresh_token_claims)?;

        let client = Client::new(
            user.id(),
            refresh_token_claims.jti,
            refresh_token_claims.exp,
            &self.uuid_service,
        );

        self.user_repo.create(user.into_create()).await?;
        self.client_repo.create(client).await?;

        Ok(SignUpDto { jwt, refresh_token })
    }

    async fn signin(&self, command: SignInCommand) -> Result<SignInDto, ServiceError> {
        let user = match self.user_repo.get_by_email(&command.email).await? {
            Some(user) => user,
            None => return Err(ServiceError::UserNotFound),
        };

        if !self
            .secret_service
            .verify_password(user.password(), &command.password)
        {
            return Err(ServiceError::PasswordVerificationFailed);
        }

        let jwt_claims = JwtClaims::new(user.id(), JWT_EXPIRATION_SECONDS);
        let refresh_token_claims =
            RefreshClaims::new(user.id(), &self.uuid_service, REFRESH_TOKEN_EXPIRATION_DAYS);

        let jwt = self.secret_service.create_jwt(&jwt_claims)?;
        let refresh_token = self
            .secret_service
            .create_refresh_token(&refresh_token_claims)?;

        let mut client = match self.client_repo.get_by_user_id(user.id()).await? {
            Some(client) => client,
            None => return Err(ServiceError::ClientNotFound),
        };

        let updated_client = client.update(
            user.id(),
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

        Ok(user.into_get())
    }
}
