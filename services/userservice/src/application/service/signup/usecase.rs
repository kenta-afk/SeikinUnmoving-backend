use crate::{
    ServiceError,
    application::{
        ports::{secret_service::SecretService, uuid_service::UuidService},
        service::signup::{
            command::SignUpCommand,
            constant::{JWT_EXPIRATION_SECONDS, REFRESH_TOKEN_EXPIRATION_DAYS},
            dto::SignUpDto,
        },
    },
    domain::{
        models::{client::Client, jwt::JwtClaims, refresh_token::RefreshClaims, user::User},
        repositories::{client_repository::ClientRepository, user_repository::UserRepository},
    },
};

pub struct SignUpImpl<UR: UserRepository, CR: ClientRepository, IP: UuidService, SS: SecretService>
{
    user_repo: UR,
    client_repo: CR,
    uuid_service: IP,
    secret_service: SS,
}

#[allow(dead_code)]
impl<UR, CR, IP, SS> SignUpImpl<UR, CR, IP, SS>
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
impl<UR, CR, IP, SS> SignUpImpl<UR, CR, IP, SS>
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

        self.user_repo.save(user).await?;
        self.client_repo.save(client).await?;

        Ok(SignUpDto { jwt, refresh_token })
    }
}
