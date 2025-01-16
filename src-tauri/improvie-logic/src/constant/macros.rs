macro_rules! def_constant_enum {
    (
        $(#[$attr:meta])*
        $pub:ident $enum:ident $name:ident
        { $($variable:ident = $num:literal,)* }
    ) => {
        $(#[$attr])* $pub $enum $name {
            $($variable = $num,)*
        }

        impl TryFrom<u8> for $name {
            type Error = String;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                Ok(match value {
                    $($num => Self::$variable,)*
                    _ => return Err(format!("invalid {}: {value}", stringify!($name))),
                })
            }
        }

        impl From<$name> for u8 {
            fn from(val: $name) -> Self {
                val as u8
            }
        }

        impl sqlx::Type<sqlx::Sqlite> for $name {
            fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
                <u8 as sqlx::Type<sqlx::Sqlite>>::type_info()
            }

            fn compatible(ty: &sqlx::sqlite::SqliteTypeInfo) -> bool {
                <u8 as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
            }
        }

        impl sqlx::Encode<'_, sqlx::Sqlite> for $name {
            fn encode_by_ref(&self, buf: &mut std::vec::Vec<sqlx::sqlite::SqliteArgumentValue<'_>>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                (*self as u8).encode_by_ref(buf)
            }
        }

        impl sqlx::Decode<'_, sqlx::Sqlite> for $name {
            fn decode(value: sqlx::sqlite::SqliteValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                let num = <u8 as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;

                num.try_into().map_err(Into::into)
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_u8(*self as u8)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let deserialized_u8 = u8::deserialize(deserializer)?;
                deserialized_u8.try_into().map_err(serde::de::Error::custom)
            }
        }
    };
}

pub(super) use def_constant_enum;
