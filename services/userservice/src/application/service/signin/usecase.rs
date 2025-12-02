use crate::{
    ServiceError,
    application::{
        ports::{
            constant::{JWT_EXPIRATION_SECONDS, REFRESH_TOKEN_EXPIRATION_DAYS},
            secret_service::SecretService,
            uuid_service::UuidService,
        },
        service::signin::{command::SignInCommand, dto::SignInDto},
    },
    domain::{
        models::{jwt::JwtClaims, refresh_token::RefreshClaims},
        repositories::{client_repository::ClientRepository, user_repository::UserRepository},
    },
};

pub struct SignInUseCase<
    UR: UserRepository,
    CR: ClientRepository,
    IP: UuidService,
    SS: SecretService,
> {
    user_repo: UR,
    client_repo: CR,
    uuid_service: IP,
    secret_service: SS,
}

#[allow(dead_code)]
impl<UR, CR, IP, SS> SignInUseCase<UR, CR, IP, SS>
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

#[allow(dead_code)]
impl<UR, CR, IP, SS> SignInUseCase<UR, CR, IP, SS>
where
    UR: UserRepository,
    CR: ClientRepository,
    IP: UuidService,
    SS: SecretService,
{
    pub async fn execute(&self, command: SignInCommand) -> Result<SignInDto, ServiceError> {
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
}
