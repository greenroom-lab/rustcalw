//! Channel types — mirrors src/config/types.channels.ts

use super::base::GroupPolicy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelHeartbeatVisibilityConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_ok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alerts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_indicator: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelHealthMonitorConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelDefaultsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_policy: Option<GroupPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat: Option<ChannelHeartbeatVisibilityConfig>,
}

/// Top-level channels configuration.
/// Known channel keys are typed; extension channels use dynamic keys.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<ChannelDefaultsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_by_channel: Option<HashMap<String, HashMap<String, String>>>,
    // Known channel types — each is an opaque JSON value for now.
    // Full typed structs per channel can be added as needed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irc: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub googlechat: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imessage: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msteams: Option<serde_json::Value>,
    /// Extension channels (dynamic keys).
    #[serde(flatten)]
    pub extensions: HashMap<String, serde_json::Value>,
}
