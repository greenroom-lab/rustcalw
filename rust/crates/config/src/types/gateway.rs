//! Gateway types — mirrors src/config/types.gateway.ts

use super::secrets::SecretInput;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GatewayBindMode {
    Auto,
    Lan,
    Loopback,
    Custom,
    Tailnet,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GatewayAuthMode {
    None,
    Token,
    Password,
    #[serde(rename = "trusted-proxy")]
    TrustedProxy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GatewayTailscaleMode {
    Off,
    Serve,
    Funnel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GatewayReloadMode {
    Off,
    Restart,
    Hot,
    Hybrid,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayTlsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_generate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayControlUiConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dangerously_allow_host_header_origin_fallback: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_insecure_auth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dangerously_disable_device_auth: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayTrustedProxyConfig {
    pub user_header: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_headers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_users: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayAuthRateLimitConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lockout_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_loopback: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayAuthConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<GatewayAuthMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<SecretInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<SecretInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_tailscale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<GatewayAuthRateLimitConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_proxy: Option<GatewayTrustedProxyConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayTailscaleConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<GatewayTailscaleMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_on_exit: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayRemoteConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<SecretInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<SecretInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_identity: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayReloadConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<GatewayReloadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debounce_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferral_timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayHttpChatCompletionsImagesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_allowlist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mimes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redirects: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayHttpChatCompletionsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_body_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_image_parts: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_total_image_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<GatewayHttpChatCompletionsImagesConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayHttpEndpointsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_completions: Option<GatewayHttpChatCompletionsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayHttpSecurityHeadersConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_transport_security: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayHttpConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<GatewayHttpEndpointsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_headers: Option<GatewayHttpSecurityHeadersConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayPushApnsRelayConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayPushConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayNodesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_commands: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny_commands: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayToolsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasHostConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_reload: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wide_area: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdns: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TalkProviderConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_aliases: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretInput>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TalkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providers: Option<HashMap<String, TalkProviderConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupt_on_speech: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silence_timeout_ms: Option<u64>,
    // Legacy ElevenLabs compat fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_aliases: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecretInput>,
}

/// Top-level gateway configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind: Option<GatewayBindMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_bind_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_ui: Option<GatewayControlUiConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<GatewayAuthConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tailscale: Option<GatewayTailscaleConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<GatewayRemoteConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reload: Option<GatewayReloadConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<GatewayTlsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<GatewayHttpConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push: Option<GatewayPushConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<GatewayNodesConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_proxies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_real_ip_fallback: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<GatewayToolsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_health_check_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_stale_event_threshold_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_max_restarts_per_hour: Option<u32>,
}
