use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// sessions.list
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionsListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_minutes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_unknown: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_derived_titles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_last_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawned_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

// ---------------------------------------------------------------------------
// sessions.preview
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionsPreviewParams {
    pub keys: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_chars: Option<u64>,
}

// ---------------------------------------------------------------------------
// sessions.resolve
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionsResolveParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawned_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_unknown: Option<bool>,
}

// ---------------------------------------------------------------------------
// sessions.create
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionsCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_session_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// ---------------------------------------------------------------------------
// sessions.send
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionsSendParams {
    pub key: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

// ---------------------------------------------------------------------------
// sessions.messages.subscribe / unsubscribe
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionsMessagesSubscribeParams {
    pub key: String,
}

pub type SessionsMessagesUnsubscribeParams = SessionsMessagesSubscribeParams;

// ---------------------------------------------------------------------------
// sessions.abort
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionsAbortParams {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

// ---------------------------------------------------------------------------
// sessions.patch
// ---------------------------------------------------------------------------

/// Patch params for sessions. All fields except `key` are optional.
/// NOTE: The TS protocol distinguishes `null` (clear field) from absent
/// (leave unchanged). For now we use `Option<String>`; the null/absent
/// distinction will be handled at the runtime layer via raw JSON if needed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionsPatchParams {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_usage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevated_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_security: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_ask: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawned_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawned_workspace_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_depth: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subagent_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subagent_control_scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_activation: Option<String>,
}

// ---------------------------------------------------------------------------
// sessions.reset / delete / compact
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionsResetParams {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionsDeleteParams {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_transcript: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_lifecycle_hooks: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionsCompactParams {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<u64>,
}

// ---------------------------------------------------------------------------
// sessions.usage
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionsUsageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_context_weight: Option<bool>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sessions_list_params_minimal() {
        let params: SessionsListParams = serde_json::from_value(json!({})).unwrap();
        assert!(params.limit.is_none());
    }

    #[test]
    fn sessions_send_params_roundtrip() {
        let params = SessionsSendParams {
            key: "sess-1".to_string(),
            message: "hello".to_string(),
            thinking: None,
            attachments: None,
            timeout_ms: Some(30000),
            idempotency_key: Some("idem-1".to_string()),
        };
        let json_str = serde_json::to_string(&params).unwrap();
        let parsed: SessionsSendParams = serde_json::from_str(&json_str).unwrap();
        assert_eq!(params, parsed);
    }

    #[test]
    fn sessions_patch_with_values() {
        let json = json!({
            "key": "sess-1",
            "model": "gpt-4",
            "fastMode": true
        });
        let params: SessionsPatchParams = serde_json::from_value(json).unwrap();
        assert_eq!(params.key, "sess-1");
        assert_eq!(params.model, Some("gpt-4".to_string()));
        assert_eq!(params.fast_mode, Some(true));
        assert!(params.label.is_none());
    }

    #[test]
    fn sessions_usage_params_serde() {
        let params = SessionsUsageParams {
            key: Some("sess-1".to_string()),
            start_date: Some("2024-01-01".to_string()),
            end_date: Some("2024-12-31".to_string()),
            mode: Some("utc".to_string()),
            utc_offset: None,
            limit: Some(50),
            include_context_weight: None,
        };
        let json_val = serde_json::to_value(&params).unwrap();
        assert_eq!(json_val["startDate"], "2024-01-01");
    }
}
