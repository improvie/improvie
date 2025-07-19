use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts", bind::ts("settings/index.ts"))]
#[cfg_attr(feature = "db", derive(sea_orm::FromJsonQueryResult))]
pub struct AppSettings {}
