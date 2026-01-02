use uuid::Uuid;

use crate::application::ports::uuid_service::UuidService;

#[derive(Clone)]
pub struct UuidServiceImpl;

impl UuidService for UuidServiceImpl {
    fn new_v7(&self) -> Uuid {
        Uuid::now_v7()
    }
}
