use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// node.pair.*
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodePairRequestParams {
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caps: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodePairListParams {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodePairApproveParams {
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodePairRejectParams {
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodePairVerifyParams {
    pub node_id: String,
    pub token: String,
}

// ---------------------------------------------------------------------------
// node management
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeRenameParams {
    pub node_id: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeListParams {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodePendingAckParams {
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeDescribeParams {
    pub node_id: String,
}

// ---------------------------------------------------------------------------
// node.invoke
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeInvokeParams {
    pub node_id: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeInvokeError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeInvokeResultParams {
    pub id: String,
    pub node_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(rename = "payloadJSON", skip_serializing_if = "Option::is_none")]
    pub payload_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<NodeInvokeError>,
}

// ---------------------------------------------------------------------------
// node.event
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeEventParams {
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(rename = "payloadJSON", skip_serializing_if = "Option::is_none")]
    pub payload_json: Option<String>,
}

// ---------------------------------------------------------------------------
// node.pending.drain / enqueue
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodePendingDrainParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodePendingDrainItem {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String, // "status.request" | "location.request"
    pub priority: String,  // "default" | "normal" | "high"
    pub created_at_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at_ms: Option<Option<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodePendingDrainResult {
    pub node_id: String,
    pub revision: u64,
    pub items: Vec<NodePendingDrainItem>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodePendingEnqueueParams {
    pub node_id: String,
    #[serde(rename = "type")]
    pub item_type: String, // "status.request" | "location.request"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>, // "normal" | "high"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wake: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodePendingEnqueueResult {
    pub node_id: String,
    pub revision: u64,
    pub queued: NodePendingDrainItem,
    pub wake_triggered: bool,
}

// ---------------------------------------------------------------------------
// node.invoke request event (from gateway → node)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeInvokeRequestEvent {
    pub id: String,
    pub node_id: String,
    pub command: String,
    #[serde(rename = "paramsJSON", skip_serializing_if = "Option::is_none")]
    pub params_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn node_pair_request_params_roundtrip() {
        let params = NodePairRequestParams {
            node_id: "node-1".to_string(),
            display_name: Some("My Device".to_string()),
            platform: Some("macos".to_string()),
            version: Some("1.0.0".to_string()),
            core_version: None,
            ui_version: None,
            device_family: None,
            model_identifier: None,
            caps: Some(vec!["tool-events".to_string()]),
            commands: None,
            remote_ip: None,
            silent: None,
        };
        let json_str = serde_json::to_string(&params).unwrap();
        let parsed: NodePairRequestParams = serde_json::from_str(&json_str).unwrap();
        assert_eq!(params, parsed);
    }

    #[test]
    fn node_invoke_params_serde() {
        let params = NodeInvokeParams {
            node_id: "node-1".to_string(),
            command: "system.run".to_string(),
            params: Some(json!({"argv": ["ls", "-la"]})),
            timeout_ms: Some(30000),
            idempotency_key: "idem-1".to_string(),
        };
        let json_val = serde_json::to_value(&params).unwrap();
        assert_eq!(json_val["command"], "system.run");
        assert_eq!(json_val["idempotencyKey"], "idem-1");
    }

    #[test]
    fn node_pending_drain_item_serde() {
        let item = NodePendingDrainItem {
            id: "item-1".to_string(),
            item_type: "status.request".to_string(),
            priority: "normal".to_string(),
            created_at_ms: 1710000000,
            expires_at_ms: Some(Some(1710003600)),
            payload: None,
        };
        let json_val = serde_json::to_value(&item).unwrap();
        assert_eq!(json_val["type"], "status.request");
        assert_eq!(json_val["createdAtMs"], 1710000000);
    }

    #[test]
    fn node_invoke_result_with_error() {
        let result = NodeInvokeResultParams {
            id: "inv-1".to_string(),
            node_id: "node-1".to_string(),
            ok: false,
            payload: None,
            payload_json: None,
            error: Some(NodeInvokeError {
                code: Some("TIMEOUT".to_string()),
                message: Some("Command timed out".to_string()),
            }),
        };
        let json_val = serde_json::to_value(&result).unwrap();
        assert_eq!(json_val["ok"], false);
        assert_eq!(json_val["error"]["code"], "TIMEOUT");
    }
}
