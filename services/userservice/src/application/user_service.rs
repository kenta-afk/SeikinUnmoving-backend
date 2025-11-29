use crate::{
    ServiceError,
    application::{command::signup::SignUpCommand, dto::signup::SignUpDto},
    domain::{
        models::{
            constant::{JWT_EXPIRATION_SECONDS, REFRESH_EXPIRATION_DAYS},
            jwt::JwtClaims,
            refresh_token::RefreshClaims,
            user::User,
        },
        services::{secret_service::SecretService, uuid_service::UuidService},
        user_repository::UserRepository,
    },
};

pub struct UserServiceImpl<UR: UserRepository, IP: UuidService, SS: SecretService> {
    user_repo: UR,
    uuid_service: IP,
    secret_service: SS,
}

#[allow(dead_code)]
impl<UR, IP, SS> UserServiceImpl<UR, IP, SS>
where
    UR: UserRepository,
    IP: UuidService,
    SS: SecretService,
{
    pub fn new(user_repo: UR, uuid_service: IP, secret_service: SS) -> Self {
        Self {
            user_repo,
            uuid_service,
            secret_service,
        }
    }
}

#[allow(dead_code)]
impl<UR, IP, SS> UserServiceImpl<UR, IP, SS>
where
    UR: UserRepository,
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
            RefreshClaims::new(user.id, &self.uuid_service, REFRESH_EXPIRATION_DAYS);

        let jwt = self.secret_service.create_jwt(&jwt_claims)?;
        let refresh_token = self
            .secret_service
            .create_refresh_token(&refresh_token_claims)?;

        self.user_repo.save(user).await?;

        Ok(SignUpDto { jwt, refresh_token })
    }
}
