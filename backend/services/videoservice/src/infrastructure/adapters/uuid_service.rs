use crate::application::ports::uuid_service::UuidService;
use uuid::Uuid;

#[derive(Clone)]
pub struct UuidServiceImpl;

impl UuidService for UuidServiceImpl {
    fn generate(&self) -> String {
        Uuid::now_v7().to_string()
    }
}
