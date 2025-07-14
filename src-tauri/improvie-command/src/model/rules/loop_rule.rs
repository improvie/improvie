use crate::state::AppState;

use super::{Rule, RuleFormat, RuleFormatIter};
use serde::{Deserialize, Serialize};
use uid::Uid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub struct LoopRule {
    pub rules: Vec<Rule>,
    pub times: u8,
}

#[async_trait::async_trait]
impl RuleFormatIter for LoopRule {
    async fn formats(&self, state: &AppState) -> Vec<RuleFormat> {
        let mut formats = Vec::new();
        for _ in 0..self.times {
            for rule in &self.rules {
                formats.extend(rule.formats(state).await);
            }
        }
        formats
    }
    async fn thumbnail(&self, state: &AppState) -> Option<Uid> {
        if let Some(rule) = self.rules.first() {
            rule.thumbnail(state).await
        } else {
            None
        }
    }
}
