//! E2E tests for the `wnc` CLI binary.
//!
//! These tests build and execute the actual binary, verifying stdout, stderr,
//! and exit codes against expected behavior — the real "does it work" check.

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::Path;
use tempfile::TempDir;

fn fixtures_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").leak()
}

fn wnc() -> Command {
    Command::cargo_bin("rustcalw-cli").expect("binary should be built")
}

// ── config path ──────────────────────────────────────────────

#[test]
fn config_path_prints_a_path() {
    wnc()
        .args(["config", "path"])
        .assert()
        .success()
        .stdout(predicate::str::contains("openclaw.json"));
}

#[test]
fn config_path_respects_env_override() {
    wnc()
        .args(["config", "path"])
        .env("OPENCLAW_CONFIG_PATH", "/custom/path/openclaw.json")
        .assert()
        .success()
        .stdout(predicate::str::contains("/custom/path/openclaw.json"));
}

// ── config check ─────────────────────────────────────────────

#[test]
fn config_check_valid_file() {
    let fixture = fixtures_dir().join("valid.json");

    wnc()
        .args(["config", "check"])
        .env("OPENCLAW_CONFIG_PATH", fixture.as_os_str())
        .assert()
        .success()
        .stdout(predicate::str::contains("Config loaded successfully."))
        .stdout(predicate::str::contains("Gateway: port=18789, mode=local"))
        .stdout(predicate::str::contains("Agents: 1 configured"))
        .stdout(predicate::str::contains("Model providers: 1"))
        .stdout(predicate::str::contains("discord"));
}

#[test]
fn config_check_missing_file() {
    let tmp = TempDir::new().unwrap();
    let missing = tmp.path().join("nonexistent.json");

    wnc()
        .args(["config", "check"])
        .env("OPENCLAW_CONFIG_PATH", missing.as_os_str())
        .assert()
        .success()
        .stdout(predicate::str::contains("Config file not found"));
}

#[test]
fn config_check_invalid_json() {
    let fixture = fixtures_dir().join("invalid.json");

    wnc()
        .args(["config", "check"])
        .env("OPENCLAW_CONFIG_PATH", fixture.as_os_str())
        .assert()
        .failure()
        .stderr(predicate::str::contains("parse")
            .or(predicate::str::contains("JSON"))
            .or(predicate::str::contains("error")));
}

#[test]
fn config_check_empty_config() {
    let tmp = TempDir::new().unwrap();
    let empty = tmp.path().join("openclaw.json");
    std::fs::write(&empty, "{}").unwrap();

    wnc()
        .args(["config", "check"])
        .env("OPENCLAW_CONFIG_PATH", empty.as_os_str())
        .assert()
        .success()
        .stdout(predicate::str::contains("Config loaded successfully."));
}

// ── config deploy-check ──────────────────────────────────────

#[test]
fn deploy_check_valid_round_trip() {
    let fixture = fixtures_dir().join("valid.json");

    wnc()
        .args(["config", "deploy-check"])
        .env("OPENCLAW_CONFIG_PATH", fixture.as_os_str())
        .assert()
        .success()
        .stdout(predicate::str::contains("Deploy check passed."))
        .stdout(predicate::str::contains("JSON round-trip: OK"))
        .stdout(predicate::str::contains("YAML round-trip: OK"));
}

#[test]
fn deploy_check_missing_file_exits_nonzero() {
    let tmp = TempDir::new().unwrap();
    let missing = tmp.path().join("nonexistent.json");

    wnc()
        .args(["config", "deploy-check"])
        .env("OPENCLAW_CONFIG_PATH", missing.as_os_str())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Config file not found"));
}

// ── gateway ──────────────────────────────────────────────────
// Gateway E2E tests are in e2e_gateway.rs

// ── usage / help ─────────────────────────────────────────────

#[test]
fn no_args_shows_help() {
    wnc()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn help_flag_shows_usage() {
    wnc()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("rustcalw"))
        .stdout(predicate::str::contains("gateway"))
        .stdout(predicate::str::contains("config"));
}

#[test]
fn invalid_subcommand_exits_nonzero() {
    wnc()
        .arg("nonexistent")
        .assert()
        .failure();
}
