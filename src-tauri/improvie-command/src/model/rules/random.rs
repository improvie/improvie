use crate::state::AppState;

use super::{Rule, RuleFormat, RuleFormatIter};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use uid::Uid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub struct RandomRule {
    // u8 is the weight
    pub rules: Vec<(Rule, u8)>,
    pub times: u8,
    pub duplicate: bool,
}

#[async_trait::async_trait]
impl RuleFormatIter for RandomRule {
    async fn formats(&self, state: &AppState) -> Vec<RuleFormat> {
        let mut formats = Vec::new();
        if self.duplicate {
            for _ in 0..self.times {
                let Ok((rule, _)) = self.rules.choose_weighted(&mut rand::rng(), |item| item.1)
                else {
                    return formats;
                };

                for format in rule.formats(state).await {
                    formats.push(format);
                }
            }
        } else {
            let Ok(rules) = self.rules.choose_multiple_weighted(
                &mut rand::rng(),
                self.times as usize,
                |item| item.1,
            ) else {
                return formats;
            };
            for (rule, _) in rules {
                for format in rule.formats(state).await {
                    formats.push(format);
                }
            }
        }

        formats
    }

    async fn thumbnail(&self, state: &AppState) -> Option<Uid> {
        if let Some((rule, _)) = self.rules.first() {
            rule.thumbnail(state).await
        } else {
            None
        }
    }
}
