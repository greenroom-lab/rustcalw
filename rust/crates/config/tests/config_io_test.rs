//! Integration tests for config I/O with fixture files.

use rustcalw_config::io::read_config_file;
use std::path::Path;

#[test]
fn load_minimal_fixture() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/minimal.json");
    let snapshot = read_config_file(&path).expect("should load minimal fixture");

    assert!(snapshot.exists);
    assert!(snapshot.valid);

    let cfg = &snapshot.config;
    let gw = cfg.gateway.as_ref().expect("gateway should be present");
    assert_eq!(gw.port, Some(18789));
    assert_eq!(gw.mode.as_deref(), Some("local"));

    let logging = cfg.logging.as_ref().expect("logging should be present");
    assert_eq!(logging.level.as_deref(), Some("info"));
    assert_eq!(logging.console_style.as_deref(), Some("pretty"));

    let session = cfg.session.as_ref().expect("session should be present");
    assert_eq!(
        session.typing_mode,
        Some(rustcalw_config::types::base::TypingMode::Thinking)
    );
}

#[test]
fn load_full_fixture() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/full.json");
    let snapshot = read_config_file(&path).expect("should load full fixture");

    assert!(snapshot.exists);
    assert!(snapshot.valid);

    let cfg = &snapshot.config;

    // Gateway
    let gw = cfg.gateway.as_ref().unwrap();
    assert_eq!(gw.port, Some(18789));
    let reload = gw.reload.as_ref().unwrap();
    assert_eq!(
        reload.mode,
        Some(rustcalw_config::types::gateway::GatewayReloadMode::Hybrid)
    );
    assert_eq!(reload.debounce_ms, Some(300));

    // Agents
    let agents = cfg.agents.as_ref().unwrap();
    let list = agents.list.as_ref().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, "default");
    assert_eq!(list[0].default, Some(true));
    assert_eq!(list[0].name.as_deref(), Some("TestBot"));
    let identity = list[0].identity.as_ref().unwrap();
    assert_eq!(identity.name.as_deref(), Some("TestBot"));
    assert_eq!(identity.emoji.as_deref(), Some("🤖"));

    // Models
    let models = cfg.models.as_ref().unwrap();
    assert_eq!(models.mode.as_deref(), Some("merge"));
    let providers = models.providers.as_ref().unwrap();
    assert!(providers.contains_key("test-provider"));
    let provider = &providers["test-provider"];
    assert_eq!(provider.base_url, "https://api.example.com/v1");
    assert_eq!(provider.models.len(), 1);
    assert_eq!(provider.models[0].id, "test-model");
    assert!(provider.models[0].cost.is_some());
    assert_eq!(provider.models[0].context_window, 131072);

    // Channels
    let channels = cfg.channels.as_ref().unwrap();
    assert!(channels.discord.is_some());

    // Session
    let session = cfg.session.as_ref().unwrap();
    assert_eq!(session.idle_minutes, Some(30));
    let reset = session.reset.as_ref().unwrap();
    assert_eq!(
        reset.mode,
        Some(rustcalw_config::types::base::SessionResetMode::Daily)
    );
    assert_eq!(reset.at_hour, Some(4));

    // Messages
    let messages = cfg.messages.as_ref().unwrap();
    assert_eq!(messages.ack_reaction.as_deref(), Some("👀"));
}

#[test]
fn load_nonexistent_returns_empty() {
    let path = Path::new("/tmp/rustcalw-nonexistent-test-config.json");
    let snapshot = read_config_file(path).expect("should handle missing file gracefully");
    assert!(!snapshot.exists);
    assert!(snapshot.valid);
}

#[test]
fn config_round_trips_through_json() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/full.json");
    let snapshot = read_config_file(&path).unwrap();

    // Serialize back to JSON
    let json_str = serde_json::to_string_pretty(&snapshot.config).unwrap();

    // Parse again
    let config2: rustcalw_config::types::openclaw::OpenClawConfig =
        serde_json::from_str(&json_str).unwrap();

    // Verify key fields survived round-trip
    assert_eq!(config2.gateway.as_ref().unwrap().port, Some(18789));
    assert_eq!(
        config2.agents.as_ref().unwrap().list.as_ref().unwrap()[0].id,
        "default"
    );
}
