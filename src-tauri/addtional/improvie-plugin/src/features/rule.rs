use serde::{Deserialize, Serialize, de::DeserializeOwned};

use uid::Uid;

// TODO: もっといい感じに修正する、serdeはSized要求するからsea_ormとかそこら辺を参考にRuleを実装する

pub struct RuleFeature<R: RuleGenerator> {
    pub name: &'static str,
    pub generator: R,
}

impl<R: RuleGenerator> RuleFeature<R> {
    pub fn new(generator: R) -> Self {
        Self {
            name: R::name(),
            generator,
        }
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
    Unknown(UnknownRule),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub struct UnknownRule {
    pub plugin_name: String,
    pub rule_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub struct ContentRule {
    pub content_id: Uid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub struct RangeRule {
    pub content_id: Uid,
    pub range_start: Option<u32>,
    pub range_end: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub struct LoopRule {
    pub rules: Vec<Rule>,
    pub times: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub struct RandomRule {
    /// u8 is the weight
    pub rules: Vec<(u8, Rule)>,
    pub times: u8,
    pub duplicate: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleGeneratorRaw {
    pub plugin_name: String,
    pub rule_name: String,
    pub json: String,
}

pub trait RuleTypeable: Serialize + DeserializeOwned {
    fn types() -> &'static [(&'static str, RuleDataType)];
}

pub trait RuleGenerator: RuleTypeable + Send + Sync + 'static {
    fn name() -> &'static str;

    fn generate_rules(&mut self) -> Vec<Rule>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ts", bind::ts("rule.ts"))]
pub enum RuleDataType {
    /// The value brought is u64
    Unsigned { min: u64, max: u64 },
    /// The value brought is i64
    Signed { min: i64, max: i64 },
    /// The value brought is f64
    Float { min: f64, max: f64 },
    /// The value brought is a String
    Input {
        min_length: usize,
        max_length: usize,
    },
    /// The value brought is a bool
    CheckBox,
    /// The value brought is a Ulid
    ContentPicker,
    // TODO: radio and select
}
