//! Qwen portal OAuth token refresh — mirrors src/providers/qwen-portal-oauth.ts

use crate::oauth_types::OAuthCredentials;
use anyhow::{bail, Context};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

const QWEN_OAUTH_BASE_URL: &str = "https://chat.qwen.ai";
const QWEN_OAUTH_CLIENT_ID: &str = "f0304373b74a44d2b584a3fb70ca9e56";

fn qwen_oauth_token_endpoint() -> String {
    format!("{QWEN_OAUTH_BASE_URL}/api/v1/oauth2/token")
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_in: Option<f64>,
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Refresh Qwen portal OAuth credentials using the stored refresh token.
///
/// Returns updated credentials with a new access token and (optionally) a new
/// refresh token, per RFC 6749 section 6.
pub async fn refresh_qwen_portal_credentials(
    credentials: &OAuthCredentials,
    client: &reqwest::Client,
) -> anyhow::Result<OAuthCredentials> {
    let refresh_token = credentials
        .refresh
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .context("Qwen OAuth refresh token missing; re-authenticate.")?;

    let response = client
        .post(qwen_oauth_token_endpoint())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", QWEN_OAUTH_CLIENT_ID),
        ])
        .send()
        .await
        .context("Qwen OAuth refresh request failed")?;

    if response.status() == reqwest::StatusCode::BAD_REQUEST {
        bail!(
            "Qwen OAuth refresh token expired or invalid. Re-authenticate with \
             `openclaw models auth login --provider qwen-portal`."
        );
    }

    if !response.status().is_success() {
        let text = response.text().await.unwrap_or_default();
        bail!("Qwen OAuth refresh failed: {text}");
    }

    let payload: TokenResponse = response
        .json()
        .await
        .context("Qwen OAuth refresh response parse failed")?;

    let access_token = payload
        .access_token
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .context("Qwen OAuth refresh response missing access token.")?
        .to_string();

    let expires_in = match payload.expires_in {
        Some(v) if v.is_finite() && v > 0.0 => v,
        _ => bail!("Qwen OAuth refresh response missing or invalid expires_in."),
    };

    let new_refresh = payload
        .refresh_token
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    Ok(OAuthCredentials {
        access: Some(access_token),
        // RFC 6749 section 6: new refresh token is optional; if present, replace old.
        refresh: Some(
            new_refresh.unwrap_or_else(|| refresh_token.to_string()),
        ),
        expires: Some(now_ms() + (expires_in as u64) * 1000),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_url() {
        assert_eq!(
            qwen_oauth_token_endpoint(),
            "https://chat.qwen.ai/api/v1/oauth2/token"
        );
    }

    #[tokio::test]
    async fn missing_refresh_token_errors() {
        let client = reqwest::Client::new();
        let creds = OAuthCredentials {
            access: Some("old".into()),
            refresh: None,
            expires: None,
        };
        let err = refresh_qwen_portal_credentials(&creds, &client)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("refresh token missing"));
    }

    #[tokio::test]
    async fn empty_refresh_token_errors() {
        let client = reqwest::Client::new();
        let creds = OAuthCredentials {
            access: Some("old".into()),
            refresh: Some("  ".into()),
            expires: None,
        };
        let err = refresh_qwen_portal_credentials(&creds, &client)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("refresh token missing"));
    }
}
