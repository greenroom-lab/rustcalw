use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::schema_primitives::SecretInput;

// ---------------------------------------------------------------------------
// talk.mode / talk.config / talk.speak
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TalkModeParams {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TalkConfigParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_secrets: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TalkSpeakParams {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_boost: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalize: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

// ---------------------------------------------------------------------------
// TalkConfig result types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    /// Additional provider-specific fields.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedTalkConfig {
    pub provider: String,
    pub config: TalkProviderConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TalkConfigInner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providers: Option<HashMap<String, TalkProviderConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedTalkConfig>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupt_on_speech: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silence_timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TalkConfigSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TalkConfigUi {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seam_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TalkConfigResultConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talk: Option<TalkConfigInner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<TalkConfigSession>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<TalkConfigUi>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TalkConfigResult {
    pub config: TalkConfigResultConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TalkSpeakResult {
    pub audio_base64: String,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_compatible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
}

// ---------------------------------------------------------------------------
// channels.status
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsStatusParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChannelAccountSnapshot {
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_attempts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_connected_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_start_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_stop_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_inbound_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_outbound_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_runs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_activity_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_probe_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_from: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_token_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_token_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unmentioned_groups: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_path: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_path: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<Option<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probe: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Value>,
    /// Additional channel-specific fields.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChannelUiMeta {
    pub id: String,
    pub label: String,
    pub detail_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsStatusResult {
    pub ts: u64,
    pub channel_order: Vec<String>,
    pub channel_labels: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_detail_labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_system_images: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_meta: Option<Vec<ChannelUiMeta>>,
    pub channels: HashMap<String, Value>,
    pub channel_accounts: HashMap<String, Vec<ChannelAccountSnapshot>>,
    pub channel_default_account_id: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// channels.logout / web login
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsLogoutParams {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebLoginStartParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WebLoginWaitParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn talk_speak_params_roundtrip() {
        let params = TalkSpeakParams {
            text: "Hello world".to_string(),
            voice_id: Some("voice-1".to_string()),
            model_id: None,
            output_format: None,
            speed: Some(1.0),
            stability: None,
            similarity: None,
            style: None,
            speaker_boost: None,
            seed: None,
            normalize: None,
            language: None,
        };
        let json_str = serde_json::to_string(&params).unwrap();
        let parsed: TalkSpeakParams = serde_json::from_str(&json_str).unwrap();
        assert_eq!(params, parsed);
    }

    #[test]
    fn channel_account_snapshot_with_extras() {
        let json = json!({
            "accountId": "acc-1",
            "enabled": true,
            "running": true,
            "customField": "value"
        });
        let snap: ChannelAccountSnapshot = serde_json::from_value(json).unwrap();
        assert_eq!(snap.account_id, "acc-1");
        assert_eq!(snap.enabled, Some(true));
        assert_eq!(snap.extra.get("customField").unwrap(), "value");
    }

    #[test]
    fn talk_speak_result_serde() {
        let result = TalkSpeakResult {
            audio_base64: "base64data".to_string(),
            provider: "elevenlabs".to_string(),
            output_format: Some("mp3".to_string()),
            voice_compatible: Some(true),
            mime_type: Some("audio/mpeg".to_string()),
            file_extension: Some("mp3".to_string()),
        };
        let json_val = serde_json::to_value(&result).unwrap();
        assert_eq!(json_val["provider"], "elevenlabs");
    }
}
