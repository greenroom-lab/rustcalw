//! OAuth credential types — mirrors OAuthCredentials from @mariozechner/pi-ai

use serde::{Deserialize, Serialize};

/// OAuth credentials used by provider auth flows.
///
/// Mirrors the `OAuthCredentials` type from `@mariozechner/pi-ai`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCredentials {
    /// Access token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    /// Refresh token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<String>,
    /// Expiry timestamp (ms since epoch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<u64>,
}
