use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

use uid::Uid;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", ts_bind::ts("Rule.ts"))]
pub struct RuleFormat {
    pub content_id: Uid,
    pub range_start: Option<u32>,
    pub range_end: Option<u32>,
}

impl RuleFormat {
    pub fn new(content_id: Uid, range_start: Option<u32>, range_end: Option<u32>) -> Self {
        Self {
            content_id,
            range_start,
            range_end,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", ts_bind::ts("Rule.ts"))]
#[cfg_attr(feature = "ts", ts(rename = "RuleType"))]
#[serde(tag = "type", content = "data")]
pub enum Rule {
    Content(ContentRule),
    Range(RangeRule),
    Loop(LoopRule),
    Random(RandomRule),
}

pub trait RuleFormatIter {
    fn formats(&self) -> Vec<RuleFormat>;
}

impl RuleFormatIter for Rule {
    fn formats(&self) -> Vec<RuleFormat> {
        match self {
            Rule::Content(rule) => rule.formats(),
            Rule::Range(rule) => rule.formats(),
            Rule::Loop(rule) => rule.formats(),
            Rule::Random(rule) => rule.formats(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", ts_bind::ts("Rule.ts"))]
pub struct ContentRule {
    pub content_id: Uid,
}

impl RuleFormatIter for ContentRule {
    fn formats(&self) -> Vec<RuleFormat> {
        vec![RuleFormat::new(self.content_id, None, None)]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", ts_bind::ts("Rule.ts"))]
pub struct RangeRule {
    pub content_id: Uid,
    pub range_start: Option<u32>,
    pub range_end: Option<u32>,
}

impl RuleFormatIter for RangeRule {
    fn formats(&self) -> Vec<RuleFormat> {
        vec![RuleFormat::new(
            self.content_id,
            self.range_start,
            self.range_end,
        )]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", ts_bind::ts("Rule.ts"))]
pub struct LoopRule {
    pub rules: Vec<Rule>,
    pub times: u8,
}

impl RuleFormatIter for LoopRule {
    fn formats(&self) -> Vec<RuleFormat> {
        let mut formats = Vec::new();
        for _ in 0..self.times {
            for rule in &self.rules {
                formats.extend(rule.formats());
            }
        }
        formats
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", ts_bind::ts("Rule.ts"))]
pub struct RandomRule {
    // u8 is the weight
    pub rules: Vec<(Rule, u8)>,
    pub times: u8,
    pub duplicate: bool,
}

impl RuleFormatIter for RandomRule {
    fn formats(&self) -> Vec<RuleFormat> {
        let rng = &mut rand::rng();
        let mut formats = Vec::new();
        if self.duplicate {
            for _ in 0..self.times {
                let Ok((rule, _)) = self.rules.choose_weighted(rng, |item| item.1) else {
                    return formats;
                };

                for format in rule.formats() {
                    formats.push(format);
                }
            }
        } else {
            let Ok(rules) = self
                .rules
                .choose_multiple_weighted(rng, self.times as usize, |item| item.1)
            else {
                return formats;
            };
            for (rule, _) in rules {
                for format in rule.formats() {
                    formats.push(format);
                }
            }
        }

        formats
    }
}
