//! Config file I/O — mirrors the core of src/config/io.ts
//!
//! Handles reading, parsing, and resolving OpenClaw config files.

use crate::env_substitution::resolve_config_env_vars;
use crate::paths;
use crate::types::openclaw::{ConfigFileSnapshot, ConfigValidationIssue, OpenClawConfig};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Legacy config directory names.
const LEGACY_STATE_DIRNAMES: &[&str] = &[".clawdbot", ".moldbot", ".moltbot"];
const CONFIG_FILENAME: &str = "openclaw.json";
const LEGACY_CONFIG_FILENAMES: &[&str] = &["clawdbot.json", "moldbot.json", "moltbot.json"];

/// Resolve the state directory.
/// Priority: OPENCLAW_STATE_DIR env > existing ~/.openclaw > existing legacy > new ~/.openclaw
pub fn resolve_state_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("OPENCLAW_STATE_DIR") {
        let trimmed = dir.trim().to_string();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    if let Ok(dir) = std::env::var("CLAWDBOT_STATE_DIR") {
        let trimmed = dir.trim().to_string();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    let new_dir = paths::config_dir();
    if new_dir.exists() {
        return new_dir;
    }

    // Check legacy dirs
    if let Some(home) = paths::home_dir() {
        for legacy in LEGACY_STATE_DIRNAMES {
            let dir = home.join(legacy);
            if dir.exists() {
                return dir;
            }
        }
    }

    new_dir
}

/// Resolve config file path candidates.
fn resolve_config_candidates(state_dir: &Path) -> Vec<PathBuf> {
    let mut candidates = vec![];
    candidates.push(state_dir.join(CONFIG_FILENAME));
    for name in LEGACY_CONFIG_FILENAMES {
        candidates.push(state_dir.join(name));
    }

    // Also check legacy dirs
    if let Some(home) = paths::home_dir() {
        for legacy_dir_name in LEGACY_STATE_DIRNAMES {
            let legacy_dir = home.join(legacy_dir_name);
            candidates.push(legacy_dir.join(CONFIG_FILENAME));
            for name in LEGACY_CONFIG_FILENAMES {
                candidates.push(legacy_dir.join(name));
            }
        }
    }

    candidates
}

/// Resolve the active config file path.
/// Priority: OPENCLAW_CONFIG_PATH env > existing candidates > canonical path.
pub fn resolve_config_path() -> PathBuf {
    if let Ok(path) = std::env::var("OPENCLAW_CONFIG_PATH") {
        let trimmed = path.trim().to_string();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    let state_dir = resolve_state_dir();
    let candidates = resolve_config_candidates(&state_dir);

    for candidate in &candidates {
        if candidate.exists() {
            return candidate.clone();
        }
    }

    // Canonical default
    state_dir.join(CONFIG_FILENAME)
}

/// Read a config file and return a snapshot.
pub fn read_config_file(path: &Path) -> Result<ConfigFileSnapshot> {
    let exists = path.exists();
    if !exists {
        return Ok(ConfigFileSnapshot {
            path: path.display().to_string(),
            exists: false,
            raw: None,
            parsed: None,
            resolved: OpenClawConfig::default(),
            valid: true,
            config: OpenClawConfig::default(),
            hash: None,
            issues: vec![],
            warnings: vec![],
            legacy_issues: vec![],
        });
    }

    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;

    let hash = format!("{:x}", md5_hash(raw.as_bytes()));

    // Parse JSON
    let parsed: serde_json::Value = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse config JSON: {}", path.display()))?;

    // Env var substitution
    let env: HashMap<String, String> = std::env::vars().collect();
    let mut env_warnings = vec![];
    let resolved_value = resolve_config_env_vars(&parsed, &env, "", &mut env_warnings);

    // Map to OpenClawConfig
    let config: OpenClawConfig = serde_json::from_value(resolved_value.clone())
        .with_context(|| "failed to map config JSON to OpenClawConfig")?;

    // Also map the resolved value (before defaults) for "resolved" field
    let resolved: OpenClawConfig = serde_json::from_value(resolved_value)
        .unwrap_or_default();

    let warnings: Vec<ConfigValidationIssue> = env_warnings
        .iter()
        .map(|w| ConfigValidationIssue {
            path: w.config_path.clone(),
            message: format!("unresolved env var: ${{{}}}", w.var_name),
            allowed_values: None,
            allowed_values_hidden_count: None,
        })
        .collect();

    Ok(ConfigFileSnapshot {
        path: path.display().to_string(),
        exists: true,
        raw: Some(raw),
        parsed: Some(parsed),
        resolved,
        valid: true,
        config,
        hash: Some(hash),
        issues: vec![],
        warnings,
        legacy_issues: vec![],
    })
}

/// Load the config from the default path.
pub fn load_config() -> Result<ConfigFileSnapshot> {
    let path = resolve_config_path();
    tracing::info!(path = %path.display(), "loading config file");
    read_config_file(&path)
}

/// Simple hash for config change detection.
fn md5_hash(data: &[u8]) -> u64 {
    // Simple non-crypto hash (matches the concept, not the exact algo)
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

/// Resolve the gateway port from config and environment.
pub const DEFAULT_GATEWAY_PORT: u16 = 18789;

pub fn resolve_gateway_port(config: Option<&OpenClawConfig>) -> u16 {
    // Check env first
    if let Ok(raw) = std::env::var("OPENCLAW_GATEWAY_PORT") {
        if let Ok(port) = raw.trim().parse::<u16>() {
            if port > 0 {
                return port;
            }
        }
    }
    if let Ok(raw) = std::env::var("CLAWDBOT_GATEWAY_PORT") {
        if let Ok(port) = raw.trim().parse::<u16>() {
            if port > 0 {
                return port;
            }
        }
    }

    // Then config
    if let Some(cfg) = config {
        if let Some(gw) = &cfg.gateway {
            if let Some(port) = gw.port {
                if port > 0 {
                    return port;
                }
            }
        }
    }

    DEFAULT_GATEWAY_PORT
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn read_nonexistent_config() {
        let path = Path::new("/tmp/nonexistent-rustcalw-config.json");
        let snapshot = read_config_file(path).unwrap();
        assert!(!snapshot.exists);
        assert!(snapshot.valid);
    }

    #[test]
    fn read_minimal_config() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, r#"{{"gateway":{{"port":18789}}}}"#).unwrap();
        let snapshot = read_config_file(file.path()).unwrap();
        assert!(snapshot.exists);
        assert!(snapshot.valid);
        assert_eq!(snapshot.config.gateway.as_ref().unwrap().port, Some(18789));
    }

    #[test]
    fn resolve_gateway_port_from_config() {
        let config = OpenClawConfig {
            gateway: Some(crate::types::gateway::GatewayConfig {
                port: Some(9999),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(resolve_gateway_port(Some(&config)), 9999);
    }

    #[test]
    fn resolve_gateway_port_default() {
        assert_eq!(resolve_gateway_port(None), DEFAULT_GATEWAY_PORT);
    }
}
