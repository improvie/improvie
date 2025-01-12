pub mod health_check;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");

macro_rules! tx_match {
    ($result:expr,$tx:expr,{$($t:tt)*}) => {
        match $result {
            Ok(r) if r.rows_affected() == 0 => {
                $tx.rollback().await?;
                return Err(improvie_logic::error::AppError::DbRowNotFound);
            }
            $($t)*
            Err(err) => {
                $tx.rollback().await?;
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

macro_rules! tx_commit {
    ($result:expr,$tx:expr) => {
        tx_commit!($result, $tx, {})
    };
    ($result:expr,$tx:expr,{$($t:tt)*}) => {
        $crate::repository::tx_match!($result, $tx, {
            $($t)*
            Ok(_) => {
                return $tx.commit().await.map_err(Into::into);
            }
        })
    };
}

pub(crate) use tx_check;
pub(crate) use tx_commit;
pub(crate) use tx_match;
