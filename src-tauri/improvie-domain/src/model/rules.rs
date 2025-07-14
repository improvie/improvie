use serde::de::{self};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct RuleData {
    pub r#type: String,
    pub data: String,
}

impl Serialize for RuleData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde_json::Value;

        let mut state = serializer.serialize_struct("RuleData", 2)?;
        state.serialize_field("type", &self.r#type)?;

        let data = serde_json::from_str::<Value>(&self.data).map_err(serde::ser::Error::custom)?;
        state.serialize_field("data", &data)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for RuleData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RuleDataHelper {
            r#type: String,
            data: serde_json::Value,
        }

        let helper = RuleDataHelper::deserialize(deserializer)?;

        let data = serde_json::to_string(&helper.data).map_err(de::Error::custom)?;

        Ok(RuleData {
            r#type: helper.r#type,
            data,
        })
    }
}

impl RuleData {
    pub fn into_value(self) -> serde_json::Result<serde_json::Value> {
        let mut value = serde_json::Map::new();
        value.insert("type".to_string(), serde_json::Value::String(self.r#type));

        let data_value: serde_json::Value = serde_json::from_str(&self.data)?;

        value.insert("data".to_string(), data_value);

        Ok(serde_json::Value::Object(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() {
        let rule_data = RuleData {
            r#type: "Content".to_string(),
            data: r#"{"content_id":"12345","range_end":10,"range_start":0}"#.to_string(),
        };

        let serialized = serde_json::to_string(&rule_data).unwrap();
        let deserialized: RuleData = serde_json::from_str(&serialized).unwrap();

        assert_eq!(rule_data.r#type, deserialized.r#type);
        assert_eq!(rule_data.data, deserialized.data);
    }

    #[test]
    fn into_value() {
        let rule_data = RuleData {
            r#type: "Content".to_string(),
            data: r#"{"content_id":"12345","range_end":10,"range_start":0}"#.to_string(),
        };

        let value = rule_data.into_value().unwrap();
        let expected = serde_json::json!({
            "type": "Content",
            "data": {
                "content_id": "12345",
                "range_end": 10,
                "range_start": 0
            }
        });

        assert_eq!(value, expected);
    }
}
