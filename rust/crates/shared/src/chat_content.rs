//! Chat content extraction — mirrors src/shared/chat-content.ts

use serde_json::Value;

/// Options for extracting text from chat content.
pub struct ExtractTextOptions<'a> {
    pub sanitize_text: Option<&'a dyn Fn(&str) -> String>,
    pub join_with: Option<&'a str>,
    pub normalize_text: Option<&'a dyn Fn(&str) -> String>,
}

impl Default for ExtractTextOptions<'_> {
    fn default() -> Self {
        Self {
            sanitize_text: None,
            join_with: None,
            normalize_text: None,
        }
    }
}

fn default_normalize(text: &str) -> String {
    let re = regex::Regex::new(r"\s+").expect("valid regex");
    re.replace_all(text, " ").trim().to_string()
}

/// Extract text from chat content (string or array of content blocks).
/// Mirrors `extractTextFromChatContent` from src/shared/chat-content.ts.
pub fn extract_text_from_chat_content(
    content: &Value,
    opts: Option<&ExtractTextOptions>,
) -> Option<String> {
    let normalize = opts
        .and_then(|o| o.normalize_text)
        .unwrap_or(&default_normalize);
    let join_with = opts.and_then(|o| o.join_with).unwrap_or(" ");

    if let Some(s) = content.as_str() {
        let value = if let Some(sanitize) = opts.and_then(|o| o.sanitize_text) {
            sanitize(s)
        } else {
            s.to_string()
        };
        let normalized = normalize(&value);
        return if normalized.is_empty() {
            None
        } else {
            Some(normalized)
        };
    }

    let arr = content.as_array()?;
    let mut chunks: Vec<String> = Vec::new();

    for block in arr {
        let obj = block.as_object()?;
        let block_type = obj.get("type").and_then(|v| v.as_str());
        if block_type != Some("text") {
            continue;
        }
        let text = match obj.get("text").and_then(|v| v.as_str()) {
            Some(t) => t,
            None => continue,
        };
        let value = if let Some(sanitize) = opts.and_then(|o| o.sanitize_text) {
            sanitize(text)
        } else {
            text.to_string()
        };
        if !value.trim().is_empty() {
            chunks.push(value);
        }
    }

    let joined = normalize(&chunks.join(join_with));
    if joined.is_empty() {
        None
    } else {
        Some(joined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extract_from_string() {
        let content = json!("  hello   world  ");
        let result = extract_text_from_chat_content(&content, None);
        assert_eq!(result, Some("hello world".into()));
    }

    #[test]
    fn extract_from_content_blocks() {
        let content = json!([
            {"type": "text", "text": "hello"},
            {"type": "image", "url": "http://example.com/img.png"},
            {"type": "text", "text": "world"}
        ]);
        let result = extract_text_from_chat_content(&content, None);
        assert_eq!(result, Some("hello world".into()));
    }

    #[test]
    fn returns_none_for_empty_content() {
        let content = json!("   ");
        let result = extract_text_from_chat_content(&content, None);
        assert_eq!(result, None);
    }

    #[test]
    fn returns_none_for_null() {
        let content = json!(null);
        let result = extract_text_from_chat_content(&content, None);
        assert_eq!(result, None);
    }
}
