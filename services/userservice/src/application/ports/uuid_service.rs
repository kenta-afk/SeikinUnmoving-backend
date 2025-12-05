use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
pub trait UuidService: Send + Sync + Clone {
    fn new_v7(&self) -> Uuid;
}

#[cfg(test)]
impl Clone for MockUuidService {
    fn clone(&self) -> Self {
        Self::default()
    }
}
