use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct VideoId(String);

impl VideoId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VideoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for VideoId {
    fn from(id: String) -> Self {
        Self::new(id)
    }
}

impl From<VideoId> for String {
    fn from(id: VideoId) -> Self {
        id.0
    }
}
