use crate::{
    ServiceError,
    application::command::signup::SignUpCommand,
    domain::{
        models::{id::UserId, user::User},
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
    async fn signup(&self, command: SignUpCommand) -> Result<(String, String), ServiceError> {
        let user_id = UserId::new(&self.uuid_service);
        let hashed_password = self.secret_service.hash_password(&command.password);

        let user = User::new(
            user_id,
            command.name,
            command.email,
            hashed_password,
            chrono::Utc::now(),
            chrono::Utc::now(),
        );

        self.user_repo.save(user).await?;

        let jwt = self.secret_service.create_jwt(user_id)?;
        let refresh_token = self.secret_service.create_secret();

        Ok((jwt, refresh_token))
    }
}
