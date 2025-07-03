use crate::state::AppState;

use super::{RuleFormat, RuleFormatIter};
use improvie_logic::model::items::ItemNode;
use serde::{Deserialize, Serialize};
use uid::Uid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub struct FolderRule {
    pub target_folder_id: Uid,
}

#[async_trait::async_trait]
impl RuleFormatIter for FolderRule {
    async fn formats(&self, state: &AppState) -> Vec<RuleFormat> {
        // Get the folder node for the target folder
        let Ok(folder_node) = state
            .modules
            .items_use_case()
            .get_items_hierarchy_current(self.target_folder_id)
            .await
        else {
            return vec![RuleFormat::ERROR];
        };

        let mut formats = Vec::new();

        // Iterate through all items in the folder and create RuleFormats for contents
        for item in folder_node.items {
            if let ItemNode::Content { id, .. } = item {
                formats.push(RuleFormat::new(id, None, None));
            }
        }

        formats
    }

    async fn first(&self, state: &AppState) -> Option<RuleFormat> {
        // Get the folder node for the target folder
        let folder_node = state
            .modules
            .items_use_case()
            .get_items_hierarchy_current(self.target_folder_id)
            .await
            .ok()?;

        // Find the first content item in the folder
        for item in folder_node.items {
            if let ItemNode::Content { id, .. } = item {
                return Some(RuleFormat::new(id, None, None));
            }
        }

        None
    }
}
