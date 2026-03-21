use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// PresenceEntry
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PresenceEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_input_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub ts: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

// ---------------------------------------------------------------------------
// SessionDefaults
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionDefaults {
    pub default_agent_id: String,
    pub main_key: String,
    pub main_session_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

// ---------------------------------------------------------------------------
// UpdateAvailable
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAvailable {
    pub current_version: String,
    pub latest_version: String,
    pub channel: String,
}

// ---------------------------------------------------------------------------
// Snapshot
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub presence: Vec<PresenceEntry>,
    pub health: Value, // HealthSnapshot is opaque
    pub state_version: crate::protocol_frames::StateVersion,
    pub uptime_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_defaults: Option<SessionDefaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_available: Option<UpdateAvailable>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn presence_entry_minimal() {
        let entry = PresenceEntry {
            host: None,
            ip: None,
            version: None,
            platform: None,
            device_family: None,
            model_identifier: None,
            mode: Some("cli".to_string()),
            last_input_seconds: None,
            reason: None,
            tags: None,
            text: None,
            ts: 1710000000,
            device_id: None,
            roles: None,
            scopes: None,
            instance_id: None,
        };
        let json_val = serde_json::to_value(&entry).unwrap();
        assert_eq!(json_val["ts"], 1710000000);
        assert_eq!(json_val["mode"], "cli");
        assert!(json_val.get("host").is_none());
    }

    #[test]
    fn snapshot_roundtrip() {
        let snap = Snapshot {
            presence: vec![],
            health: json!({}),
            state_version: crate::protocol_frames::StateVersion {
                presence: 1,
                health: 1,
            },
            uptime_ms: 5000,
            config_path: Some("/etc/openclaw.toml".to_string()),
            state_dir: None,
            session_defaults: Some(SessionDefaults {
                default_agent_id: "main".to_string(),
                main_key: "default".to_string(),
                main_session_key: "main:default".to_string(),
                scope: Some("per-sender".to_string()),
            }),
            auth_mode: Some("token".to_string()),
            update_available: None,
        };
        let json_str = serde_json::to_string(&snap).unwrap();
        let parsed: Snapshot = serde_json::from_str(&json_str).unwrap();
        assert_eq!(snap, parsed);
    }

    #[test]
    fn session_defaults_serde() {
        let defaults = SessionDefaults {
            default_agent_id: "main".to_string(),
            main_key: "key".to_string(),
            main_session_key: "main:key".to_string(),
            scope: None,
        };
        let json_val = serde_json::to_value(&defaults).unwrap();
        assert_eq!(json_val["defaultAgentId"], "main");
        assert_eq!(json_val["mainSessionKey"], "main:key");
    }
}
