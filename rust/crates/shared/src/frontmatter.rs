//! Frontmatter parsing — mirrors src/shared/frontmatter.ts

use serde_json::Value;
use std::collections::HashMap;

/// Normalize input to a list of trimmed, non-empty strings.
/// Arrays are stringified per element; strings are split on commas.
pub fn normalize_string_list(input: &Value) -> Vec<String> {
    match input {
        Value::Null | Value::Bool(_) | Value::Number(_) => vec![],
        Value::String(s) => s
            .split(',')
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .collect(),
        Value::Array(arr) => arr
            .iter()
            .map(|v| match v {
                Value::String(s) => s.trim().to_string(),
                other => other.to_string().trim().to_string(),
            })
            .filter(|v| !v.is_empty())
            .collect(),
        Value::Object(_) => vec![],
    }
}

/// Extract a string value from a frontmatter map.
pub fn get_frontmatter_string(frontmatter: &HashMap<String, Value>, key: &str) -> Option<String> {
    match frontmatter.get(key) {
        Some(Value::String(s)) => Some(s.clone()),
        _ => None,
    }
}

/// Parse a boolean value from string, with fallback.
pub fn parse_frontmatter_bool(value: Option<&str>, fallback: bool) -> bool {
    match value {
        Some(v) => match v.trim().to_lowercase().as_str() {
            "true" | "yes" | "1" | "on" => true,
            "false" | "no" | "0" | "off" => false,
            _ => fallback,
        },
        None => fallback,
    }
}

/// Manifest keys, current and legacy.
const MANIFEST_KEY: &str = "openclaw";
const LEGACY_MANIFEST_KEYS: &[&str] = &["clawdbot", "moldbot", "moltbot"];

/// Resolve the openclaw manifest block from frontmatter metadata.
/// Parses JSON from the `metadata` (or custom `key`) field, then searches
/// for `openclaw` / legacy manifest keys within the parsed object.
pub fn resolve_openclaw_manifest_block(
    frontmatter: &HashMap<String, Value>,
    key: Option<&str>,
) -> Option<HashMap<String, Value>> {
    let raw = get_frontmatter_string(frontmatter, key.unwrap_or("metadata"))?;
    let parsed: Value = serde_json::from_str(&raw).ok()?;
    let obj = parsed.as_object()?;

    let manifest_keys = std::iter::once(MANIFEST_KEY).chain(LEGACY_MANIFEST_KEYS.iter().copied());
    for mk in manifest_keys {
        if let Some(Value::Object(candidate)) = obj.get(mk) {
            return Some(candidate.clone().into_iter().collect());
        }
    }
    None
}

/// Parsed requires block from manifest.
#[derive(Debug, Clone, Default)]
pub struct OpenClawManifestRequires {
    pub bins: Vec<String>,
    pub any_bins: Vec<String>,
    pub env: Vec<String>,
    pub config: Vec<String>,
}

/// Extract requires from a manifest metadata object.
pub fn resolve_openclaw_manifest_requires(
    metadata_obj: &HashMap<String, Value>,
) -> Option<OpenClawManifestRequires> {
    let requires_val = metadata_obj.get("requires")?;
    let requires_obj = requires_val.as_object()?;

    Some(OpenClawManifestRequires {
        bins: requires_obj
            .get("bins")
            .map(|v| normalize_string_list(v))
            .unwrap_or_default(),
        any_bins: requires_obj
            .get("anyBins")
            .map(|v| normalize_string_list(v))
            .unwrap_or_default(),
        env: requires_obj
            .get("env")
            .map(|v| normalize_string_list(v))
            .unwrap_or_default(),
        config: requires_obj
            .get("config")
            .map(|v| normalize_string_list(v))
            .unwrap_or_default(),
    })
}

/// Extract OS constraints from a manifest metadata object.
pub fn resolve_openclaw_manifest_os(metadata_obj: &HashMap<String, Value>) -> Vec<String> {
    metadata_obj
        .get("os")
        .map(|v| normalize_string_list(v))
        .unwrap_or_default()
}

/// Parsed install base spec from manifest.
#[derive(Debug, Clone)]
pub struct ParsedOpenClawManifestInstallBase {
    pub raw: HashMap<String, Value>,
    pub kind: String,
    pub id: Option<String>,
    pub label: Option<String>,
    pub bins: Option<Vec<String>>,
}

/// Parse an install spec entry, validating the kind/type against allowed kinds.
pub fn parse_openclaw_manifest_install_base(
    input: &Value,
    allowed_kinds: &[&str],
) -> Option<ParsedOpenClawManifestInstallBase> {
    let obj = input.as_object()?;

    let kind_raw = obj
        .get("kind")
        .and_then(|v| v.as_str())
        .or_else(|| obj.get("type").and_then(|v| v.as_str()))
        .unwrap_or("");
    let kind = kind_raw.trim().to_lowercase();

    if !allowed_kinds.contains(&kind.as_str()) {
        return None;
    }

    let raw: HashMap<String, Value> = obj.clone().into_iter().collect();
    let id = obj.get("id").and_then(|v| v.as_str()).map(String::from);
    let label = obj.get("label").and_then(|v| v.as_str()).map(String::from);
    let bins_list = obj.get("bins").map(|v| normalize_string_list(v));
    let bins = bins_list.filter(|b| !b.is_empty());

    Some(ParsedOpenClawManifestInstallBase {
        raw,
        kind,
        id,
        label,
        bins,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_string_list_from_csv() {
        let input = Value::String("git, node, cargo".into());
        assert_eq!(normalize_string_list(&input), vec!["git", "node", "cargo"]);
    }

    #[test]
    fn normalize_string_list_from_array() {
        let input = Value::Array(vec![
            Value::String("git".into()),
            Value::String("  node  ".into()),
        ]);
        assert_eq!(normalize_string_list(&input), vec!["git", "node"]);
    }

    #[test]
    fn normalize_string_list_empty() {
        assert!(normalize_string_list(&Value::Null).is_empty());
    }

    #[test]
    fn parse_frontmatter_bool_cases() {
        assert!(parse_frontmatter_bool(Some("true"), false));
        assert!(!parse_frontmatter_bool(Some("false"), true));
        assert!(parse_frontmatter_bool(Some("YES"), false));
        assert!(parse_frontmatter_bool(None, true));
        assert!(!parse_frontmatter_bool(Some("unknown"), false));
    }

    #[test]
    fn parse_install_base_valid() {
        let input = serde_json::json!({
            "kind": "NPM",
            "id": "my-pkg",
            "label": "My Package",
            "bins": ["my-bin"]
        });
        let result = parse_openclaw_manifest_install_base(&input, &["npm"]);
        assert!(result.is_some());
        let spec = result.unwrap();
        assert_eq!(spec.kind, "npm");
        assert_eq!(spec.id.as_deref(), Some("my-pkg"));
        assert_eq!(spec.bins.as_ref().unwrap(), &vec!["my-bin".to_string()]);
    }

    #[test]
    fn parse_install_base_rejects_unknown_kind() {
        let input = serde_json::json!({"kind": "unknown"});
        let result = parse_openclaw_manifest_install_base(&input, &["npm", "pip"]);
        assert!(result.is_none());
    }
}
