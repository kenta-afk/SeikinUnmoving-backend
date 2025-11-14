use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
pub trait UuidService: Send + Sync {
    fn new_v7(&self) -> Uuid;
}
