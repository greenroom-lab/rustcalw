use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Secrets method params & results
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecretsReloadParams {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SecretsResolveParams {
    pub command_name: String,
    pub target_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SecretsResolveAssignment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub path_segments: Vec<String>,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SecretsResolveResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<SecretsResolveAssignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_ref_paths: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn secrets_resolve_params_roundtrip() {
        let params = SecretsResolveParams {
            command_name: "chat.send".to_string(),
            target_ids: vec!["id-1".to_string(), "id-2".to_string()],
        };
        let json_str = serde_json::to_string(&params).unwrap();
        let parsed: SecretsResolveParams = serde_json::from_str(&json_str).unwrap();
        assert_eq!(params, parsed);
    }

    #[test]
    fn secrets_resolve_result_serde() {
        let result = SecretsResolveResult {
            ok: Some(true),
            assignments: Some(vec![SecretsResolveAssignment {
                path: Some("config.apiKey".to_string()),
                path_segments: vec!["config".to_string(), "apiKey".to_string()],
                value: json!("sk-..."),
            }]),
            diagnostics: None,
            inactive_ref_paths: None,
        };
        let json_val = serde_json::to_value(&result).unwrap();
        assert_eq!(json_val["ok"], true);
        assert_eq!(json_val["assignments"].as_array().unwrap().len(), 1);
    }
}
