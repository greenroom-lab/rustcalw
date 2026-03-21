use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Protocol error codes
// ---------------------------------------------------------------------------

pub const NOT_LINKED: &str = "NOT_LINKED";
pub const NOT_PAIRED: &str = "NOT_PAIRED";
pub const AGENT_TIMEOUT: &str = "AGENT_TIMEOUT";
pub const INVALID_REQUEST: &str = "INVALID_REQUEST";
pub const UNAVAILABLE: &str = "UNAVAILABLE";

pub type ErrorCode = &'static str;

// ---------------------------------------------------------------------------
// ErrorShape
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorShape {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retryable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_ms: Option<u64>,
}

/// Convenience builder matching the TS `errorShape()` function.
pub fn error_shape(code: &str, message: &str) -> ErrorShape {
    ErrorShape {
        code: code.to_string(),
        message: message.to_string(),
        details: None,
        retryable: None,
        retry_after_ms: None,
    }
}

/// Builder with optional fields.
pub fn error_shape_with_opts(
    code: &str,
    message: &str,
    details: Option<Value>,
    retryable: Option<bool>,
    retry_after_ms: Option<u64>,
) -> ErrorShape {
    ErrorShape {
        code: code.to_string(),
        message: message.to_string(),
        details,
        retryable,
        retry_after_ms,
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
    fn error_shape_basic() {
        let e = error_shape(NOT_LINKED, "Gateway is not linked");
        assert_eq!(e.code, "NOT_LINKED");
        assert_eq!(e.message, "Gateway is not linked");
        assert!(e.details.is_none());
        assert!(e.retryable.is_none());
    }

    #[test]
    fn error_shape_with_all_opts() {
        let e = error_shape_with_opts(
            UNAVAILABLE,
            "Service unavailable",
            Some(json!({"reason": "overloaded"})),
            Some(true),
            Some(5000),
        );
        assert_eq!(e.code, "UNAVAILABLE");
        assert_eq!(e.retryable, Some(true));
        assert_eq!(e.retry_after_ms, Some(5000));
    }

    #[test]
    fn serde_roundtrip() {
        let e = error_shape_with_opts(
            INVALID_REQUEST,
            "Bad request",
            Some(json!({"field": "name"})),
            None,
            None,
        );
        let json_str = serde_json::to_string(&e).unwrap();
        let parsed: ErrorShape = serde_json::from_str(&json_str).unwrap();
        assert_eq!(e, parsed);
    }

    #[test]
    fn serde_omits_none_fields() {
        let e = error_shape(AGENT_TIMEOUT, "Agent timed out");
        let json_val: Value = serde_json::to_value(&e).unwrap();
        let obj = json_val.as_object().unwrap();
        assert!(!obj.contains_key("details"));
        assert!(!obj.contains_key("retryable"));
        assert!(!obj.contains_key("retryAfterMs"));
    }
}
