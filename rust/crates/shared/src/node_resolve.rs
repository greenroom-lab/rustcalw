//! Node resolution — mirrors src/shared/node-resolve.ts

use crate::node_match::{resolve_node_id_from_candidates, NodeMatchCandidate};

/// Resolve a node ID from a list, with optional default picker.
pub fn resolve_node_id_from_node_list(
    nodes: &[NodeMatchCandidate],
    query: Option<&str>,
    allow_default: bool,
    pick_default: Option<&dyn Fn(&[NodeMatchCandidate]) -> Option<String>>,
) -> Result<String, String> {
    let q = query.unwrap_or("").trim();
    if q.is_empty() {
        if allow_default {
            if let Some(picker) = pick_default {
                if let Some(id) = picker(nodes) {
                    return Ok(id);
                }
            }
        }
        return Err("node required".into());
    }
    resolve_node_id_from_candidates(nodes, q)
}

/// Resolve a full node from a list.
pub fn resolve_node_from_node_list<'a>(
    nodes: &'a [NodeMatchCandidate],
    query: Option<&str>,
    allow_default: bool,
    pick_default: Option<&dyn Fn(&[NodeMatchCandidate]) -> Option<String>>,
) -> Result<&'a NodeMatchCandidate, String> {
    let node_id = resolve_node_id_from_node_list(nodes, query, allow_default, pick_default)?;
    nodes
        .iter()
        .find(|n| n.node_id == node_id)
        .ok_or_else(|| format!("node not found: {}", node_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_with_query() {
        let nodes = vec![NodeMatchCandidate {
            node_id: "abc-123".into(),
            display_name: Some("Test".into()),
            remote_ip: None,
            connected: None,
        }];
        let result = resolve_node_id_from_node_list(&nodes, Some("abc-123"), false, None);
        assert_eq!(result.unwrap(), "abc-123");
    }

    #[test]
    fn resolve_empty_query_no_default() {
        let nodes = vec![];
        let result = resolve_node_id_from_node_list(&nodes, None, false, None);
        assert!(result.is_err());
    }

    #[test]
    fn resolve_empty_query_with_default() {
        let nodes = vec![NodeMatchCandidate {
            node_id: "default-node".into(),
            display_name: None,
            remote_ip: None,
            connected: None,
        }];
        let picker = |_nodes: &[NodeMatchCandidate]| -> Option<String> {
            Some("default-node".into())
        };
        let result = resolve_node_id_from_node_list(&nodes, None, true, Some(&picker));
        assert_eq!(result.unwrap(), "default-node");
    }
}
