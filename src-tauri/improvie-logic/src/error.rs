use more_convert::VariantName;

// TODO: もっといい感じのエラーにする

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error, more_convert::VariantName)]
#[variant_name(prefix = "App")]
pub enum AppError {
    #[error("db error: {0}")]
    #[cfg(feature = "db")]
    Db(#[from] sqlx::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("not ready on {0}: {1}")]
    NotReady(&'static str, String),
    #[error("errored on {0}: {1}")]
    Errored(&'static str, String),
}

crate::impl_serialize_for_dyn_app_error!(AppError);

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

impl<T> DynAppError for T
where
    T: std::error::Error + Send + Sync + 'static + VariantName,
{
    fn error_kind(&self) -> &'static str {
        self.variant_name()
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
