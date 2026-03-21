use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schema_primitives::InputProvenance;

// ---------------------------------------------------------------------------
// logs.tail
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LogsTailParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LogsTailResult {
    pub file: String,
    pub cursor: u64,
    pub size: u64,
    pub lines: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<bool>,
}

// ---------------------------------------------------------------------------
// chat.history
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChatHistoryParams {
    pub session_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

// ---------------------------------------------------------------------------
// chat.send
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChatSendParams {
    pub session_key: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_input_provenance: Option<InputProvenance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_provenance_receipt: Option<String>,
    pub idempotency_key: String,
}

// ---------------------------------------------------------------------------
// chat.abort
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChatAbortParams {
    pub session_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

// ---------------------------------------------------------------------------
// chat.inject
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChatInjectParams {
    pub session_key: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

// ---------------------------------------------------------------------------
// ChatEvent (streaming)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChatEvent {
    pub run_id: String,
    pub session_key: String,
    pub seq: u64,
    pub state: String, // "delta" | "final" | "aborted" | "error"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn logs_tail_params_minimal() {
        let params: LogsTailParams = serde_json::from_value(json!({})).unwrap();
        assert!(params.cursor.is_none());
    }

    #[test]
    fn logs_tail_result_roundtrip() {
        let result = LogsTailResult {
            file: "/var/log/gateway.log".to_string(),
            cursor: 100,
            size: 5000,
            lines: vec!["line 1".to_string(), "line 2".to_string()],
            truncated: None,
            reset: None,
        };
        let json_str = serde_json::to_string(&result).unwrap();
        let parsed: LogsTailResult = serde_json::from_str(&json_str).unwrap();
        assert_eq!(result, parsed);
    }

    #[test]
    fn chat_send_params_serde() {
        let params = ChatSendParams {
            session_key: "sess-1".to_string(),
            message: "hello".to_string(),
            thinking: None,
            deliver: Some(true),
            attachments: None,
            timeout_ms: None,
            system_input_provenance: None,
            system_provenance_receipt: None,
            idempotency_key: "idem-1".to_string(),
        };
        let json_val = serde_json::to_value(&params).unwrap();
        assert_eq!(json_val["sessionKey"], "sess-1");
        assert_eq!(json_val["deliver"], true);
    }

    #[test]
    fn chat_event_serde() {
        let event = ChatEvent {
            run_id: "run-1".to_string(),
            session_key: "sess-1".to_string(),
            seq: 0,
            state: "delta".to_string(),
            message: Some(json!({"text": "hi"})),
            error_message: None,
            usage: None,
            stop_reason: None,
        };
        let json_str = serde_json::to_string(&event).unwrap();
        let parsed: ChatEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event, parsed);
    }
}
