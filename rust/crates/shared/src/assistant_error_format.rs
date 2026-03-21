//! Assistant error formatting — mirrors src/shared/assistant-error-format.ts

use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;
use std::sync::LazyLock;

static ERROR_PAYLOAD_PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)^(?:error|(?:[a-z][\w-]*\s+)?api\s*error|apierror|openai\s*error|anthropic\s*error|gateway\s*error)(?:\s+\d{3})?[:\s-]+"
    ).unwrap()
});

static HTTP_STATUS_PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(?:http\s*)?(\d{3})\s+(.+)$").unwrap()
});

static HTTP_STATUS_CODE_PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?is)^(?:http\s*)?(\d{3})(?:\s+([\s\S]+))?$").unwrap()
});

static HTML_ERROR_PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^\s*(?:<!doctype\s+html\b|<html\b)").unwrap()
});

static HTML_CLOSE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)</html>").unwrap()
});

static CLOUDFLARE_HTML_ERROR_CODES: LazyLock<HashSet<u16>> = LazyLock::new(|| {
    [521, 522, 523, 524, 525, 526, 530].into_iter().collect()
});

/// Parsed API error info.
#[derive(Debug, Clone, Default)]
pub struct ApiErrorInfo {
    pub http_code: Option<String>,
    pub error_type: Option<String>,
    pub message: Option<String>,
    pub request_id: Option<String>,
}

fn is_error_payload_object(payload: &Value) -> bool {
    let obj = match payload.as_object() {
        Some(o) => o,
        None => return false,
    };
    if obj.get("type").and_then(|v| v.as_str()) == Some("error") {
        return true;
    }
    if obj.get("request_id").and_then(|v| v.as_str()).is_some()
        || obj.get("requestId").and_then(|v| v.as_str()).is_some()
    {
        return true;
    }
    if let Some(err) = obj.get("error").and_then(|v| v.as_object()) {
        if err.get("message").and_then(|v| v.as_str()).is_some()
            || err.get("type").and_then(|v| v.as_str()).is_some()
            || err.get("code").and_then(|v| v.as_str()).is_some()
        {
            return true;
        }
    }
    false
}

fn parse_api_error_payload(raw: &str) -> Option<Value> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut candidates = vec![trimmed.to_string()];
    if ERROR_PAYLOAD_PREFIX_RE.is_match(trimmed) {
        let stripped = ERROR_PAYLOAD_PREFIX_RE.replace(trimmed, "").trim().to_string();
        candidates.push(stripped);
    }

    for candidate in &candidates {
        if !candidate.starts_with('{') || !candidate.ends_with('}') {
            continue;
        }
        if let Ok(parsed) = serde_json::from_str::<Value>(candidate) {
            if is_error_payload_object(&parsed) {
                return Some(parsed);
            }
        }
    }
    None
}

/// Extract a leading HTTP status code from a raw error string.
pub fn extract_leading_http_status(raw: &str) -> Option<(u16, String)> {
    let caps = HTTP_STATUS_CODE_PREFIX_RE.captures(raw)?;
    let code: u16 = caps.get(1)?.as_str().parse().ok()?;
    let rest = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("").to_string();
    Some((code, rest))
}

/// Detect Cloudflare or HTML error pages.
pub fn is_cloudflare_or_html_error_page(raw: &str) -> bool {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return false;
    }
    let (code, rest) = match extract_leading_http_status(trimmed) {
        Some(v) => v,
        None => return false,
    };
    if code < 500 {
        return false;
    }
    if CLOUDFLARE_HTML_ERROR_CODES.contains(&code) {
        return true;
    }
    code < 600 && HTML_ERROR_PREFIX_RE.is_match(&rest) && HTML_CLOSE_RE.is_match(&rest)
}

/// Parse structured API error info from a raw error string.
pub fn parse_api_error_info(raw: Option<&str>) -> Option<ApiErrorInfo> {
    let trimmed = raw?.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut http_code: Option<String> = None;
    let mut candidate = trimmed.to_string();

    // Check for leading HTTP status code
    let http_prefix_re = Regex::new(r"(?s)^(\d{3})\s+(.+)$").unwrap();
    if let Some(caps) = http_prefix_re.captures(&candidate) {
        http_code = Some(caps[1].to_string());
        candidate = caps[2].trim().to_string();
    }

    let payload = parse_api_error_payload(&candidate)?;
    let obj = payload.as_object()?;

    let request_id = obj
        .get("request_id")
        .and_then(|v| v.as_str())
        .or_else(|| obj.get("requestId").and_then(|v| v.as_str()))
        .map(String::from);

    let top_type = obj.get("type").and_then(|v| v.as_str()).map(String::from);
    let top_message = obj.get("message").and_then(|v| v.as_str()).map(String::from);

    let mut err_type: Option<String> = None;
    let mut err_message: Option<String> = None;

    if let Some(err) = obj.get("error").and_then(|v| v.as_object()) {
        if let Some(t) = err.get("type").and_then(|v| v.as_str()) {
            err_type = Some(t.to_string());
        }
        if err_type.is_none() {
            if let Some(c) = err.get("code").and_then(|v| v.as_str()) {
                err_type = Some(c.to_string());
            }
        }
        if let Some(m) = err.get("message").and_then(|v| v.as_str()) {
            err_message = Some(m.to_string());
        }
    }

    Some(ApiErrorInfo {
        http_code,
        error_type: err_type.or(top_type),
        message: err_message.or(top_message),
        request_id,
    })
}

/// Format a raw assistant error for UI display.
pub fn format_raw_assistant_error_for_ui(raw: Option<&str>) -> String {
    let trimmed = raw.unwrap_or("").trim().to_string();
    if trimmed.is_empty() {
        return "LLM request failed with an unknown error.".to_string();
    }

    if is_cloudflare_or_html_error_page(&trimmed) {
        if let Some((code, _)) = extract_leading_http_status(&trimmed) {
            return format!(
                "The AI service is temporarily unavailable (HTTP {}). Please try again in a moment.",
                code
            );
        }
    }

    if let Some(caps) = HTTP_STATUS_PREFIX_RE.captures(&trimmed) {
        let code = &caps[1];
        let rest = caps[2].trim();
        if !rest.starts_with('{') {
            return format!("HTTP {}: {}", code, rest);
        }
    }

    if let Some(info) = parse_api_error_info(Some(&trimmed)) {
        if let Some(ref message) = info.message {
            let prefix = info
                .http_code
                .as_ref()
                .map(|c| format!("HTTP {}", c))
                .unwrap_or_else(|| "LLM error".to_string());
            let type_str = info.error_type.as_ref().map(|t| format!(" {}", t)).unwrap_or_default();
            let req_id = info
                .request_id
                .as_ref()
                .map(|r| format!(" (request_id: {})", r))
                .unwrap_or_default();
            return format!("{}{}: {}{}", prefix, type_str, message, req_id);
        }
    }

    if trimmed.len() > 600 {
        format!("{}…", &trimmed[..600])
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_error_for_empty() {
        assert_eq!(
            format_raw_assistant_error_for_ui(None),
            "LLM request failed with an unknown error."
        );
        assert_eq!(
            format_raw_assistant_error_for_ui(Some("")),
            "LLM request failed with an unknown error."
        );
    }

    #[test]
    fn detects_cloudflare_error() {
        assert!(is_cloudflare_or_html_error_page("521 <!DOCTYPE html><html></html>"));
        assert!(!is_cloudflare_or_html_error_page("200 OK"));
    }

    #[test]
    fn parses_api_error_json() {
        let raw = r#"{"error": {"type": "rate_limit", "message": "Too many requests"}, "request_id": "abc-123"}"#;
        let info = parse_api_error_info(Some(raw)).unwrap();
        assert_eq!(info.error_type.as_deref(), Some("rate_limit"));
        assert_eq!(info.message.as_deref(), Some("Too many requests"));
        assert_eq!(info.request_id.as_deref(), Some("abc-123"));
    }

    #[test]
    fn formats_api_error_for_ui() {
        let raw = r#"429 {"error": {"type": "rate_limit", "message": "Too many requests"}}"#;
        let formatted = format_raw_assistant_error_for_ui(Some(raw));
        assert!(formatted.contains("HTTP 429"));
        assert!(formatted.contains("Too many requests"));
    }

    #[test]
    fn extract_http_status() {
        let result = extract_leading_http_status("503 Service Unavailable");
        assert!(result.is_some());
        let (code, rest) = result.unwrap();
        assert_eq!(code, 503);
        assert_eq!(rest, "Service Unavailable");
    }
}
