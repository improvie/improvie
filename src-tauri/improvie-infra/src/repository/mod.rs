pub mod health_check;
pub mod items;
pub mod playlists;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");

macro_rules! tx_match {
    ($result:expr,$tx:expr,{$($t:tt)*}) => {
        match $result {
            Ok(r) if r.rows_affected() == 0 => {
                return Err(improvie_logic::AppError::Db(sqlx::Error::RowNotFound));
            }
            $($t)*
            Err(err) => {
                return Err(err.into());
            }
        }
    };
}

macro_rules! tx_check {
    ($result:expr,$tx:expr) => {
        tx_check!($result, $tx, {})
    };
    ($result:expr,$tx:expr,{$($t:tt)*}) => {
        $crate::repository::tx_match!($result, $tx, {
            $($t)*
            Ok(_) => {}
        })
    };
}

macro_rules! def_repository_impl {
    ($impl:ident) => {
        pub struct $impl {
            db: crate::persistence::db::DbPool,
        }

        impl $impl {
            pub fn new(db: crate::persistence::db::DbPool) -> Self {
                Self { db }
            }
        }
    };
}

pub(super) use tx_check;
pub(super) use tx_match;

pub(super) use def_repository_impl;
