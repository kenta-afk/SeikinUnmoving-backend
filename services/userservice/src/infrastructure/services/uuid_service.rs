use uuid::Uuid;

use crate::domain::services::uuid_service::UuidService;

#[allow(dead_code)]
pub struct UuidServiceImpl;

impl UuidService for UuidServiceImpl {
    fn new_v7(&self) -> Uuid {
        Uuid::now_v7()
    }
}
