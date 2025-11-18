use crate::{
    ServiceError,
    application::{command::signup::SignUpCommand, dto::signup::SignUpDto},
    domain::{
        models::user::User,
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

        let jwt = self.secret_service.create_jwt(user.id)?;
        let refresh_token = self.secret_service.create_secret();

        self.user_repo.save(user).await?;

        Ok(SignUpDto { jwt, refresh_token })
    }
}
