//! Base config types — mirrors src/config/types.base.ts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Enums ──

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReplyMode {
    Text,
    Command,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TypingMode {
    Never,
    Instant,
    Thinking,
    Message,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SessionScope {
    PerSender,
    Global,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DmScope {
    Main,
    PerPeer,
    PerChannelPeer,
    PerAccountChannelPeer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReplyToMode {
    Off,
    First,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GroupPolicy {
    Open,
    Disabled,
    Allowlist,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DmPolicy {
    Pairing,
    Allowlist,
    Open,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MarkdownTableMode {
    Off,
    Bullets,
    Code,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SessionResetMode {
    Daily,
    Idle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SessionMaintenanceMode {
    Enforce,
    Warn,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SessionSendPolicyAction {
    Allow,
    Deny,
}

// ── Structs ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboundRetryConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_delay_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_delay_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jitter: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockStreamingCoalesceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_chars: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_chars: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_ms: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockStreamingChunkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_chars: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_chars: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub break_preference: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<MarkdownTableMode>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanDelayConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ms: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSendPolicyMatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSendPolicyRule {
    pub action: SessionSendPolicyAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<SessionSendPolicyMatch>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSendPolicyConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<SessionSendPolicyAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<SessionSendPolicyRule>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionResetConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<SessionResetMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_hour: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_minutes: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionResetByTypeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct: Option<SessionResetConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm: Option<SessionResetConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<SessionResetConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<SessionResetConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionThreadBindingsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_hours: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_hours: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionMaintenanceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<SessionMaintenanceMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune_after: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_entries: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_bytes: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_archive_retention: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_disk_bytes: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_water_bytes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentToAgentConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ping_pong_turns: Option<u8>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<SessionScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_scope: Option<DmScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_links: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_triggers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<SessionResetConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_by_type: Option<SessionResetByTypeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_by_channel: Option<HashMap<String, SessionResetConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing_interval_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing_mode: Option<TypingMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_fork_max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_policy: Option<SessionSendPolicyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_to_agent: Option<AgentToAgentConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_bindings: Option<SessionThreadBindingsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<SessionMaintenanceConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_sensitive: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_patterns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsOtelConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traces: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush_interval_ms: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsCacheTraceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_prompt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_system: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stuck_session_warn_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otel: Option<DiagnosticsOtelConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_trace: Option<DiagnosticsCacheTraceConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebReconnectConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jitter: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect: Option<WebReconnectConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}
