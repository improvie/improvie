use std::{fmt, time::SystemTime};

use serde::{Deserialize, Serialize, Serializer};

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

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Uid(ulid::Ulid);

impl Uid {
    pub fn new() -> Self {
        Self(ulid::Ulid::new())
    }

    pub fn from_datetime(datetime: SystemTime) -> Self {
        Self(ulid::Ulid::from_datetime(datetime))
    }

    pub const fn nil() -> Self {
        Self(ulid::Ulid::nil())
    }

    pub const fn is_nil(&self) -> bool {
        self.0.is_nil()
    }

    pub const fn as_u128(&self) -> u128 {
        self.0.0
    }

    pub const fn try_parse(s: &str) -> Result<Uid, ulid::DecodeError> {
        match ulid::Ulid::from_string(s) {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(e),
        }
    }

    pub const fn try_parse_ascii(input: &[u8]) -> Result<Uid, ulid::DecodeError> {
        let s = match std::str::from_utf8(input) {
            Ok(s) => s,
            Err(_) => return Err(ulid::DecodeError::InvalidChar),
        };
        Self::try_parse(s)
    }

    pub fn into_string(self) -> String {
        self.0.to_string()
    }
}

impl fmt::Debug for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Uid").field(&self.into_string()).finish()
    }
}

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.into_string())
    }
}

impl From<ulid::Ulid> for Uid {
    fn from(value: ulid::Ulid) -> Self {
        Self(value)
    }
}

impl From<Uid> for ulid::Ulid {
    fn from(value: Uid) -> Self {
        value.0
    }
}

impl Serialize for Uid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.into_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Uid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Uid::try_parse(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "db")]
mod db_impl {
    use sea_orm::{
        DbErr,
        sea_query::{Nullable, ValueType, ValueTypeErr},
    };

    use super::*;

    impl sea_orm::TryFromU64 for Uid {
        fn try_from_u64(_: u64) -> Result<Self, sea_orm::DbErr> {
            Err(DbErr::ConvertFromU64("Uid"))
        }
    }
    impl std::convert::From<Uid> for sea_orm::Value {
        fn from(source: Uid) -> Self {
            source.into_string().into()
        }
    }
    impl sea_orm::TryGetable for Uid {
        fn try_get_by<I: sea_orm::ColIdx>(
            res: &sea_orm::QueryResult,
            idx: I,
        ) -> std::result::Result<Self, sea_orm::TryGetError> {
            let value = String::try_get_by::<I>(res, idx)?;

            Uid::try_parse(&value).map_err(|e| {
                sea_orm::DbErr::TryIntoErr {
                    from: "String",
                    into: "Uid",
                    source: Box::new(e),
                }
                .into()
            })
        }
    }
    impl ValueType for Uid {
        fn try_from(v: sea_orm::Value) -> Result<Self, ValueTypeErr> {
            let s = <String as ValueType>::try_from(v)?;
            Uid::try_parse(&s).map_err(|_| ValueTypeErr)
        }
        fn type_name() -> std::string::String {
            stringify!(Uid).to_owned()
        }
        fn array_type() -> sea_orm::sea_query::ArrayType {
            String::array_type()
        }
        fn column_type() -> sea_orm::sea_query::ColumnType {
            sea_orm::ColumnType::String(sea_orm::prelude::StringLen::N(ulid::ULID_LEN as u32))
        }
    }
    impl Nullable for Uid {
        fn null() -> sea_orm::Value {
            String::null()
        }
    }
}

#[cfg(feature = "ts")]
mod ts_impl {
    use super::*;

    impl ::ts_rs::TS for Uid {
        type WithoutGenerics = Uid;
        type OptionInnerType = Self;
        fn ident() -> String {
            ("Uid").to_string()
        }
        fn name() -> String {
            ("Uid").to_string()
        }
        fn decl_concrete() -> String {
            format!("type {} = {};", "Uid", <Self as ::ts_rs::TS>::inline())
        }
        fn decl() -> String {
            let inline = <Uid as ::ts_rs::TS>::inline();
            let generics = "";
            format!("type {}{generics} = {inline};", "Uid")
        }
        fn inline() -> String {
            <String as ::ts_rs::TS>::name()
        }
        fn inline_flattened() -> String {
            panic!("{} cannot be flattened", <Self as ::ts_rs::TS>::name())
        }
        fn output_path() -> Option<std::path::PathBuf> {
            Some(std::path::PathBuf::from({
                let dir_or_file = "uid.ts".to_string();
                if dir_or_file.ends_with('/') {
                    format!("{dir_or_file}{}.ts", "Uid")
                } else {
                    dir_or_file.to_string()
                }
            }))
        }
    }
    #[cfg(test)]
    #[test]
    fn export_bindings_uid() {
        <Uid as ::ts_rs::TS>::export_all().expect("could not export type");
    }
}
