use crate::domain::{services::uuid_service::UuidService, user_repository::UserRepository};

#[allow(dead_code)]
pub struct UserService<UR: UserRepository, IP: UuidService> {
    user_repo: UR,
    uuid_service: IP,
}

#[allow(dead_code)]
impl<UR: UserRepository, IP: UuidService> UserService<UR, IP> {
    pub fn new(user_repo: UR, uuid_service: IP) -> Self {
        Self {
            user_repo,
            uuid_service,
        }
    }
}
