use more_convert::VariantName;

pub mod rules;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error, more_convert::VariantName)]
#[variant_name(prefix = "App")]
pub enum AppError {
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
}

crate::impl_serialize_for_dyn_app_error!(AppError);

pub trait DynAppError: std::error::Error + Send + Sync + 'static {
    fn error_kind(&self) -> &'static str;

    #[inline(always)]
    fn boxed(self) -> Box<Self>
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
        ($error:ident) => {
            impl serde::Serialize for $error {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    use $crate::DynAppError;

                    let mut serde_state = serializer.serialize_struct(stringify!($error), 2)?;
                    serde_state.serialize_field("kind", self.error_kind())?;
                    serde_state.serialize_field("message", &self.to_string())?;
                    serde_state.end()
                }
            }
        };
    }
}
