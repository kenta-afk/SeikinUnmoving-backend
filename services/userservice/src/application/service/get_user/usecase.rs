use crate::{
    ServiceError,
    application::service::get_user::{command::GetUserCommand, dto::GetUserDto},
    domain::repositories::user_repository::UserRepository,
};

pub struct GetUserUseCase<UR: UserRepository> {
    user_repo: UR,
}

#[allow(dead_code)]
impl<UR> GetUserUseCase<UR>
where
    UR: UserRepository,
{
    pub fn new(user_repo: UR) -> Self {
        Self { user_repo }
    }
}

#[allow(dead_code)]
impl<UR> GetUserUseCase<UR>
where
    UR: UserRepository,
{
    pub async fn execute(&self, command: GetUserCommand) -> Result<GetUserDto, ServiceError> {
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
