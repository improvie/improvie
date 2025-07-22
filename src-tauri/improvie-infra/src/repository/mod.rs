pub mod items;
pub mod plays;
pub mod recents;
pub mod rules;
pub mod settings;

macro_rules! modify_match {
    ($result:expr, {$($t:tt)*}) => {
        match $result {
            $($t)*
            Err(err) => {
                return Err(improvie_logic::DbErr(err).into());
            }
        }
    };
}

macro_rules! insert_check {
    ($result:expr) => {
        $crate::repository::insert_check!($result, {})
    };
    ($result:expr, {$($t:tt)*}) => {
        $crate::repository::modify_match!($result, {
            Ok(rows_affected) if rows_affected == 0 => {
                return Err(sea_orm::DbErr::RecordNotInserted.into());
            }
            $($t)*
            Ok(_) => {}
        })
    };
}

macro_rules! modify_check {
    ($result:expr) => {
        $crate::repository::modify_check!($result, {})
    };
    ($result:expr, {$($t:tt)*}) => {
        $crate::repository::modify_match!($result, {
            Ok(modify_result) if modify_result.rows_affected == 0 => {
                return Err(sea_orm::DbErr::RecordNotUpdated.into());
            }
            $($t)*
            Ok(_) => {}
        })
    };
}

macro_rules! def_repository_impl {
    ($impl:ident) => {
        pub struct $impl {
            db: $crate::persistence::db::DbPoolImpl,
        }

        impl $impl {
            pub fn new(db: $crate::persistence::db::DbPoolImpl) -> Self {
                Self { db }
            }
        }
    };
}

pub(super) use insert_check;
pub(super) use modify_check;
pub(super) use modify_match;

pub(super) use def_repository_impl;
