use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVideosCommand;

impl GetVideosCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GetVideosCommand {
    fn default() -> Self {
        Self::new()
    }
}
