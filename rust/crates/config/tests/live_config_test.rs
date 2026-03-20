//! Live config integration tests.
//!
//! These tests run against the actual ~/.openclaw/openclaw.json.
//! They are skipped if the file does not exist.

use rustcalw_config::io::{read_config_file, resolve_config_path, resolve_gateway_port};

fn skip_if_no_live_config() -> Option<std::path::PathBuf> {
    let path = resolve_config_path();
    if path.exists() {
        Some(path)
    } else {
        eprintln!("SKIP: live config not found at {}", path.display());
        None
    }
}

#[test]
fn live_config_loads_successfully() {
    let Some(path) = skip_if_no_live_config() else {
        return;
    };
    let snapshot = read_config_file(&path).expect("live config should load without error");
    assert!(snapshot.exists, "file should exist");
    assert!(snapshot.valid, "config should be valid");
    assert!(snapshot.raw.is_some(), "raw content should be present");
    assert!(snapshot.hash.is_some(), "hash should be computed");
}

#[test]
fn live_config_has_gateway() {
    let Some(path) = skip_if_no_live_config() else {
        return;
    };
    let snapshot = read_config_file(&path).unwrap();
    let gw = snapshot
        .config
        .gateway
        .as_ref()
        .expect("live config should have gateway section");
    assert!(gw.port.is_some(), "gateway should have a port");
    let port = resolve_gateway_port(Some(&snapshot.config));
    assert!(port > 0, "resolved port should be positive");
}

#[test]
fn live_config_has_dashscope_provider() {
    let Some(path) = skip_if_no_live_config() else {
        return;
    };
    let snapshot = read_config_file(&path).unwrap();
    let models = snapshot
        .config
        .models
        .as_ref()
        .expect("live config should have models section");
    let providers = models
        .providers
        .as_ref()
        .expect("models should have providers");
    assert!(
        providers.contains_key("dashscope"),
        "dashscope provider should be configured"
    );
    let ds = &providers["dashscope"];
    assert!(
        ds.base_url.contains("dashscope"),
        "dashscope baseUrl should contain 'dashscope'"
    );
    assert!(
        !ds.models.is_empty(),
        "dashscope should have at least one model"
    );
}

#[test]
fn live_config_round_trips_through_json() {
    let Some(path) = skip_if_no_live_config() else {
        return;
    };
    let snapshot = read_config_file(&path).unwrap();

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&snapshot.config).unwrap();

    // Parse back
    let config2: rustcalw_config::types::openclaw::OpenClawConfig =
        serde_json::from_str(&json).expect("round-tripped JSON should parse");

    // Key fields should survive
    assert_eq!(
        config2.gateway.as_ref().unwrap().port,
        snapshot.config.gateway.as_ref().unwrap().port
    );
    assert!(config2.models.as_ref().unwrap().providers.as_ref().unwrap().contains_key("dashscope"));
}

#[test]
fn live_config_round_trips_through_yaml() {
    let Some(path) = skip_if_no_live_config() else {
        return;
    };
    let snapshot = read_config_file(&path).unwrap();

    // Serialize to YAML
    let yaml = serde_yaml::to_string(&snapshot.config).unwrap();

    // Parse back
    let config2: rustcalw_config::types::openclaw::OpenClawConfig =
        serde_yaml::from_str(&yaml).expect("round-tripped YAML should parse");

    assert_eq!(
        config2.gateway.as_ref().unwrap().port,
        snapshot.config.gateway.as_ref().unwrap().port
    );
}
