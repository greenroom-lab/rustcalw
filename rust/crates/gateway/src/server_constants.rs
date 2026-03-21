/// Maximum WebSocket payload size (25 MiB).
/// Aligned with gateway client to prevent disconnection on large payloads.
pub const MAX_PAYLOAD_BYTES: usize = 25 * 1024 * 1024;

/// Per-connection send buffer limit (2x max payload).
pub const MAX_BUFFERED_BYTES: usize = 50 * 1024 * 1024;

/// Maximum payload size for pre-authentication messages.
pub const MAX_PREAUTH_PAYLOAD_BYTES: usize = 64 * 1024;

/// Default max bytes for chat history message responses.
pub const DEFAULT_MAX_CHAT_HISTORY_MESSAGES_BYTES: usize = 6 * 1024 * 1024;

/// Default handshake timeout in milliseconds.
pub const DEFAULT_HANDSHAKE_TIMEOUT_MS: u64 = 10_000;

/// Tick interval in milliseconds.
pub const TICK_INTERVAL_MS: u64 = 30_000;

/// Health refresh interval in milliseconds.
pub const HEALTH_REFRESH_INTERVAL_MS: u64 = 60_000;

/// Idempotency deduplication TTL in milliseconds.
pub const DEDUPE_TTL_MS: u64 = 5 * 60_000;

/// Maximum number of deduplication entries.
pub const DEDUPE_MAX: usize = 1000;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_payload_is_25mb() {
        assert_eq!(MAX_PAYLOAD_BYTES, 25 * 1024 * 1024);
    }

    #[test]
    fn max_buffered_is_2x_payload() {
        assert_eq!(MAX_BUFFERED_BYTES, 2 * MAX_PAYLOAD_BYTES);
    }

    #[test]
    fn dedupe_ttl_is_5_minutes() {
        assert_eq!(DEDUPE_TTL_MS, 300_000);
    }
}
