use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
pub trait UuidService: Send + Sync {
    #[allow(dead_code)]
    fn new_v7(&self) -> Uuid;
}
