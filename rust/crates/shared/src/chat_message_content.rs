//! Chat message content utilities — mirrors src/shared/chat-message-content.ts

use serde_json::Value;

/// Extract the text from the first content block of a message.
/// Mirrors `extractFirstTextBlock` from src/shared/chat-message-content.ts.
pub fn extract_first_text_block(message: &Value) -> Option<&str> {
    let content = message.as_object()?.get("content")?.as_array()?;
    let first = content.first()?.as_object()?;
    first.get("text")?.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_first_text_block() {
        let msg = json!({
            "content": [
                {"type": "text", "text": "hello world"},
                {"type": "text", "text": "second block"}
            ]
        });
        assert_eq!(extract_first_text_block(&msg), Some("hello world"));
    }

    #[test]
    fn returns_none_for_empty_content() {
        let msg = json!({"content": []});
        assert_eq!(extract_first_text_block(&msg), None);
    }

    #[test]
    fn returns_none_for_non_object() {
        assert_eq!(extract_first_text_block(&json!(null)), None);
        assert_eq!(extract_first_text_block(&json!("string")), None);
    }
}
