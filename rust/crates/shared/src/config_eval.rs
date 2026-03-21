//! Config evaluation — mirrors src/shared/config-eval.ts

use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::sync::Mutex;

/// Check if a value is "truthy" (non-null, non-empty, non-zero).
pub fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().map(|f| f != 0.0).unwrap_or(false),
        Value::String(s) => !s.trim().is_empty(),
        _ => true, // arrays, objects are truthy
    }
}

/// Navigate a dot-notation path in a config value.
pub fn resolve_config_path(config: &Value, path_str: &str) -> Option<Value> {
    let parts: Vec<&str> = path_str.split('.').filter(|p| !p.is_empty()).collect();
    let mut current = config;
    for part in parts {
        match current {
            Value::Object(map) => match map.get(part) {
                Some(v) => current = v,
                None => return None,
            },
            _ => return None,
        }
    }
    Some(current.clone())
}

/// Check if a config path is truthy, with optional defaults for undefined paths.
pub fn is_config_path_truthy_with_defaults(
    config: &Value,
    path_str: &str,
    defaults: &HashMap<String, bool>,
) -> bool {
    match resolve_config_path(config, path_str) {
        Some(v) => is_truthy(&v),
        None => defaults.get(path_str).copied().unwrap_or(false),
    }
}

/// Runtime requirement specification.
#[derive(Debug, Clone, Default)]
pub struct RuntimeRequires {
    pub bins: Vec<String>,
    pub any_bins: Vec<String>,
    pub env: Vec<String>,
    pub config: Vec<String>,
}

/// Evaluate whether all runtime requirements are satisfied.
pub fn evaluate_runtime_requires(
    requires: Option<&RuntimeRequires>,
    has_bin: &dyn Fn(&str) -> bool,
    has_remote_bin: Option<&dyn Fn(&str) -> bool>,
    has_any_remote_bin: Option<&dyn Fn(&[String]) -> bool>,
    has_env: &dyn Fn(&str) -> bool,
    is_config_path_truthy: &dyn Fn(&str) -> bool,
) -> bool {
    let requires = match requires {
        Some(r) => r,
        None => return true,
    };

    // Check required bins
    for bin in &requires.bins {
        if has_bin(bin) {
            continue;
        }
        if let Some(remote) = has_remote_bin {
            if remote(bin) {
                continue;
            }
        }
        return false;
    }

    // Check anyBins
    if !requires.any_bins.is_empty() {
        let any_found = requires.any_bins.iter().any(|bin| has_bin(bin));
        if !any_found {
            if let Some(remote) = has_any_remote_bin {
                if !remote(&requires.any_bins) {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    // Check env vars
    for env_name in &requires.env {
        if !has_env(env_name) {
            return false;
        }
    }

    // Check config paths
    for config_path in &requires.config {
        if !is_config_path_truthy(config_path) {
            return false;
        }
    }

    true
}

/// Evaluate runtime eligibility combining OS check and requirements.
pub fn evaluate_runtime_eligibility(
    os: &[String],
    remote_platforms: &[String],
    always: bool,
    requires: Option<&RuntimeRequires>,
    has_bin: &dyn Fn(&str) -> bool,
    has_remote_bin: Option<&dyn Fn(&str) -> bool>,
    has_any_remote_bin: Option<&dyn Fn(&[String]) -> bool>,
    has_env: &dyn Fn(&str) -> bool,
    is_config_path_truthy: &dyn Fn(&str) -> bool,
) -> bool {
    if !os.is_empty() {
        let local = resolve_runtime_platform();
        let local_match = os.iter().any(|p| p == &local);
        let remote_match = remote_platforms.iter().any(|p| os.contains(p));
        if !local_match && !remote_match {
            return false;
        }
    }
    if always {
        return true;
    }
    evaluate_runtime_requires(
        requires,
        has_bin,
        has_remote_bin,
        has_any_remote_bin,
        has_env,
        is_config_path_truthy,
    )
}

/// Get the current runtime platform string.
pub fn resolve_runtime_platform() -> String {
    if cfg!(target_os = "windows") {
        "win32".to_string()
    } else if cfg!(target_os = "macos") {
        "darwin".to_string()
    } else {
        "linux".to_string()
    }
}

/// Windows PATHEXT extensions.
fn windows_path_extensions() -> Vec<String> {
    let raw = env::var("PATHEXT").ok();
    let list: Vec<String> = match raw {
        Some(ref val) => val.split(';').map(|v| v.trim().to_string()).collect(),
        None => vec![
            ".EXE".to_string(),
            ".CMD".to_string(),
            ".BAT".to_string(),
            ".COM".to_string(),
        ],
    };
    let mut result = vec![String::new()];
    result.extend(list.into_iter().filter(|s| !s.is_empty()));
    result
}

static HAS_BINARY_CACHE: Mutex<Option<(String, String, HashMap<String, bool>)>> =
    Mutex::new(None);

/// Check if a binary is available on PATH.
pub fn has_binary(bin: &str) -> bool {
    let path_env = env::var("PATH").unwrap_or_default();
    let path_ext = if cfg!(target_os = "windows") {
        env::var("PATHEXT").unwrap_or_default()
    } else {
        String::new()
    };

    let mut cache = HAS_BINARY_CACHE.lock().unwrap();
    if let Some((ref cached_path, ref cached_ext, ref map)) = *cache {
        if cached_path == &path_env && cached_ext == &path_ext {
            if let Some(&result) = map.get(bin) {
                return result;
            }
        } else {
            *cache = Some((path_env.clone(), path_ext.clone(), HashMap::new()));
        }
    } else {
        *cache = Some((path_env.clone(), path_ext.clone(), HashMap::new()));
    }

    let delimiter = if cfg!(target_os = "windows") { ';' } else { ':' };
    let parts: Vec<&str> = path_env.split(delimiter).filter(|p| !p.is_empty()).collect();
    let extensions = if cfg!(target_os = "windows") {
        windows_path_extensions()
    } else {
        vec![String::new()]
    };

    for part in &parts {
        for ext in &extensions {
            let candidate = Path::new(part).join(format!("{}{}", bin, ext));
            if candidate.exists() {
                if let Some((_, _, ref mut map)) = *cache {
                    map.insert(bin.to_string(), true);
                }
                return true;
            }
        }
    }

    if let Some((_, _, ref mut map)) = *cache {
        map.insert(bin.to_string(), false);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_truthy_values() {
        assert!(!is_truthy(&Value::Null));
        assert!(!is_truthy(&Value::Bool(false)));
        assert!(is_truthy(&Value::Bool(true)));
        assert!(!is_truthy(&Value::Number(0.into())));
        assert!(is_truthy(&Value::Number(1.into())));
        assert!(!is_truthy(&Value::String("".into())));
        assert!(!is_truthy(&Value::String("  ".into())));
        assert!(is_truthy(&Value::String("hello".into())));
        assert!(is_truthy(&Value::Array(vec![])));
    }

    #[test]
    fn resolve_config_path_nested() {
        let config = serde_json::json!({
            "gateway": {
                "port": 3000,
                "tls": {
                    "enabled": true
                }
            }
        });
        assert_eq!(
            resolve_config_path(&config, "gateway.port"),
            Some(Value::Number(3000.into()))
        );
        assert_eq!(
            resolve_config_path(&config, "gateway.tls.enabled"),
            Some(Value::Bool(true))
        );
        assert_eq!(resolve_config_path(&config, "gateway.missing"), None);
    }

    #[test]
    fn config_path_truthy_with_defaults() {
        let config = serde_json::json!({"a": {"b": true}});
        let mut defaults = HashMap::new();
        defaults.insert("x.y".to_string(), true);

        assert!(is_config_path_truthy_with_defaults(&config, "a.b", &defaults));
        assert!(is_config_path_truthy_with_defaults(&config, "x.y", &defaults));
        assert!(!is_config_path_truthy_with_defaults(
            &config,
            "missing",
            &defaults
        ));
    }

    #[test]
    fn evaluate_runtime_requires_no_requires() {
        assert!(evaluate_runtime_requires(
            None,
            &|_| false,
            None,
            None,
            &|_| false,
            &|_| false,
        ));
    }

    #[test]
    fn evaluate_runtime_requires_bins_missing() {
        let req = RuntimeRequires {
            bins: vec!["missing-bin".into()],
            ..Default::default()
        };
        assert!(!evaluate_runtime_requires(
            Some(&req),
            &|_| false,
            None,
            None,
            &|_| false,
            &|_| false,
        ));
    }

    #[test]
    fn evaluate_runtime_requires_bins_present() {
        let req = RuntimeRequires {
            bins: vec!["git".into()],
            ..Default::default()
        };
        assert!(evaluate_runtime_requires(
            Some(&req),
            &|b| b == "git",
            None,
            None,
            &|_| false,
            &|_| false,
        ));
    }
}
