macro_rules! def_constant_enum {
    (
        $(
            $(#[$attr:meta])*
            $pub:ident $enum:ident $name:ident
            { $($(#[$field_attr:meta])* $variable:ident = $num:literal,)* }
        )+
    ) => {
        $(
            $(#[$attr])* #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, serde::Serialize, serde::Deserialize)]
            #[cfg_attr(feature = "ts", ts_bind::ts("Constants.ts"))]
            $pub $enum $name {
                $(
                    $(#[$field_attr])*
                    $variable = $num,
                )*
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
        )+
    };
}

pub(super) use def_constant_enum;
