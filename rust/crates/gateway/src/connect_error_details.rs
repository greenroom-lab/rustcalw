use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Connect error detail codes
// ---------------------------------------------------------------------------

pub const AUTH_REQUIRED: &str = "AUTH_REQUIRED";
pub const AUTH_UNAUTHORIZED: &str = "AUTH_UNAUTHORIZED";
pub const AUTH_TOKEN_MISSING: &str = "AUTH_TOKEN_MISSING";
pub const AUTH_TOKEN_MISMATCH: &str = "AUTH_TOKEN_MISMATCH";
pub const AUTH_TOKEN_NOT_CONFIGURED: &str = "AUTH_TOKEN_NOT_CONFIGURED";
pub const AUTH_PASSWORD_MISSING: &str = "AUTH_PASSWORD_MISSING";
pub const AUTH_PASSWORD_MISMATCH: &str = "AUTH_PASSWORD_MISMATCH";
pub const AUTH_PASSWORD_NOT_CONFIGURED: &str = "AUTH_PASSWORD_NOT_CONFIGURED";
pub const AUTH_BOOTSTRAP_TOKEN_INVALID: &str = "AUTH_BOOTSTRAP_TOKEN_INVALID";
pub const AUTH_DEVICE_TOKEN_MISMATCH: &str = "AUTH_DEVICE_TOKEN_MISMATCH";
pub const AUTH_RATE_LIMITED: &str = "AUTH_RATE_LIMITED";
pub const AUTH_TAILSCALE_IDENTITY_MISSING: &str = "AUTH_TAILSCALE_IDENTITY_MISSING";
pub const AUTH_TAILSCALE_PROXY_MISSING: &str = "AUTH_TAILSCALE_PROXY_MISSING";
pub const AUTH_TAILSCALE_WHOIS_FAILED: &str = "AUTH_TAILSCALE_WHOIS_FAILED";
pub const AUTH_TAILSCALE_IDENTITY_MISMATCH: &str = "AUTH_TAILSCALE_IDENTITY_MISMATCH";
pub const CONTROL_UI_ORIGIN_NOT_ALLOWED: &str = "CONTROL_UI_ORIGIN_NOT_ALLOWED";
pub const CONTROL_UI_DEVICE_IDENTITY_REQUIRED: &str = "CONTROL_UI_DEVICE_IDENTITY_REQUIRED";
pub const DEVICE_IDENTITY_REQUIRED: &str = "DEVICE_IDENTITY_REQUIRED";
pub const DEVICE_AUTH_INVALID: &str = "DEVICE_AUTH_INVALID";
pub const DEVICE_AUTH_DEVICE_ID_MISMATCH: &str = "DEVICE_AUTH_DEVICE_ID_MISMATCH";
pub const DEVICE_AUTH_SIGNATURE_EXPIRED: &str = "DEVICE_AUTH_SIGNATURE_EXPIRED";
pub const DEVICE_AUTH_NONCE_REQUIRED: &str = "DEVICE_AUTH_NONCE_REQUIRED";
pub const DEVICE_AUTH_NONCE_MISMATCH: &str = "DEVICE_AUTH_NONCE_MISMATCH";
pub const DEVICE_AUTH_SIGNATURE_INVALID: &str = "DEVICE_AUTH_SIGNATURE_INVALID";
pub const DEVICE_AUTH_PUBLIC_KEY_INVALID: &str = "DEVICE_AUTH_PUBLIC_KEY_INVALID";
pub const PAIRING_REQUIRED: &str = "PAIRING_REQUIRED";

pub type ConnectErrorDetailCode = &'static str;

// ---------------------------------------------------------------------------
// Recovery next-step
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectRecoveryNextStep {
    RetryWithDeviceToken,
    UpdateAuthConfiguration,
    UpdateAuthCredentials,
    WaitThenRetry,
    ReviewAuthConfiguration,
}

impl ConnectRecoveryNextStep {
    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "retry_with_device_token" => Some(Self::RetryWithDeviceToken),
            "update_auth_configuration" => Some(Self::UpdateAuthConfiguration),
            "update_auth_credentials" => Some(Self::UpdateAuthCredentials),
            "wait_then_retry" => Some(Self::WaitThenRetry),
            "review_auth_configuration" => Some(Self::ReviewAuthConfiguration),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Recovery advice
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectErrorRecoveryAdvice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_retry_with_device_token: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_next_step: Option<ConnectRecoveryNextStep>,
}

// ---------------------------------------------------------------------------
// Resolution helpers
// ---------------------------------------------------------------------------

/// Map a gateway auth failure reason string to the corresponding
/// `ConnectErrorDetailCode`.
pub fn resolve_auth_connect_error_detail_code(reason: Option<&str>) -> &'static str {
    match reason {
        Some("token_missing") => AUTH_TOKEN_MISSING,
        Some("token_mismatch") => AUTH_TOKEN_MISMATCH,
        Some("token_missing_config") => AUTH_TOKEN_NOT_CONFIGURED,
        Some("password_missing") => AUTH_PASSWORD_MISSING,
        Some("password_mismatch") => AUTH_PASSWORD_MISMATCH,
        Some("password_missing_config") => AUTH_PASSWORD_NOT_CONFIGURED,
        Some("bootstrap_token_invalid") => AUTH_BOOTSTRAP_TOKEN_INVALID,
        Some("tailscale_user_missing") => AUTH_TAILSCALE_IDENTITY_MISSING,
        Some("tailscale_proxy_missing") => AUTH_TAILSCALE_PROXY_MISSING,
        Some("tailscale_whois_failed") => AUTH_TAILSCALE_WHOIS_FAILED,
        Some("tailscale_user_mismatch") => AUTH_TAILSCALE_IDENTITY_MISMATCH,
        Some("rate_limited") => AUTH_RATE_LIMITED,
        Some("device_token_mismatch") => AUTH_DEVICE_TOKEN_MISMATCH,
        None => AUTH_REQUIRED,
        Some(_) => AUTH_UNAUTHORIZED,
    }
}

/// Map a device auth failure reason string to the corresponding
/// `ConnectErrorDetailCode`.
pub fn resolve_device_auth_connect_error_detail_code(reason: Option<&str>) -> &'static str {
    match reason {
        Some("device-id-mismatch") => DEVICE_AUTH_DEVICE_ID_MISMATCH,
        Some("device-signature-stale") => DEVICE_AUTH_SIGNATURE_EXPIRED,
        Some("device-nonce-missing") => DEVICE_AUTH_NONCE_REQUIRED,
        Some("device-nonce-mismatch") => DEVICE_AUTH_NONCE_MISMATCH,
        Some("device-signature") => DEVICE_AUTH_SIGNATURE_INVALID,
        Some("device-public-key") => DEVICE_AUTH_PUBLIC_KEY_INVALID,
        _ => DEVICE_AUTH_INVALID,
    }
}

/// Extract the `code` field from an opaque details object.
pub fn read_connect_error_detail_code(details: &serde_json::Value) -> Option<&str> {
    let code = details.as_object()?.get("code")?.as_str()?;
    if code.trim().is_empty() {
        None
    } else {
        Some(code)
    }
}

/// Extract recovery advice from an opaque details object.
pub fn read_connect_error_recovery_advice(
    details: &serde_json::Value,
) -> ConnectErrorRecoveryAdvice {
    let obj = match details.as_object() {
        Some(o) => o,
        None => return ConnectErrorRecoveryAdvice::default(),
    };

    let can_retry_with_device_token = obj
        .get("canRetryWithDeviceToken")
        .and_then(|v| v.as_bool());

    let recommended_next_step = obj
        .get("recommendedNextStep")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .and_then(ConnectRecoveryNextStep::from_str_opt);

    ConnectErrorRecoveryAdvice {
        can_retry_with_device_token,
        recommended_next_step,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn auth_error_codes() {
        assert_eq!(
            resolve_auth_connect_error_detail_code(Some("token_missing")),
            AUTH_TOKEN_MISSING,
        );
        assert_eq!(
            resolve_auth_connect_error_detail_code(Some("rate_limited")),
            AUTH_RATE_LIMITED,
        );
        assert_eq!(
            resolve_auth_connect_error_detail_code(None),
            AUTH_REQUIRED,
        );
        assert_eq!(
            resolve_auth_connect_error_detail_code(Some("something_else")),
            AUTH_UNAUTHORIZED,
        );
    }

    #[test]
    fn device_auth_error_codes() {
        assert_eq!(
            resolve_device_auth_connect_error_detail_code(Some("device-id-mismatch")),
            DEVICE_AUTH_DEVICE_ID_MISMATCH,
        );
        assert_eq!(
            resolve_device_auth_connect_error_detail_code(Some("device-signature")),
            DEVICE_AUTH_SIGNATURE_INVALID,
        );
        assert_eq!(
            resolve_device_auth_connect_error_detail_code(Some("unknown")),
            DEVICE_AUTH_INVALID,
        );
        assert_eq!(
            resolve_device_auth_connect_error_detail_code(None),
            DEVICE_AUTH_INVALID,
        );
    }

    #[test]
    fn read_detail_code() {
        assert_eq!(
            read_connect_error_detail_code(&json!({"code": "AUTH_REQUIRED"})),
            Some("AUTH_REQUIRED"),
        );
        assert_eq!(
            read_connect_error_detail_code(&json!({"code": ""})),
            None,
        );
        assert_eq!(read_connect_error_detail_code(&json!(42)), None);
        assert_eq!(read_connect_error_detail_code(&json!(null)), None);
    }

    #[test]
    fn read_recovery_advice() {
        let advice = read_connect_error_recovery_advice(&json!({
            "canRetryWithDeviceToken": true,
            "recommendedNextStep": "wait_then_retry"
        }));
        assert_eq!(advice.can_retry_with_device_token, Some(true));
        assert_eq!(
            advice.recommended_next_step,
            Some(ConnectRecoveryNextStep::WaitThenRetry),
        );

        let empty = read_connect_error_recovery_advice(&json!({}));
        assert_eq!(empty, ConnectErrorRecoveryAdvice::default());
    }
}
