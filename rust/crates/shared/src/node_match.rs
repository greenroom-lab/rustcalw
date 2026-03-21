//! Node matching — mirrors src/shared/node-match.ts

use regex::Regex;

/// A candidate for node matching.
#[derive(Debug, Clone)]
pub struct NodeMatchCandidate {
    pub node_id: String,
    pub display_name: Option<String>,
    pub remote_ip: Option<String>,
    pub connected: Option<bool>,
}

/// Normalize a node key to lowercase kebab-case for fuzzy matching.
pub fn normalize_node_key(value: &str) -> String {
    let re = Regex::new(r"[^a-z0-9]+").unwrap();
    let lower = value.to_lowercase();
    let kebab = re.replace_all(&lower, "-").to_string();
    kebab.trim_matches('-').to_string()
}

fn list_known_nodes(nodes: &[NodeMatchCandidate]) -> String {
    nodes
        .iter()
        .filter_map(|n| {
            n.display_name
                .as_deref()
                .or(n.remote_ip.as_deref())
                .or(Some(&n.node_id))
                .filter(|s| !s.is_empty())
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Resolve node matches by exact ID, IP, normalized name, or prefix.
pub fn resolve_node_matches<'a>(
    nodes: &'a [NodeMatchCandidate],
    query: &str,
) -> Vec<&'a NodeMatchCandidate> {
    let q = query.trim();
    if q.is_empty() {
        return vec![];
    }

    let q_norm = normalize_node_key(q);
    nodes
        .iter()
        .filter(|n| {
            // Exact node ID match
            if n.node_id == q {
                return true;
            }
            // Exact IP match
            if n.remote_ip.as_deref() == Some(q) {
                return true;
            }
            // Normalized display name match
            if let Some(ref name) = n.display_name {
                if !name.is_empty() && normalize_node_key(name) == q_norm {
                    return true;
                }
            }
            // Prefix match (at least 6 chars)
            if q.len() >= 6 && n.node_id.starts_with(q) {
                return true;
            }
            false
        })
        .collect()
}

/// Resolve a single node ID from candidates, preferring connected nodes.
pub fn resolve_node_id_from_candidates(
    nodes: &[NodeMatchCandidate],
    query: &str,
) -> Result<String, String> {
    let q = query.trim();
    if q.is_empty() {
        return Err("node required".into());
    }

    let raw_matches = resolve_node_matches(nodes, q);
    if raw_matches.len() == 1 {
        return Ok(raw_matches[0].node_id.clone());
    }
    if raw_matches.is_empty() {
        let known = list_known_nodes(nodes);
        let suffix = if known.is_empty() {
            String::new()
        } else {
            format!(" (known: {})", known)
        };
        return Err(format!("unknown node: {}{}", q, suffix));
    }

    // Prefer connected matches
    let connected: Vec<_> = raw_matches
        .iter()
        .filter(|n| n.connected == Some(true))
        .collect();
    let matches = if !connected.is_empty() {
        connected
    } else {
        raw_matches.iter().collect()
    };

    if matches.len() == 1 {
        return Ok(matches[0].node_id.clone());
    }

    let names: Vec<String> = matches
        .iter()
        .map(|n| {
            n.display_name
                .as_deref()
                .or(n.remote_ip.as_deref())
                .unwrap_or(&n.node_id)
                .to_string()
        })
        .collect();

    Err(format!(
        "ambiguous node: {} (matches: {})",
        q,
        names.join(", ")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(id: &str, name: Option<&str>, connected: Option<bool>) -> NodeMatchCandidate {
        NodeMatchCandidate {
            node_id: id.to_string(),
            display_name: name.map(|s| s.to_string()),
            remote_ip: None,
            connected,
        }
    }

    #[test]
    fn normalize_key() {
        assert_eq!(normalize_node_key("My Device!"), "my-device");
        assert_eq!(normalize_node_key("  hello world  "), "hello-world");
    }

    #[test]
    fn exact_id_match() {
        let nodes = vec![make_node("abc-123", Some("Node"), None)];
        let result = resolve_node_id_from_candidates(&nodes, "abc-123");
        assert_eq!(result.unwrap(), "abc-123");
    }

    #[test]
    fn name_match() {
        let nodes = vec![make_node("abc-123", Some("My Device"), None)];
        let result = resolve_node_id_from_candidates(&nodes, "my device");
        assert_eq!(result.unwrap(), "abc-123");
    }

    #[test]
    fn prefix_match() {
        let nodes = vec![make_node("abc-123-def-456", Some("Node"), None)];
        let result = resolve_node_id_from_candidates(&nodes, "abc-12");
        assert_eq!(result.unwrap(), "abc-123-def-456");
    }

    #[test]
    fn prefix_too_short() {
        let nodes = vec![make_node("abc-123", Some("Node"), None)];
        let result = resolve_node_id_from_candidates(&nodes, "abc");
        assert!(result.is_err());
    }

    #[test]
    fn ambiguous_prefers_connected() {
        let nodes = vec![
            make_node("node-1", Some("Test"), Some(false)),
            make_node("node-2", Some("Test"), Some(true)),
        ];
        let result = resolve_node_id_from_candidates(&nodes, "test");
        assert_eq!(result.unwrap(), "node-2");
    }

    #[test]
    fn unknown_node_error() {
        let nodes = vec![make_node("abc", Some("Node"), None)];
        let result = resolve_node_id_from_candidates(&nodes, "zzz");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown node"));
    }
}
