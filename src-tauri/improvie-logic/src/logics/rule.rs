use ambassador::{delegatable_trait, Delegate};
use generator::{done, Generator, Gn};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleFormat {
    pub content_uid: Uuid,
    pub range_start: Option<u64>,
    pub range_end: Option<u64>,
}

impl RuleFormat {
    pub fn new(content_uid: Uuid, range_start: Option<u64>, range_end: Option<u64>) -> Self {
        Self {
            content_uid,
            range_start,
            range_end,
        }
    }
}

#[derive(Clone, Delegate, Serialize, Deserialize)]
#[delegate(RuleFormatIter)]
pub enum Rule {
    Content(ContentRule),
    Range(RangeRule),
    Loop(LoopRule),
    Random(RandomRule),
}

#[delegatable_trait]
pub trait RuleFormatIter {
    fn formats(&self) -> Generator<'_, (), RuleFormat>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ContentRule {
    pub content_uid: Uuid,
}

impl RuleFormatIter for ContentRule {
    fn formats(&self) -> Generator<'_, (), RuleFormat> {
        Gn::new_scoped(|mut s| {
            s.yield_with(RuleFormat::new(self.content_uid, None, None));
            done!();
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RangeRule {
    pub content_uid: Uuid,
    pub range_start: Option<u64>,
    pub range_end: Option<u64>,
}

impl RuleFormatIter for RangeRule {
    fn formats(&self) -> Generator<(), RuleFormat> {
        Gn::new_scoped(|mut s| {
            s.yield_with(RuleFormat::new(
                self.content_uid,
                self.range_start,
                self.range_end,
            ));
            done!();
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoopRule {
    pub rules: Vec<Rule>,
    pub times: u8,
}

impl RuleFormatIter for LoopRule {
    fn formats(&self) -> Generator<'_, (), RuleFormat> {
        Gn::new_scoped(|mut s| {
            for _ in 0..self.times {
                for rule in &self.rules {
                    for format in rule.formats() {
                        s.yield_with(format);
                    }
                }
            }
            done!();
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RandomRule {
    // u8 is the weight
    pub rules: Vec<(Rule, u8)>,
    pub times: u8,
    pub duplicate: bool,
}

impl RuleFormatIter for RandomRule {
    fn formats(&self) -> Generator<'_, (), RuleFormat> {
        Gn::new_scoped(|mut s| {
            let rng = &mut rand::thread_rng();
            if self.duplicate {
                for _ in 0..self.times {
                    let Ok((rule, _)) = self.rules.choose_weighted(rng, |item| item.1) else {
                        done!();
                    };

                    for format in rule.formats() {
                        s.yield_with(format);
                    }
                }
            } else {
                let Ok(rules) =
                    self.rules
                        .choose_multiple_weighted(rng, self.times as usize, |item| item.1)
                else {
                    done!();
                };
                for rule in rules {
                    for format in rule.0.formats() {
                        s.yield_with(format);
                    }
                }
            }
            done!();
        })
    }
}
