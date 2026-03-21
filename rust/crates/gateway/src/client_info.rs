use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Gateway client IDs
// ---------------------------------------------------------------------------

pub const WEBCHAT_UI: &str = "webchat-ui";
pub const CONTROL_UI: &str = "openclaw-control-ui";
pub const WEBCHAT: &str = "webchat";
pub const CLI: &str = "cli";
pub const GATEWAY_CLIENT: &str = "gateway-client";
pub const MACOS_APP: &str = "openclaw-macos";
pub const IOS_APP: &str = "openclaw-ios";
pub const ANDROID_APP: &str = "openclaw-android";
pub const NODE_HOST: &str = "node-host";
pub const TEST: &str = "test";
pub const FINGERPRINT: &str = "fingerprint";
pub const PROBE: &str = "openclaw-probe";

const CLIENT_IDS: &[&str] = &[
    WEBCHAT_UI,
    CONTROL_UI,
    WEBCHAT,
    CLI,
    GATEWAY_CLIENT,
    MACOS_APP,
    IOS_APP,
    ANDROID_APP,
    NODE_HOST,
    TEST,
    FINGERPRINT,
    PROBE,
];

/// Back-compat alias.
pub use self::{
    WEBCHAT_UI as WEBCHAT_UI_NAME,
    CONTROL_UI as CONTROL_UI_NAME,
};

// ---------------------------------------------------------------------------
// Gateway client modes
// ---------------------------------------------------------------------------

pub mod modes {
    pub const WEBCHAT: &str = "webchat";
    pub const CLI: &str = "cli";
    pub const UI: &str = "ui";
    pub const BACKEND: &str = "backend";
    pub const NODE: &str = "node";
    pub const PROBE: &str = "probe";
    pub const TEST: &str = "test";
}

const CLIENT_MODES: &[&str] = &[
    modes::WEBCHAT,
    modes::CLI,
    modes::UI,
    modes::BACKEND,
    modes::NODE,
    modes::PROBE,
    modes::TEST,
];

// ---------------------------------------------------------------------------
// Gateway client capabilities
// ---------------------------------------------------------------------------

pub mod caps {
    pub const TOOL_EVENTS: &str = "tool-events";
}

// ---------------------------------------------------------------------------
// GatewayClientInfo
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GatewayClientInfo {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub version: String,
    pub platform: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_identifier: Option<String>,
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Normalization helpers
// ---------------------------------------------------------------------------

/// Normalise a raw string to a known `GatewayClientId`, returning `None`
/// if the value is empty or unrecognised.
pub fn normalize_gateway_client_id(raw: Option<&str>) -> Option<&'static str> {
    let normalized = raw?.trim();
    if normalized.is_empty() {
        return None;
    }
    let lower = normalized.to_ascii_lowercase();
    CLIENT_IDS.iter().find(|&&id| id == lower).copied()
}

/// Back-compat alias.
pub fn normalize_gateway_client_name(raw: Option<&str>) -> Option<&'static str> {
    normalize_gateway_client_id(raw)
}

/// Normalise a raw string to a known `GatewayClientMode`.
pub fn normalize_gateway_client_mode(raw: Option<&str>) -> Option<&'static str> {
    let normalized = raw?.trim();
    if normalized.is_empty() {
        return None;
    }
    let lower = normalized.to_ascii_lowercase();
    CLIENT_MODES.iter().find(|&&m| m == lower).copied()
}

/// Check whether a capability is present in the given caps list.
pub fn has_gateway_client_cap(caps: Option<&[String]>, cap: &str) -> bool {
    match caps {
        Some(list) => list.iter().any(|c| c == cap),
        None => false,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_known_client_id() {
        assert_eq!(normalize_gateway_client_id(Some("CLI")), Some(CLI));
        assert_eq!(normalize_gateway_client_id(Some(" webchat-ui ")), Some(WEBCHAT_UI));
        assert_eq!(normalize_gateway_client_id(Some("OpenClaw-Probe")), Some(PROBE));
    }

    #[test]
    fn normalize_unknown_client_id() {
        assert_eq!(normalize_gateway_client_id(Some("unknown")), None);
        assert_eq!(normalize_gateway_client_id(Some("")), None);
        assert_eq!(normalize_gateway_client_id(None), None);
    }

    #[test]
    fn normalize_known_mode() {
        assert_eq!(normalize_gateway_client_mode(Some("CLI")), Some(modes::CLI));
        assert_eq!(normalize_gateway_client_mode(Some("webchat")), Some(modes::WEBCHAT));
    }

    #[test]
    fn normalize_unknown_mode() {
        assert_eq!(normalize_gateway_client_mode(Some("magic")), None);
        assert_eq!(normalize_gateway_client_mode(None), None);
    }

    #[test]
    fn cap_check() {
        let caps = vec!["tool-events".to_string()];
        assert!(has_gateway_client_cap(Some(&caps), caps::TOOL_EVENTS));
        assert!(!has_gateway_client_cap(Some(&caps), "other"));
        assert!(!has_gateway_client_cap(None, caps::TOOL_EVENTS));
    }

    #[test]
    fn client_info_serde_roundtrip() {
        let info = GatewayClientInfo {
            id: CLI.to_string(),
            display_name: None,
            version: "1.0.0".to_string(),
            platform: "linux".to_string(),
            device_family: None,
            model_identifier: None,
            mode: modes::CLI.to_string(),
            instance_id: None,
        };
        let json = serde_json::to_string(&info).unwrap();
        let parsed: GatewayClientInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, parsed);
    }
}
