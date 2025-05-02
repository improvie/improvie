use std::fmt;

use serde::{Deserialize, Serialize, Serializer};
use sqlx::{
    Decode, Encode, Sqlite, Type,
    encode::IsNull,
    error::BoxDynError,
    sqlite::{SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef},
};
use uuid::fmt::Hyphenated;

// ref: [uuid crate](https://github.com/uuid-rs/uuid/blob/main/src/macros.rs)
#[macro_export]
macro_rules! uid {
    ($uid:expr) => {{
        const OUTPUT: $crate::Uid = match $crate::Uid::try_parse($uid) {
            Ok(u) => u,
            Err(_) => panic!("invalid UID"),
        };
        OUTPUT
    }};
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "ts", bind::ts("uid.ts"))]
#[repr(transparent)]
pub struct Uid(uuid::Uuid);

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.as_hyphenated().fmt(f)
    }
}

impl Uid {
    pub fn now() -> Self {
        Self(uuid::Uuid::now_v7())
    }

    pub fn new(ts: uuid::Timestamp) -> Self {
        Self(uuid::Uuid::new_v7(ts))
    }

    pub const fn nil() -> Self {
        Self(uuid::Uuid::nil())
    }

    pub const fn is_nil(&self) -> bool {
        self.0.is_nil()
    }

    pub const fn as_u128(&self) -> u128 {
        self.0.as_u128()
    }

    pub const fn try_parse(s: &str) -> Result<Uid, uuid::Error> {
        match uuid::Uuid::try_parse(s) {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(e),
        }
    }
}

impl From<uuid::Uuid> for Uid {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl From<Uid> for uuid::Uuid {
    fn from(value: Uid) -> Self {
        value.0
    }
}

impl Serialize for Uid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.as_hyphenated().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Uid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        uuid::Uuid::deserialize(deserializer).map(Self)
    }
}

impl Type<Sqlite> for Uid {
    fn type_info() -> SqliteTypeInfo {
        Hyphenated::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for Uid {
    fn encode_by_ref(
        &self,
        args: &mut Vec<SqliteArgumentValue<'q>>,
    ) -> Result<IsNull, BoxDynError> {
        self.0.as_hyphenated().encode_by_ref(args)
    }
}

impl Decode<'_, Sqlite> for Uid {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        Hyphenated::decode(value).map(|v| Self(v.into_uuid()))
    }
}
