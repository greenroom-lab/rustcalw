//! Environment variable substitution — mirrors src/config/env-substitution.ts
//!
//! Supports `${VAR_NAME}` syntax in string values, substituted at config load time.
//! - Only uppercase env vars are matched: `[A-Z_][A-Z0-9_]*`
//! - Escape with `$${}` to output literal `${}`
//! - Missing env vars are collected as warnings (non-fatal by default).

use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

static ENV_VAR_NAME_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Z_][A-Z0-9_]*$").unwrap());

/// Warning emitted when an env var reference cannot be resolved.
#[derive(Debug, Clone)]
pub struct EnvSubstitutionWarning {
    pub var_name: String,
    pub config_path: String,
}

/// Token parsed from a `$` position in a string.
enum EnvToken {
    Escaped { name: String, end: usize },
    Substitution { name: String, end: usize },
}

fn parse_env_token_at(value: &str, index: usize) -> Option<EnvToken> {
    let bytes = value.as_bytes();
    if bytes.get(index) != Some(&b'$') {
        return None;
    }

    let next = bytes.get(index + 1).copied();
    let after_next = bytes.get(index + 2).copied();

    // Escaped: $${VAR} -> ${VAR}
    if next == Some(b'$') && after_next == Some(b'{') {
        let start = index + 3;
        if let Some(end) = value[start..].find('}') {
            let end = start + end;
            let name = &value[start..end];
            if ENV_VAR_NAME_PATTERN.is_match(name) {
                return Some(EnvToken::Escaped {
                    name: name.to_string(),
                    end,
                });
            }
        }
    }

    // Substitution: ${VAR} -> value
    if next == Some(b'{') {
        let start = index + 2;
        if let Some(end) = value[start..].find('}') {
            let end = start + end;
            let name = &value[start..end];
            if ENV_VAR_NAME_PATTERN.is_match(name) {
                return Some(EnvToken::Substitution {
                    name: name.to_string(),
                    end,
                });
            }
        }
    }

    None
}

/// Substitute `${VAR}` references in a single string.
fn substitute_string(
    value: &str,
    env: &HashMap<String, String>,
    config_path: &str,
    warnings: &mut Vec<EnvSubstitutionWarning>,
) -> String {
    if !value.contains('$') {
        return value.to_string();
    }

    let mut chunks = Vec::new();
    let mut i = 0;
    let bytes = value.as_bytes();

    while i < bytes.len() {
        if bytes[i] != b'$' {
            chunks.push(value[i..i + 1].to_string());
            i += 1;
            continue;
        }

        match parse_env_token_at(value, i) {
            Some(EnvToken::Escaped { name, end }) => {
                chunks.push(format!("${{{name}}}"));
                i = end + 1;
            }
            Some(EnvToken::Substitution { name, end }) => {
                match env.get(&name) {
                    Some(v) if !v.is_empty() => {
                        chunks.push(v.clone());
                    }
                    _ => {
                        warnings.push(EnvSubstitutionWarning {
                            var_name: name.clone(),
                            config_path: config_path.to_string(),
                        });
                        // Preserve original placeholder
                        chunks.push(format!("${{{name}}}"));
                    }
                }
                i = end + 1;
            }
            None => {
                chunks.push("$".to_string());
                i += 1;
            }
        }
    }

    chunks.join("")
}

/// Check whether a string contains any `${VAR}` references.
pub fn contains_env_var_reference(value: &str) -> bool {
    if !value.contains('$') {
        return false;
    }
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'$' {
            i += 1;
            continue;
        }
        match parse_env_token_at(value, i) {
            Some(EnvToken::Escaped { end, .. }) => {
                i = end + 1;
            }
            Some(EnvToken::Substitution { .. }) => {
                return true;
            }
            None => {
                i += 1;
            }
        }
    }
    false
}

/// Recursively substitute `${VAR}` in all string values of a JSON value.
pub fn resolve_config_env_vars(
    value: &serde_json::Value,
    env: &HashMap<String, String>,
    path: &str,
    warnings: &mut Vec<EnvSubstitutionWarning>,
) -> serde_json::Value {
    match value {
        serde_json::Value::String(s) => {
            let substituted = substitute_string(s, env, path, warnings);
            serde_json::Value::String(substituted)
        }
        serde_json::Value::Array(arr) => {
            let resolved: Vec<serde_json::Value> = arr
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let child_path = format!("{path}[{i}]");
                    resolve_config_env_vars(item, env, &child_path, warnings)
                })
                .collect();
            serde_json::Value::Array(resolved)
        }
        serde_json::Value::Object(obj) => {
            let mut result = serde_json::Map::new();
            for (key, val) in obj {
                let child_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                result.insert(
                    key.clone(),
                    resolve_config_env_vars(val, env, &child_path, warnings),
                );
            }
            serde_json::Value::Object(result)
        }
        // Primitives pass through unchanged
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_env(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn substitute_simple_var() {
        let env = make_env(&[("MY_KEY", "secret123")]);
        let mut warnings = vec![];
        let result = substitute_string("Bearer ${MY_KEY}", &env, "auth.token", &mut warnings);
        assert_eq!(result, "Bearer secret123");
        assert!(warnings.is_empty());
    }

    #[test]
    fn substitute_missing_var_produces_warning() {
        let env = make_env(&[]);
        let mut warnings = vec![];
        let result = substitute_string("${MISSING_VAR}", &env, "some.path", &mut warnings);
        assert_eq!(result, "${MISSING_VAR}");
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].var_name, "MISSING_VAR");
    }

    #[test]
    fn escaped_var_preserved() {
        let env = make_env(&[("MY_KEY", "val")]);
        let mut warnings = vec![];
        let result = substitute_string("$${MY_KEY}", &env, "", &mut warnings);
        assert_eq!(result, "${MY_KEY}");
    }

    #[test]
    fn no_substitution_in_plain_string() {
        let env = make_env(&[]);
        let mut warnings = vec![];
        let result = substitute_string("plain text", &env, "", &mut warnings);
        assert_eq!(result, "plain text");
        assert!(warnings.is_empty());
    }

    #[test]
    fn contains_env_var_reference_works() {
        assert!(contains_env_var_reference("${MY_KEY}"));
        assert!(!contains_env_var_reference("$${MY_KEY}"));
        assert!(!contains_env_var_reference("plain text"));
        assert!(!contains_env_var_reference("$lowercasevar"));
    }

    #[test]
    fn resolve_config_env_vars_recursive() {
        let json = serde_json::json!({
            "a": "${A_VAR}",
            "b": {
                "c": "prefix-${B_VAR}-suffix"
            },
            "d": [1, "${C_VAR}", true]
        });
        let env = make_env(&[("A_VAR", "alpha"), ("B_VAR", "beta"), ("C_VAR", "gamma")]);
        let mut warnings = vec![];
        let result = resolve_config_env_vars(&json, &env, "", &mut warnings);
        assert_eq!(result["a"], "alpha");
        assert_eq!(result["b"]["c"], "prefix-beta-suffix");
        assert_eq!(result["d"][1], "gamma");
        assert!(warnings.is_empty());
    }
}
