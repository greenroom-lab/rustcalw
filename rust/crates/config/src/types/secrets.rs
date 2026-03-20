//! Secret types — mirrors src/config/types.secrets.ts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SecretRefSource {
    Env,
    File,
    Exec,
}

/// Stable identifier for a secret in a configured source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretRef {
    pub source: SecretRefSource,
    pub provider: String,
    pub id: String,
}

/// A secret can be either a plain string or a structured reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretInput {
    Plain(String),
    Ref(SecretRef),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvSecretProviderConfig {
    /// Must be "env".
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowlist: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSecretProviderConfig {
    pub source: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bytes: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecSecretProviderConfig {
    pub source: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_output_timeout_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_env: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_dirs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_insecure_path: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_symlink_command: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretResolutionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_provider_concurrency: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_refs_per_provider: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_bytes: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providers: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<SecretDefaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<SecretResolutionConfig>,
}

pub const DEFAULT_SECRET_PROVIDER_ALIAS: &str = "default";
