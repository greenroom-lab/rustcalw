/// Maximum WebSocket close reason bytes (RFC 6455 allows 123, we use 120
/// for safety like the TS implementation).
const CLOSE_REASON_MAX_BYTES: usize = 120;

/// Truncate a close reason string to fit within WebSocket close frame limits.
///
/// Returns `"invalid handshake"` for empty reasons.
pub fn truncate_close_reason(reason: &str) -> String {
    truncate_close_reason_with_max(reason, CLOSE_REASON_MAX_BYTES)
}

fn truncate_close_reason_with_max(reason: &str, max_bytes: usize) -> String {
    if reason.is_empty() {
        return "invalid handshake".to_string();
    }
    if reason.len() <= max_bytes {
        return reason.to_string();
    }
    // Truncate at a valid UTF-8 boundary.
    let mut end = max_bytes;
    while end > 0 && !reason.is_char_boundary(end) {
        end -= 1;
    }
    reason[..end].to_string()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_reason() {
        assert_eq!(truncate_close_reason(""), "invalid handshake");
    }

    #[test]
    fn short_reason_unchanged() {
        assert_eq!(truncate_close_reason("auth failed"), "auth failed");
    }

    #[test]
    fn exact_limit() {
        let reason = "a".repeat(CLOSE_REASON_MAX_BYTES);
        assert_eq!(truncate_close_reason(&reason), reason);
    }

    #[test]
    fn over_limit_truncated() {
        let reason = "a".repeat(CLOSE_REASON_MAX_BYTES + 50);
        let result = truncate_close_reason(&reason);
        assert_eq!(result.len(), CLOSE_REASON_MAX_BYTES);
    }

    #[test]
    fn multibyte_truncation_safe() {
        // Create a string with multi-byte characters that would split mid-char.
        let reason = "あ".repeat(50); // 3 bytes each = 150 bytes
        let result = truncate_close_reason(&reason);
        assert!(result.len() <= CLOSE_REASON_MAX_BYTES);
        // Must be valid UTF-8.
        assert!(result.is_char_boundary(result.len()));
        // Must be a multiple of 3 (each char is 3 bytes).
        assert_eq!(result.len() % 3, 0);
    }
}
