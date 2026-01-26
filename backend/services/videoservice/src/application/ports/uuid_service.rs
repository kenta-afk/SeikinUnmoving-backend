pub trait UuidService: Send + Sync {
    fn generate(&self) -> String;
}
