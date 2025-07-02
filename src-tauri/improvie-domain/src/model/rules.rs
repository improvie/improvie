#[derive(Debug, Clone, serde::Serialize)]
pub struct RuleData {
    pub r#type: String,
    // Json data
    pub data: String,
}

impl<'de> serde::Deserialize<'de> for RuleData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map: serde_json::Map<String, serde_json::Value> =
            serde_json::Map::deserialize(deserializer)?;
        let r#type = map
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let data = map.get("data").map(|v| v.to_string()).unwrap_or_default();
        Ok(RuleData { r#type, data })
    }
}
