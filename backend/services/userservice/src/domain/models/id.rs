use crate::application::ports::uuid_service::UuidService;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use uuid::Uuid;

#[cfg(not(target_arch = "wasm32"))]
use sqlx::{Type, encode::IsNull, error::BoxDynError, sqlite::{SqliteTypeInfo, SqliteValueRef}};

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

#[cfg(not(target_arch = "wasm32"))]
impl Type<sqlx::Sqlite> for UserId {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<sqlx::Sqlite>>::type_info()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for UserId {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
        Ok(UserId(Uuid::parse_str(s)?))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for UserId {
    fn encode_by_ref(&self, buf: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> Result<IsNull, BoxDynError> {
        <String as sqlx::Encode<sqlx::Sqlite>>::encode(self.0.to_string(), buf)
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

#[cfg(not(target_arch = "wasm32"))]
impl Type<sqlx::Sqlite> for ClientId {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<sqlx::Sqlite>>::type_info()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for ClientId {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
        Ok(ClientId(Uuid::parse_str(s)?))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for ClientId {
    fn encode_by_ref(&self, buf: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> Result<IsNull, BoxDynError> {
        <String as sqlx::Encode<sqlx::Sqlite>>::encode(self.0.to_string(), buf)
    }
}
