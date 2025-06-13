pub trait DynAppError: std::error::Error + Send + Sync + 'static {
    fn error_kind(&self) -> &'static str;

    #[inline(always)]
    fn boxed(self) -> Box<dyn DynAppError>
    where
        Self: std::marker::Sized,
    {
        Box::new(self)
    }
}

impl<E> From<E> for Box<dyn DynAppError>
where
    E: DynAppError,
{
    fn from(error: E) -> Self {
        error.boxed()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct BoxDynAppError(Box<dyn DynAppError>);

impl BoxDynAppError {
    #[inline(always)]
    pub fn new<E>(error: E) -> Self
    where
        E: DynAppError,
    {
        Self(error.boxed())
    }
}

impl From<Box<dyn DynAppError>> for BoxDynAppError {
    #[inline(always)]
    fn from(error: Box<dyn DynAppError>) -> Self {
        Self(error)
    }
}

impl<E> From<E> for BoxDynAppError
where
    E: DynAppError,
{
    #[inline(always)]
    fn from(error: E) -> Self {
        Self::new(error)
    }
}

crate::impl_serialize_for_dyn_app_error!(BoxDynAppError, 0);

pub type DynAppResult<T> = std::result::Result<T, BoxDynAppError>;

#[cfg(feature = "db")]
mod db {
    #[derive(Debug, thiserror::Error)]
    #[error("Database error: {0}")]
    pub struct DbErr(pub sqlx::Error);

    crate::impl_serialize_for_dyn_app_error!(DbErr);

    impl From<sqlx::Error> for DbErr {
        fn from(error: sqlx::Error) -> Self {
            Self(error)
        }
    }

    impl From<sqlx::Error> for super::BoxDynAppError {
        fn from(error: sqlx::Error) -> Self {
            super::BoxDynAppError::new(DbErr::from(error))
        }
    }

    impl super::DynAppError for DbErr {
        fn error_kind(&self) -> &'static str {
            "DbErr"
        }
    }
}

#[cfg(feature = "db")]
pub use db::*;

mod macros {
    #[macro_export]
    macro_rules! def_unit_dyn_app_error {
        (
            $(
                $(#[$attr:meta])*
                $vis:vis struct $ident:ident = $error:literal;
            )+
        ) => {
            $(
                #[derive(Debug)] $(#[$($attr)*])*
                $vis struct $ident;

                impl $crate::DynAppError for $ident {
                    fn error_kind(&self) -> &'static str {
                        stringify!($ident)
                    }
                }

                impl std::error::Error for $ident {}

                impl std::fmt::Display for $ident {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str($error)
                    }
                }

                $crate::impl_serialize_for_dyn_app_error!($ident);
            )+
        };
    }

    #[macro_export]
    macro_rules! impl_serialize_for_dyn_app_error {
        ($error:ident, $($self:tt)*) => {
            impl serde::Serialize for $error {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    #[allow(unused_imports)]
                    use $crate::DynAppError;

                    let mut serde_state = serializer.serialize_struct(stringify!($error), 2)?;
                    serde_state.serialize_field("kind", self$(.$self)*.error_kind())?;
                    serde_state.serialize_field("message", &self$(.$self)*.to_string())?;
                    serde_state.end()
                }
            }
        };
        ($error:ident) => {
            $crate::impl_serialize_for_dyn_app_error!($error, );
        }
    }
}
