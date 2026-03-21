//! Config UI hint types — mirrors src/shared/config-ui-hints-types.ts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Hints for rendering a single config field in a UI.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    pub order: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_template: Option<serde_json::Value>,
}

/// Map of config keys to their UI hints.
pub type ConfigUiHints = HashMap<String, ConfigUiHint>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_hint() {
        let json = r#"{"label":"Foo","advanced":true}"#;
        let hint: ConfigUiHint = serde_json::from_str(json).unwrap();
        assert_eq!(hint.label.as_deref(), Some("Foo"));
        assert_eq!(hint.advanced, Some(true));
        assert!(hint.help.is_none());
    }

    #[test]
    fn round_trip_hints_map() {
        let mut map = ConfigUiHints::new();
        map.insert(
            "key".to_string(),
            ConfigUiHint {
                label: Some("Label".to_string()),
                ..Default::default()
            },
        );
        let json = serde_json::to_string(&map).unwrap();
        let back: ConfigUiHints = serde_json::from_str(&json).unwrap();
        assert_eq!(back["key"].label.as_deref(), Some("Label"));
    }
}
