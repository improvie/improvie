use crate::state::AppState;

use super::{RuleFormat, RuleFormatIter};
use serde::{Deserialize, Serialize};
use uid::Uid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub struct FolderRule {
    pub target_folder_id: Uid,
}

#[async_trait::async_trait]
impl RuleFormatIter for FolderRule {
    async fn formats(&self, _state: &AppState) -> Vec<RuleFormat> {
        todo!()
    }

    async fn first(&self, _state: &AppState) -> Option<RuleFormat> {
        todo!()
    }
}
