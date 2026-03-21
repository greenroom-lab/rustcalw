//! GitHub Copilot OAuth device flow — mirrors src/providers/github-copilot-auth.ts
//!
//! Implements the OAuth 2.0 device authorization grant (RFC 8628) against
//! GitHub's endpoints.  The high-level `githubCopilotLoginCommand` is NOT
//! ported here because it depends on CLI/runtime/config modules that are not
//! yet available.  Only the protocol primitives are exposed.

use anyhow::{bail, Context};
use serde::Deserialize;

const CLIENT_ID: &str = "Iv1.b507a08c87ecfe98";
const DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const ACCESS_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

// ── Types ────────────────────────────────────────────────────────────

/// Response from the GitHub device code endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

/// Successful token response.
#[derive(Debug, Clone, Deserialize)]
struct TokenSuccess {
    access_token: String,
    #[allow(dead_code)]
    token_type: Option<String>,
    #[allow(dead_code)]
    scope: Option<String>,
}

/// Error token response.
#[derive(Debug, Clone, Deserialize)]
struct TokenError {
    error: String,
    #[allow(dead_code)]
    error_description: Option<String>,
}

/// Outcome of a single poll iteration.
#[derive(Debug)]
pub enum PollOutcome {
    /// Access token obtained.
    Token(String),
    /// Authorization pending — caller should wait `interval` and retry.
    Pending,
    /// GitHub asked us to slow down — add extra delay.
    SlowDown,
}

// ── Protocol helpers ─────────────────────────────────────────────────

/// Request a device code from GitHub.
///
/// The caller must display `user_code` and `verification_uri` to the user.
pub async fn request_device_code(
    client: &reqwest::Client,
    scope: &str,
) -> anyhow::Result<DeviceCodeResponse> {
    let res = client
        .post(DEVICE_CODE_URL)
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[("client_id", CLIENT_ID), ("scope", scope)])
        .send()
        .await
        .context("GitHub device code request failed")?;

    if !res.status().is_success() {
        bail!("GitHub device code failed: HTTP {}", res.status());
    }

    let json: DeviceCodeResponse = res
        .json()
        .await
        .context("GitHub device code response missing fields")?;

    if json.device_code.is_empty() || json.user_code.is_empty() || json.verification_uri.is_empty()
    {
        bail!("GitHub device code response missing fields");
    }

    Ok(json)
}

/// Perform a single poll against the access-token endpoint.
///
/// Returns `PollOutcome::Token` on success, `PollOutcome::Pending` when the
/// user hasn't authorized yet, and `PollOutcome::SlowDown` when GitHub asks
/// us to back off.  Any terminal error (expired, denied, unknown) is returned
/// as `Err`.
pub async fn poll_access_token(
    client: &reqwest::Client,
    device_code: &str,
) -> anyhow::Result<PollOutcome> {
    let res = client
        .post(ACCESS_TOKEN_URL)
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("client_id", CLIENT_ID),
            ("device_code", device_code),
            (
                "grant_type",
                "urn:ietf:params:oauth:grant-type:device_code",
            ),
        ])
        .send()
        .await
        .context("GitHub device token request failed")?;

    if !res.status().is_success() {
        bail!("GitHub device token failed: HTTP {}", res.status());
    }

    let body = res.text().await.context("Failed to read token response")?;

    // Try success first
    if let Ok(ok) = serde_json::from_str::<TokenSuccess>(&body) {
        if !ok.access_token.is_empty() {
            return Ok(PollOutcome::Token(ok.access_token));
        }
    }

    // Otherwise parse error
    let err: TokenError = serde_json::from_str(&body)
        .context("Unexpected response from GitHub")?;

    match err.error.as_str() {
        "authorization_pending" => Ok(PollOutcome::Pending),
        "slow_down" => Ok(PollOutcome::SlowDown),
        "expired_token" => bail!("GitHub device code expired; run login again"),
        "access_denied" => bail!("GitHub login cancelled"),
        other => bail!("GitHub device flow error: {other}"),
    }
}

/// Run the full device-flow polling loop until a token is obtained or the
/// device code expires.
///
/// `interval_ms` is the base polling interval in milliseconds.
/// `expires_at_ms` is the absolute timestamp (ms since epoch) at which the
/// device code expires.
pub async fn poll_for_access_token(
    client: &reqwest::Client,
    device_code: &str,
    mut interval_ms: u64,
    expires_at_ms: u64,
) -> anyhow::Result<String> {
    loop {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        if now >= expires_at_ms {
            bail!("GitHub device code expired; run login again");
        }

        match poll_access_token(client, device_code).await? {
            PollOutcome::Token(token) => return Ok(token),
            PollOutcome::Pending => {
                tokio::time::sleep(std::time::Duration::from_millis(interval_ms)).await;
            }
            PollOutcome::SlowDown => {
                interval_ms += 2000;
                tokio::time::sleep(std::time::Duration::from_millis(interval_ms)).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_match_upstream() {
        assert_eq!(CLIENT_ID, "Iv1.b507a08c87ecfe98");
        assert_eq!(
            DEVICE_CODE_URL,
            "https://github.com/login/device/code"
        );
        assert_eq!(
            ACCESS_TOKEN_URL,
            "https://github.com/login/oauth/access_token"
        );
    }

    #[test]
    fn parse_device_code_response() {
        let json = r#"{
            "device_code": "abc123",
            "user_code": "WDJB-MJHT",
            "verification_uri": "https://github.com/login/device",
            "expires_in": 900,
            "interval": 5
        }"#;
        let resp: DeviceCodeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.device_code, "abc123");
        assert_eq!(resp.user_code, "WDJB-MJHT");
        assert_eq!(resp.verification_uri, "https://github.com/login/device");
        assert_eq!(resp.expires_in, 900);
        assert_eq!(resp.interval, 5);
    }

    #[test]
    fn parse_token_success() {
        let json = r#"{"access_token": "gho_abc", "token_type": "bearer", "scope": "read:user"}"#;
        let ok: TokenSuccess = serde_json::from_str(json).unwrap();
        assert_eq!(ok.access_token, "gho_abc");
    }

    #[test]
    fn parse_token_error() {
        let json = r#"{"error": "authorization_pending"}"#;
        let err: TokenError = serde_json::from_str(json).unwrap();
        assert_eq!(err.error, "authorization_pending");
    }
}
