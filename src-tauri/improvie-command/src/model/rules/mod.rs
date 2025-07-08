use improvie_domain::model::rules::RuleData;
use serde::{Deserialize, Serialize};

use uid::Uid;

mod loop_rule;
pub use loop_rule::*;

mod random;
pub use random::*;

mod folder;
pub use folder::*;

use crate::state::AppState;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub struct RuleFormat {
    pub content_id: Uid,
    pub range_start: Option<u32>,
    pub range_end: Option<u32>,
}

impl RuleFormat {
    pub const ERROR: Self = Self {
        content_id: Uid::nil(),
        range_start: None,
        range_end: None,
    };

    pub fn new(content_id: Uid, range_start: Option<u32>, range_end: Option<u32>) -> Self {
        Self {
            content_id,
            range_start,
            range_end,
        }
    }

    pub const fn is_error(&self) -> bool {
        self.content_id.is_nil()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
#[cfg_attr(feature = "ts", ts(rename = "RuleType"))]
#[serde(tag = "type", content = "data")]
pub enum Rule {
    Content(ContentRule),
    Range(RangeRule),
    Loop(LoopRule),
    Random(RandomRule),
    Folder(FolderRule),
    Unknown,
}

impl Rule {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Rule::Unknown)
    }

    pub fn from_data(data: RuleData) -> Self {
        serde_json::from_str(&data.data).unwrap_or(Rule::Unknown)
    }
}

#[async_trait::async_trait]
pub trait RuleFormatIter {
    async fn formats(&self, state: &AppState) -> Vec<RuleFormat>;
    async fn thumbnail(&self, state: &AppState) -> Option<Uid>;
}

#[async_trait::async_trait]
impl RuleFormatIter for Rule {
    async fn formats(&self, state: &AppState) -> Vec<RuleFormat> {
        match self {
            Rule::Content(rule) => rule.formats(state).await,
            Rule::Range(rule) => rule.formats(state).await,
            Rule::Loop(rule) => rule.formats(state).await,
            Rule::Random(rule) => rule.formats(state).await,
            Rule::Folder(rule) => rule.formats(state).await,
            Rule::Unknown => Vec::new(),
        }
    }

    async fn thumbnail(&self, state: &AppState) -> Option<Uid> {
        match self {
            Rule::Content(rule) => rule.thumbnail(state).await,
            Rule::Range(rule) => rule.thumbnail(state).await,
            Rule::Loop(rule) => rule.thumbnail(state).await,
            Rule::Random(rule) => rule.thumbnail(state).await,
            Rule::Folder(rule) => rule.thumbnail(state).await,
            Rule::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub struct ContentRule {
    pub content_id: Uid,
}

#[async_trait::async_trait]
impl RuleFormatIter for ContentRule {
    async fn formats(&self, _: &AppState) -> Vec<RuleFormat> {
        vec![RuleFormat::new(self.content_id, None, None)]
    }
    async fn thumbnail(&self, _: &AppState) -> Option<Uid> {
        Some(self.content_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub struct RangeRule {
    pub content_id: Uid,
    pub range_start: Option<u32>,
    pub range_end: Option<u32>,
}

#[async_trait::async_trait]
impl RuleFormatIter for RangeRule {
    async fn formats(&self, _: &AppState) -> Vec<RuleFormat> {
        vec![RuleFormat::new(
            self.content_id,
            self.range_start,
            self.range_end,
        )]
    }
    async fn thumbnail(&self, _: &AppState) -> Option<Uid> {
        Some(self.content_id)
    }
}
