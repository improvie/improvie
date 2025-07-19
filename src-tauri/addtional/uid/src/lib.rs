use std::fmt;

use serde::{Deserialize, Serialize, Serializer};

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
#[cfg_attr(feature = "db", derive(sea_orm::DeriveValueType))]
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

#[cfg(feature = "db")]
impl sea_orm::TryFromU64 for Uid {
    fn try_from_u64(n: u64) -> Result<Self, sea_orm::DbErr> {
        uuid::Uuid::try_from_u64(n).map(Self)
    }
}
