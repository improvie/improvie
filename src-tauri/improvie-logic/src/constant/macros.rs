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
            #[cfg_attr(feature = "db", derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum))]
            #[cfg_attr(feature = "db", sea_orm(rs_type = "u8", db_type = "TinyUnsigned"))]
            #[cfg_attr(feature = "ts", bind::ts("constants.ts"))]
            $pub $enum $name {
                $(
                    $(#[$field_attr])*
                    #[cfg_attr(feature = "db", sea_orm(num_value = $num))]
                    $variable = $num,
                )*
            }

            impl TryFrom<u8> for $name {
                type Error = $crate::TryFromConstantEnumError;

                fn try_from(value: u8) -> Result<Self, Self::Error> {
                    match value {
                        $($num => Ok(Self::$variable),)*
                        _ => Err($crate::TryFromConstantEnumError {
                            enum_name: stringify!($enum),
                            value,
                        }),
                    }
                }
            }

            impl From<$name> for u8 {
                fn from(val: $name) -> Self {
                    val as u8
                }
            }

            #[cfg(feature = "db")]
            impl sqlx::Type<sqlx::Sqlite> for $name {
                fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
                    <u8 as sqlx::Type<sqlx::Sqlite>>::type_info()
                }

                fn compatible(ty: &sqlx::sqlite::SqliteTypeInfo) -> bool {
                    <u8 as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
                }
            }

            #[cfg(feature = "db")]
            impl sqlx::Encode<'_, sqlx::Sqlite> for $name {
                fn encode_by_ref(&self, buf: &mut std::vec::Vec<sqlx::sqlite::SqliteArgumentValue<'_>>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                    (*self as u8).encode_by_ref(buf)
                }
            }

            #[cfg(feature = "db")]
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
