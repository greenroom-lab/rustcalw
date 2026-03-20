//! Root OpenClaw config type — mirrors src/config/types.openclaw.ts

use super::agents::{AgentBinding, AgentsConfig};
use super::base::{DiagnosticsConfig, LoggingConfig, SessionConfig, WebConfig};
use super::channels::ChannelsConfig;
use super::gateway::{CanvasHostConfig, DiscoveryConfig, GatewayConfig, TalkConfig};
use super::hooks::HooksConfig;
use super::messages::{AudioConfig, BroadcastConfig, CommandsConfig, MessagesConfig};
use super::models::ModelsConfig;
use super::secrets::SecretsConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Config file metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_touched_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_touched_at: Option<String>,
}

/// Shell env import config.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellEnvConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

/// Env configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_env: Option<ShellEnvConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vars: Option<HashMap<String, String>>,
    /// Additional string env vars (sugar syntax).
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Wizard state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WizardConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_mode: Option<String>,
}

/// Auth configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// ACP configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcpConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Approvals configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalsConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Update configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAutoConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stable_delay_hours: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stable_jitter_hours: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta_check_interval_hours: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_on_start: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto: Option<UpdateAutoConfig>,
}

/// Browser configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// UI configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiAssistantConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seam_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant: Option<UiAssistantConfig>,
}

/// CLI configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CliConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Skills configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillsConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Plugins configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginsConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Tools configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Node host configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeHostConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Cron configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Memory configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// MCP configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Media configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_filenames: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_hours: Option<u32>,
}

/// Root OpenClaw configuration — mirrors `OpenClawConfig` from types.openclaw.ts.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenClawConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ConfigMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acp: Option<AcpConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<EnvConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wizard: Option<WizardConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli: Option<CliConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<BrowserConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<UiConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<SecretsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<SkillsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<PluginsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<ModelsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_host: Option<NodeHostConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<AgentsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<AgentBinding>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<BroadcastConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<MediaConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<MessagesConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<CommandsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approvals: Option<ApprovalsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<SessionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<WebConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<ChannelsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<CronConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<HooksConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery: Option<DiscoveryConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_host: Option<CanvasHostConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talk: Option<TalkConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<GatewayConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemoryConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp: Option<McpConfig>,
}

/// A single config validation issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigValidationIssue {
    pub path: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values_hidden_count: Option<u32>,
}

/// Legacy config migration issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyConfigIssue {
    pub path: String,
    pub message: String,
}

/// In-memory snapshot of a config file.
#[derive(Debug, Clone)]
pub struct ConfigFileSnapshot {
    pub path: String,
    pub exists: bool,
    pub raw: Option<String>,
    pub parsed: Option<serde_json::Value>,
    /// Config after $include resolution and ${ENV} substitution,
    /// but BEFORE runtime defaults are applied.
    pub resolved: OpenClawConfig,
    pub valid: bool,
    pub config: OpenClawConfig,
    pub hash: Option<String>,
    pub issues: Vec<ConfigValidationIssue>,
    pub warnings: Vec<ConfigValidationIssue>,
    pub legacy_issues: Vec<LegacyConfigIssue>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_serializes_to_empty_object() {
        let config = OpenClawConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn config_round_trips_through_json() {
        let json = r#"{"gateway":{"port":18789},"logging":{"level":"info"}}"#;
        let config: OpenClawConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.gateway.as_ref().unwrap().port, Some(18789));
        assert_eq!(
            config.logging.as_ref().unwrap().level.as_deref(),
            Some("info")
        );
        let roundtrip = serde_json::to_string(&config).unwrap();
        let config2: OpenClawConfig = serde_json::from_str(&roundtrip).unwrap();
        assert_eq!(config2.gateway.as_ref().unwrap().port, Some(18789));
    }

    #[test]
    fn config_round_trips_through_yaml() {
        let yaml = "gateway:\n  port: 18789\nlogging:\n  level: info\n";
        let config: OpenClawConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.gateway.as_ref().unwrap().port, Some(18789));
    }
}
