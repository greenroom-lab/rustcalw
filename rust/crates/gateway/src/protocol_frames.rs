use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error_codes::ErrorShape;

// ---------------------------------------------------------------------------
// Connect params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectDeviceInfo {
    pub id: String,
    pub public_key: String,
    pub signature: String,
    pub signed_at: u64,
    pub nonce: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectClientInfo {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectParams {
    pub min_protocol: u32,
    pub max_protocol: u32,
    pub client: ConnectClientInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caps: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<serde_json::Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_env: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<ConnectDeviceInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<ConnectAuth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

// ---------------------------------------------------------------------------
// Hello OK
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HelloOkServer {
    pub version: String,
    pub conn_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HelloOkFeatures {
    pub methods: Vec<String>,
    pub events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HelloOkAuth {
    pub device_token: String,
    pub role: String,
    pub scopes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HelloOkPolicy {
    pub max_payload: u64,
    pub max_buffered_bytes: u64,
    pub tick_interval_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HelloOk {
    #[serde(rename = "type")]
    pub frame_type: String, // always "hello-ok"
    pub protocol: u32,
    pub server: HelloOkServer,
    pub features: HelloOkFeatures,
    pub snapshot: Value, // Snapshot — complex nested type, use Value for now
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_host_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<HelloOkAuth>,
    pub policy: HelloOkPolicy,
}

// ---------------------------------------------------------------------------
// State version
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StateVersion {
    pub presence: u64,
    pub health: u64,
}

// ---------------------------------------------------------------------------
// Wire frames
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestFrame {
    #[serde(rename = "type")]
    pub frame_type: String, // "req"
    pub id: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResponseFrame {
    #[serde(rename = "type")]
    pub frame_type: String, // "res"
    pub id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorShape>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventFrame {
    #[serde(rename = "type")]
    pub frame_type: String, // "event"
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_version: Option<StateVersion>,
}

/// Discriminated union of all top-level gateway frames.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum GatewayFrame {
    #[serde(rename = "req")]
    Request(RequestFrameInner),
    #[serde(rename = "res")]
    Response(ResponseFrameInner),
    #[serde(rename = "event")]
    Event(EventFrameInner),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestFrameInner {
    pub id: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResponseFrameInner {
    pub id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorShape>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventFrameInner {
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_version: Option<StateVersion>,
}

// ---------------------------------------------------------------------------
// Tick / Shutdown events
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TickEvent {
    pub ts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShutdownEvent {
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_expected_ms: Option<u64>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn request_frame_serde() {
        let frame = RequestFrame {
            frame_type: "req".to_string(),
            id: "r1".to_string(),
            method: "health".to_string(),
            params: None,
        };
        let json_val = serde_json::to_value(&frame).unwrap();
        assert_eq!(json_val["type"], "req");
        assert_eq!(json_val["method"], "health");
    }

    #[test]
    fn response_frame_with_error() {
        let frame = ResponseFrame {
            frame_type: "res".to_string(),
            id: "r1".to_string(),
            ok: false,
            payload: None,
            error: Some(ErrorShape {
                code: "NOT_LINKED".to_string(),
                message: "Gateway is not linked".to_string(),
                details: None,
                retryable: None,
                retry_after_ms: None,
            }),
        };
        let json_str = serde_json::to_string(&frame).unwrap();
        assert!(json_str.contains("NOT_LINKED"));
    }

    #[test]
    fn gateway_frame_discriminated_union() {
        let req_json = json!({
            "type": "req",
            "id": "1",
            "method": "health"
        });
        let frame: GatewayFrame = serde_json::from_value(req_json).unwrap();
        match frame {
            GatewayFrame::Request(r) => {
                assert_eq!(r.method, "health");
            }
            _ => panic!("expected Request"),
        }

        let event_json = json!({
            "type": "event",
            "event": "tick",
            "payload": {"ts": 1234}
        });
        let frame: GatewayFrame = serde_json::from_value(event_json).unwrap();
        match frame {
            GatewayFrame::Event(e) => {
                assert_eq!(e.event, "tick");
            }
            _ => panic!("expected Event"),
        }
    }

    #[test]
    fn tick_event_serde() {
        let tick = TickEvent { ts: 1710000000 };
        let json_str = serde_json::to_string(&tick).unwrap();
        let parsed: TickEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(tick, parsed);
    }

    #[test]
    fn shutdown_event_serde() {
        let shutdown = ShutdownEvent {
            reason: "update".to_string(),
            restart_expected_ms: Some(5000),
        };
        let json_val = serde_json::to_value(&shutdown).unwrap();
        assert_eq!(json_val["reason"], "update");
        assert_eq!(json_val["restartExpectedMs"], 5000);
    }

    #[test]
    fn connect_params_minimal() {
        let params = ConnectParams {
            min_protocol: 1,
            max_protocol: 1,
            client: ConnectClientInfo {
                id: "cli".to_string(),
                display_name: None,
                version: "1.0.0".to_string(),
                platform: "linux".to_string(),
                device_family: None,
                model_identifier: None,
                mode: "cli".to_string(),
                instance_id: None,
            },
            caps: None,
            commands: None,
            permissions: None,
            path_env: None,
            role: None,
            scopes: None,
            device: None,
            auth: None,
            locale: None,
            user_agent: None,
        };
        let json_val = serde_json::to_value(&params).unwrap();
        assert_eq!(json_val["minProtocol"], 1);
        assert_eq!(json_val["client"]["id"], "cli");
        // Optional fields should be absent
        assert!(json_val.get("caps").is_none());
        assert!(json_val.get("device").is_none());
    }

    #[test]
    fn state_version_serde() {
        let sv = StateVersion {
            presence: 42,
            health: 7,
        };
        let json_str = serde_json::to_string(&sv).unwrap();
        let parsed: StateVersion = serde_json::from_str(&json_str).unwrap();
        assert_eq!(sv, parsed);
    }
}
