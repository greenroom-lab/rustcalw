use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schema_primitives::InputProvenance;

// ---------------------------------------------------------------------------
// AgentInternalEvent
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentInternalEvent {
    #[serde(rename = "type")]
    pub event_type: String, // "task_completion"
    pub source: String,     // "subagent" | "cron"
    pub child_session_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_session_id: Option<String>,
    pub announce_type: String,
    pub task_label: String,
    pub status: String, // "ok" | "timeout" | "error" | "unknown"
    pub status_label: String,
    pub result: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_line: Option<String>,
    pub reply_instruction: String,
}

// ---------------------------------------------------------------------------
// AgentEvent (streaming)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentEvent {
    pub run_id: String,
    pub seq: u64,
    pub stream: String,
    pub ts: u64,
    pub data: std::collections::HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// send
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendParams {
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_playback: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
    pub idempotency_key: String,
}

// ---------------------------------------------------------------------------
// poll
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PollParams {
    pub to: String,
    pub question: String,
    pub options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selections: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_hours: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    pub idempotency_key: String,
}

// ---------------------------------------------------------------------------
// agent (run an agent turn)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentParams {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_space: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_effort_deliver: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lane: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_events: Option<Vec<AgentInternalEvent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_provenance: Option<InputProvenance>,
    pub idempotency_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

// ---------------------------------------------------------------------------
// agent.identity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentIdentityParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentIdentityResult {
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
}

// ---------------------------------------------------------------------------
// agent.wait
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentWaitParams {
    pub run_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

// ---------------------------------------------------------------------------
// wake
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WakeParams {
    pub mode: String, // "now" | "next-heartbeat"
    pub text: String,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn send_params_roundtrip() {
        let params = SendParams {
            to: "user-1".to_string(),
            message: Some("hello".to_string()),
            media_url: None,
            media_urls: None,
            gif_playback: None,
            channel: Some("telegram".to_string()),
            account_id: None,
            agent_id: None,
            thread_id: None,
            session_key: None,
            idempotency_key: "idem-1".to_string(),
        };
        let json_str = serde_json::to_string(&params).unwrap();
        let parsed: SendParams = serde_json::from_str(&json_str).unwrap();
        assert_eq!(params, parsed);
    }

    #[test]
    fn poll_params_serde() {
        let params = PollParams {
            to: "group-1".to_string(),
            question: "What's for lunch?".to_string(),
            options: vec!["Pizza".to_string(), "Sushi".to_string()],
            max_selections: Some(1),
            duration_seconds: Some(3600),
            duration_hours: None,
            silent: None,
            is_anonymous: None,
            thread_id: None,
            channel: None,
            account_id: None,
            idempotency_key: "idem-2".to_string(),
        };
        let json_val = serde_json::to_value(&params).unwrap();
        assert_eq!(json_val["options"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn agent_event_serde() {
        let event = AgentEvent {
            run_id: "run-1".to_string(),
            seq: 0,
            stream: "main".to_string(),
            ts: 1710000000,
            data: {
                let mut m = std::collections::HashMap::new();
                m.insert("text".to_string(), json!("hello"));
                m
            },
        };
        let json_str = serde_json::to_string(&event).unwrap();
        let parsed: AgentEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event, parsed);
    }

    #[test]
    fn agent_identity_result_serde() {
        let result = AgentIdentityResult {
            agent_id: "main".to_string(),
            name: Some("My Agent".to_string()),
            avatar: None,
            emoji: Some("🤖".to_string()),
        };
        let json_val = serde_json::to_value(&result).unwrap();
        assert_eq!(json_val["agentId"], "main");
        assert_eq!(json_val["emoji"], "🤖");
        assert!(json_val.get("avatar").is_none());
    }
}
