use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("settings/index.ts"))]
pub struct AppSettings {}
