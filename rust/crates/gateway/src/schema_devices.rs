use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Device pairing method params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DevicePairListParams {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DevicePairApproveParams {
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DevicePairRejectParams {
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DevicePairRemoveParams {
    pub device_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTokenRotateParams {
    pub device_id: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTokenRevokeParams {
    pub device_id: String,
    pub role: String,
}

// ---------------------------------------------------------------------------
// Device pairing events
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DevicePairRequestedEvent {
    pub request_id: String,
    pub device_id: String,
    pub public_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_repair: Option<bool>,
    pub ts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DevicePairResolvedEvent {
    pub request_id: String,
    pub device_id: String,
    pub decision: String,
    pub ts: u64,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn device_pair_approve_roundtrip() {
        let params = DevicePairApproveParams {
            request_id: "req-1".to_string(),
        };
        let json_str = serde_json::to_string(&params).unwrap();
        let parsed: DevicePairApproveParams = serde_json::from_str(&json_str).unwrap();
        assert_eq!(params, parsed);
    }

    #[test]
    fn device_pair_requested_event_serde() {
        let event = DevicePairRequestedEvent {
            request_id: "req-1".to_string(),
            device_id: "dev-1".to_string(),
            public_key: "pk-abc".to_string(),
            display_name: Some("My Phone".to_string()),
            platform: Some("ios".to_string()),
            device_family: None,
            client_id: None,
            client_mode: None,
            role: None,
            roles: None,
            scopes: None,
            remote_ip: None,
            silent: None,
            is_repair: None,
            ts: 1710000000,
        };
        let json_val = serde_json::to_value(&event).unwrap();
        assert_eq!(json_val["requestId"], "req-1");
        assert_eq!(json_val["displayName"], "My Phone");
        assert_eq!(json_val["ts"], 1710000000);
        assert!(json_val.get("deviceFamily").is_none());
    }

    #[test]
    fn device_pair_resolved_event_serde() {
        let event = DevicePairResolvedEvent {
            request_id: "req-1".to_string(),
            device_id: "dev-1".to_string(),
            decision: "approved".to_string(),
            ts: 1710000000,
        };
        let json_str = serde_json::to_string(&event).unwrap();
        let parsed: DevicePairResolvedEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(event, parsed);
    }

    #[test]
    fn device_token_rotate_with_scopes() {
        let params = DeviceTokenRotateParams {
            device_id: "dev-1".to_string(),
            role: "operator".to_string(),
            scopes: Some(vec!["admin".to_string(), "read".to_string()]),
        };
        let json_val = serde_json::to_value(&params).unwrap();
        assert_eq!(json_val["scopes"].as_array().unwrap().len(), 2);
    }
}
