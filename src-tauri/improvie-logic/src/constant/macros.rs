macro_rules! def_constant_enum {
    (
        $(
            $(#[$attr:meta])*
            $pub:ident $enum:ident $name:ident
            { $($(#[$field_attr:meta])* $variable:ident = $num:literal,)* }
        )+
    ) => {
        $(
            $(#[$attr])* #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, serde::Serialize, serde::Deserialize)]
            #[repr(u8)]
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
        )+
    };
}

pub(super) use def_constant_enum;
