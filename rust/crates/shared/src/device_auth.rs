//! Device auth types — mirrors src/shared/device-auth.ts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single device auth entry stored per role.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAuthEntry {
    pub token: String,
    pub role: String,
    pub scopes: Vec<String>,
    pub updated_at_ms: u64,
}

/// Persisted device auth store.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAuthStore {
    pub version: u32,
    pub device_id: String,
    pub tokens: HashMap<String, DeviceAuthEntry>,
}

/// Normalize a device auth role string (trim whitespace).
pub fn normalize_device_auth_role(role: &str) -> String {
    role.trim().to_string()
}

/// Normalize device auth scopes: trim, deduplicate, sort.
pub fn normalize_device_auth_scopes(scopes: Option<&[String]>) -> Vec<String> {
    let Some(scopes) = scopes else {
        return Vec::new();
    };
    let mut out: Vec<String> = scopes
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    out.sort();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_role_trims() {
        assert_eq!(normalize_device_auth_role("  admin  "), "admin");
    }

    #[test]
    fn normalize_scopes_dedup_sort() {
        let scopes = vec![
            " b ".to_string(),
            "a".to_string(),
            "b".to_string(),
            "  ".to_string(),
        ];
        let result = normalize_device_auth_scopes(Some(&scopes));
        assert_eq!(result, vec!["a", "b"]);
    }

    #[test]
    fn normalize_scopes_none() {
        let result = normalize_device_auth_scopes(None);
        assert!(result.is_empty());
    }

    #[test]
    fn store_round_trip() {
        let store = DeviceAuthStore {
            version: 1,
            device_id: "dev-123".to_string(),
            tokens: HashMap::new(),
        };
        let json = serde_json::to_string(&store).unwrap();
        let back: DeviceAuthStore = serde_json::from_str(&json).unwrap();
        assert_eq!(back.device_id, "dev-123");
        assert_eq!(back.version, 1);
    }
}
