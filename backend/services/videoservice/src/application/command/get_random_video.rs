use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRandomVideoCommand;

impl GetRandomVideoCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GetRandomVideoCommand {
    fn default() -> Self {
        Self::new()
    }
}
