use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Config method params & results
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigGetParams {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSetParams {
    pub raw: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigApplyParams {
    pub raw: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay_ms: Option<u64>,
}

pub type ConfigPatchParams = ConfigApplyParams;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigSchemaParams {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSchemaLookupParams {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRunParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

// ---------------------------------------------------------------------------
// ConfigUiHint
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigUiHint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_template: Option<Value>,
}

// ---------------------------------------------------------------------------
// Config schema response types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSchemaResponse {
    pub schema: Value,
    pub ui_hints: std::collections::HashMap<String, ConfigUiHint>,
    pub version: String,
    pub generated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSchemaLookupChild {
    pub key: String,
    pub path: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<SchemaTypeValue>,
    pub required: bool,
    pub has_children: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<ConfigUiHint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint_path: Option<String>,
}

/// The `type` field can be a single string or an array of strings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SchemaTypeValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSchemaLookupResult {
    pub path: String,
    pub schema: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<ConfigUiHint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint_path: Option<String>,
    pub children: Vec<ConfigSchemaLookupChild>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn config_set_params_roundtrip() {
        let params = ConfigSetParams {
            raw: "key: value".to_string(),
            base_hash: Some("abc123".to_string()),
        };
        let json_str = serde_json::to_string(&params).unwrap();
        let parsed: ConfigSetParams = serde_json::from_str(&json_str).unwrap();
        assert_eq!(params, parsed);
    }

    #[test]
    fn config_apply_params_minimal() {
        let params = ConfigApplyParams {
            raw: "key: value".to_string(),
            base_hash: None,
            session_key: None,
            note: None,
            restart_delay_ms: None,
        };
        let json_val = serde_json::to_value(&params).unwrap();
        assert_eq!(json_val["raw"], "key: value");
        assert!(json_val.get("baseHash").is_none());
    }

    #[test]
    fn config_ui_hint_serde() {
        let hint = ConfigUiHint {
            label: Some("My Label".to_string()),
            help: None,
            tags: Some(vec!["tag1".to_string()]),
            group: None,
            order: Some(1),
            advanced: None,
            sensitive: Some(true),
            placeholder: None,
            item_template: None,
        };
        let json_str = serde_json::to_string(&hint).unwrap();
        let parsed: ConfigUiHint = serde_json::from_str(&json_str).unwrap();
        assert_eq!(hint, parsed);
    }

    #[test]
    fn schema_type_value_single() {
        let val: SchemaTypeValue = serde_json::from_value(json!("string")).unwrap();
        assert_eq!(val, SchemaTypeValue::Single("string".to_string()));
    }

    #[test]
    fn schema_type_value_multiple() {
        let val: SchemaTypeValue = serde_json::from_value(json!(["string", "number"])).unwrap();
        match val {
            SchemaTypeValue::Multiple(v) => assert_eq!(v.len(), 2),
            _ => panic!("expected Multiple"),
        }
    }
}
