use std::fmt;

use serde::{Deserialize, Serialize, Serializer};
#[cfg(feature = "db")]
use sqlx::{
    Decode, Encode, Sqlite, Type,
    encode::IsNull,
    error::BoxDynError,
    sqlite::{SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef},
};

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

#[cfg(feature = "db")]
impl Type<Sqlite> for Uid {
    fn type_info() -> SqliteTypeInfo {
        uuid::fmt::Hyphenated::type_info()
    }
}

#[cfg(feature = "db")]
impl<'q> Encode<'q, Sqlite> for Uid {
    fn encode_by_ref(
        &self,
        args: &mut Vec<SqliteArgumentValue<'q>>,
    ) -> Result<IsNull, BoxDynError> {
        self.0.as_hyphenated().encode_by_ref(args)
    }
}

#[cfg(feature = "db")]
impl Decode<'_, Sqlite> for Uid {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        uuid::fmt::Hyphenated::decode(value).map(|v| Self(v.into_uuid()))
    }
}

#[cfg(feature = "db")]
impl From<Uid> for sea_orm::Value {
    fn from(val: Uid) -> Self {
        val.0.into()
    }
}

#[cfg(feature = "db")]
impl sea_orm::sea_query::ValueType for Uid {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        <uuid::Uuid as sea_orm::sea_query::ValueType>::try_from(v).map(Self)
    }

    fn type_name() -> String {
        uuid::Uuid::type_name()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        uuid::Uuid::array_type()
    }

    fn column_type() -> sea_orm::ColumnType {
        uuid::Uuid::column_type()
    }
}

#[cfg(feature = "db")]
impl sea_orm::TryFromU64 for Uid {
    fn try_from_u64(n: u64) -> Result<Self, sea_orm::DbErr> {
        uuid::Uuid::try_from_u64(n).map(Self)
    }
}

#[cfg(feature = "db")]
impl sea_orm::sea_query::Nullable for Uid {
    fn null() -> sea_orm::Value {
        uuid::Uuid::null()
    }
}

#[cfg(feature = "db")]
impl sea_orm::TryGetable for Uid {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        uuid::Uuid::try_get_by(res, index).map(Self)
    }
}
