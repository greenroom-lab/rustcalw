//! Chat envelope stripping — mirrors src/shared/chat-envelope.ts

use regex::Regex;
use std::sync::LazyLock;

static ENVELOPE_PREFIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\[([^\]]+)\]\s*").expect("valid regex"));

static ISO_DATETIME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}Z\b").expect("valid regex"));

static DATETIME_SPACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}\b").expect("valid regex"));

static MESSAGE_ID_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^\s*\[message_id:\s*[^\]]+\]\s*$").expect("valid regex"));

const ENVELOPE_CHANNELS: &[&str] = &[
    "WebChat",
    "WhatsApp",
    "Telegram",
    "Signal",
    "Slack",
    "Discord",
    "Google Chat",
    "iMessage",
    "Teams",
    "Matrix",
    "Zalo",
    "Zalo Personal",
    "BlueBubbles",
];

fn looks_like_envelope_header(header: &str) -> bool {
    if ISO_DATETIME.is_match(header) {
        return true;
    }
    if DATETIME_SPACE.is_match(header) {
        return true;
    }
    ENVELOPE_CHANNELS
        .iter()
        .any(|label| header.starts_with(&format!("{label} ")))
}

/// Strip envelope header prefix from a chat message text.
/// Mirrors `stripEnvelope` from src/shared/chat-envelope.ts.
pub fn strip_envelope(text: &str) -> &str {
    let Some(m) = ENVELOPE_PREFIX.find(text) else {
        return text;
    };
    let caps = ENVELOPE_PREFIX.captures(text).expect("matched above");
    let header = caps.get(1).map_or("", |m| m.as_str());
    if !looks_like_envelope_header(header) {
        return text;
    }
    &text[m.end()..]
}

/// Strip `[message_id: ...]` hint lines from text.
/// Mirrors `stripMessageIdHints` from src/shared/chat-envelope.ts.
pub fn strip_message_id_hints(text: &str) -> String {
    if !text.contains("[message_id:") && !text.contains("[MESSAGE_ID:") {
        return text.to_string();
    }
    let lines: Vec<&str> = text.lines().collect();
    let filtered: Vec<&str> = lines
        .iter()
        .filter(|line| !MESSAGE_ID_LINE.is_match(line))
        .copied()
        .collect();
    if filtered.len() == lines.len() {
        text.to_string()
    } else {
        filtered.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_envelope_with_timestamp() {
        let text = "[2024-01-15T10:30Z WhatsApp] Hello world";
        assert_eq!(strip_envelope(text), "Hello world");
    }

    #[test]
    fn strip_envelope_with_channel_prefix() {
        let text = "[Telegram 12345] Hi there";
        assert_eq!(strip_envelope(text), "Hi there");
    }

    #[test]
    fn strip_envelope_no_match() {
        let text = "[some random text] not an envelope";
        assert_eq!(strip_envelope(text), text);
    }

    #[test]
    fn strip_envelope_no_brackets() {
        let text = "plain message";
        assert_eq!(strip_envelope(text), text);
    }

    #[test]
    fn strip_message_id_hints_removes_lines() {
        let text = "Hello\n  [message_id: abc123]  \nWorld";
        let result = strip_message_id_hints(text);
        assert_eq!(result, "Hello\nWorld");
    }

    #[test]
    fn strip_message_id_hints_no_match() {
        let text = "Hello\nWorld";
        assert_eq!(strip_message_id_hints(text), text);
    }
}
