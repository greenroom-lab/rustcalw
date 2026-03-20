//! Agent types — mirrors src/config/types.agents.ts

use super::base::{HumanDelayConfig, IdentityConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Agent model config — can be a string alias or an object with primary + fallbacks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AgentModelConfig {
    Alias(String),
    Full {
        primary: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        fallbacks: Option<Vec<String>>,
    },
}

/// Agent sandbox configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSandboxConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Memory search configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemorySearchConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Group chat configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupChatConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_patterns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_limit: Option<u32>,
}

/// Agent tools configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentToolsConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// ACP runtime configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRuntimeAcpConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
}

/// Agent runtime configuration (embedded or ACP).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AgentRuntimeConfig {
    Embedded,
    Acp {
        #[serde(skip_serializing_if = "Option::is_none")]
        acp: Option<AgentRuntimeAcpConfig>,
    },
}

/// Agent binding match criteria.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentBindingMatch {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}

/// Agent binding (route or ACP).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentBinding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub r#match: AgentBindingMatch,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acp: Option<serde_json::Value>,
}

/// Subagents configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubagentsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_agents: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<AgentModelConfig>,
}

/// Agent defaults configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentDefaultsConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Single agent configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentConfig {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<AgentModelConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_search: Option<MemorySearchConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_delay: Option<HumanDelayConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat: Option<GroupChatConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subagents: Option<SubagentsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<AgentSandboxConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<AgentToolsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<AgentRuntimeConfig>,
}

/// Top-level agents configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<AgentDefaultsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AgentConfig>>,
}
