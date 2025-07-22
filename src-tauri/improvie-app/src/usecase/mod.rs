pub mod items;
pub mod plays;
pub mod recents;
pub mod rules;
pub mod settings;

macro_rules! def_use_case {
    ($ident:ident) => {
        pub struct $ident<R: RepositoriesModule> {
            repository: std::sync::Arc<R>,
        }

        impl<R: RepositoriesModule> $ident<R> {
            pub fn new(repository: std::sync::Arc<R>) -> Self {
                Self { repository }
            }
        }
    };
}

macro_rules! tx_match {
    ($tx:ident, $result:expr, {$($t:tt)*}) => {
        match $result {
            $($t)*
            Err(err) => {
                $tx.rollback().await?;
                return Err(err);
            }
        }
    };
}

macro_rules! tx_check {
    ($tx:ident, $result:expr) => {
        $crate::usecase::tx_check!($tx, $result, {})
    };
    ($tx:ident, $result:expr, {$($t:tt)*}) => {
        $crate::usecase::tx_match!($tx, $result, {
            $($t)*
            Ok(result) => result,
        })
    };
}

macro_rules! tx_commit {
    ($tx:ident, $result:expr) => {
        $crate::usecase::tx_commit!($tx, $result, {})
    };
    ($tx:ident, $result:expr, {$($t:tt)*}) => {
        $crate::usecase::tx_match!($tx, $result, {
            $($t)*
            Ok(result) => {
                $tx.commit().await?;
                return Ok(result);
            }
        })
    };
}

use def_use_case;
use tx_check;
use tx_commit;
use tx_match;
