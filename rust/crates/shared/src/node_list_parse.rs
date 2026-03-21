//! Node list parsing — mirrors src/shared/node-list-parse.ts

use crate::node_list_types::{NodeListNode, PairedNode, PairingList, PendingRequest};
use serde_json::Value;

/// Parse a pairing list from a JSON value.
pub fn parse_pairing_list(value: &Value) -> PairingList {
    let obj = value.as_object();
    let pending: Vec<PendingRequest> = obj
        .and_then(|o| o.get("pending"))
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    let paired: Vec<PairedNode> = obj
        .and_then(|o| o.get("paired"))
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    PairingList { pending, paired }
}

/// Parse a node list from a JSON value.
pub fn parse_node_list(value: &Value) -> Vec<NodeListNode> {
    value
        .as_object()
        .and_then(|o| o.get("nodes"))
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_pairing_list() {
        let val = serde_json::json!({});
        let list = parse_pairing_list(&val);
        assert!(list.pending.is_empty());
        assert!(list.paired.is_empty());
    }

    #[test]
    fn parse_node_list_from_object() {
        let val = serde_json::json!({
            "nodes": [
                {"nodeId": "abc", "displayName": "Test Node"}
            ]
        });
        let nodes = parse_node_list(&val);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].node_id, "abc");
    }

    #[test]
    fn parse_node_list_missing_key() {
        let val = serde_json::json!({"other": 123});
        let nodes = parse_node_list(&val);
        assert!(nodes.is_empty());
    }
}
