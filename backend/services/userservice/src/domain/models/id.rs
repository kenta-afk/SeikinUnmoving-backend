use crate::application::ports::uuid_service::UuidService;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(uuid_service: &impl UuidService) -> Self {
        Self(uuid_service.new_v7())
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UserId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserId(Uuid::parse_str(s)?))
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ClientId(Uuid);

impl ClientId {
    pub fn new(uuid_service: &impl UuidService) -> Self {
        Self(uuid_service.new_v7())
    }
}

impl fmt::Display for ClientId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ClientId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ClientId(Uuid::parse_str(s)?))
    }
}
