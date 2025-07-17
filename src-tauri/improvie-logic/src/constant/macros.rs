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
            #[cfg_attr(feature = "ts", bind::ts("constants.ts"))]
            $pub $enum $name {
                $(
                    $(#[$field_attr])*
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

            #[cfg(feature = "db")]
            impl From<$name> for sea_orm::Value {
                fn from(val: $name) -> Self {
                    sea_orm::Value::TinyUnsigned(Some(val.into()))
                }
            }

            #[cfg(feature = "db")]
            impl sea_orm::sea_query::Nullable for $name {
                fn null() -> sea_orm::Value {
                    <u8 as sea_orm::sea_query::Nullable>::null()
                }
            }

            #[cfg(feature = "db")]
            impl sea_orm::sea_query::ValueType for $name {
                fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                    let num = <u8 as sea_orm::sea_query::ValueType>::try_from(v)?;
                    num.try_into().map_err(|_| sea_orm::sea_query::ValueTypeErr)
                }

                fn type_name() -> String {
                    stringify!($name).to_owned()
                }

                fn array_type() -> sea_orm::sea_query::ArrayType {
                    <u8 as sea_orm::sea_query::ValueType>::array_type()
                }

                fn column_type() -> sea_orm::sea_query::ColumnType {
                    <u8 as sea_orm::sea_query::ValueType>::column_type()
                }
            }

            #[cfg(feature = "db")]
            impl sea_orm::TryGetable for $name {
                fn try_get_by<I: sea_orm::ColIdx>(
                    res: &sea_orm::QueryResult,
                    index: I,
                ) -> Result<Self, sea_orm::TryGetError> {
                    let value = <u8 as sea_orm::TryGetable>::try_get_by(res, index)?;
                    value.try_into().map_err(|e| sea_orm::DbErr::TryIntoErr {
                        from: "u8",
                        into: stringify!($name),
                        source: Box::new(e),
                    }.into())
                }
            }
        )+
    };
}

pub(super) use def_constant_enum;
