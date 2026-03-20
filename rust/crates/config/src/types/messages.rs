//! Message types — mirrors src/config/types.messages.ts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupChatConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_patterns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_limit: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_channel: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debounce_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debounce_ms_by_channel: Option<HashMap<String, u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboundDebounceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debounce_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_channel: Option<HashMap<String, u64>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusReactionsEmojiConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stall_soft: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stall_hard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compacting: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusReactionsTimingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debounce_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stall_soft_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stall_hard_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_hold_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_hold_ms: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusReactionsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emojis: Option<StatusReactionsEmojiConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<StatusReactionsTimingConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat: Option<GroupChatConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<QueueConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound: Option<InboundDebounceConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ack_reaction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ack_reaction_scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_ack_after_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reactions: Option<StatusReactionsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_tool_errors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(flatten)]
    pub peers: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_skills: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bash: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bash_foreground_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_access_groups: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_allow_from: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_display_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_from: Option<HashMap<String, Vec<serde_json::Value>>>,
}
