//! String summarization — mirrors src/shared/string-sample.ts

/// Summarize string entries with truncation and overflow indicator.
pub fn summarize_string_entries(
    entries: Option<&[String]>,
    limit: Option<usize>,
    empty_text: Option<&str>,
) -> String {
    let entries = entries.unwrap_or(&[]);
    if entries.is_empty() {
        return empty_text.unwrap_or("").to_string();
    }
    let limit = limit.map(|l| l.max(1)).unwrap_or(6);
    let sample = &entries[..entries.len().min(limit)];
    let suffix = if entries.len() > sample.len() {
        format!(" (+{})", entries.len() - sample.len())
    } else {
        String::new()
    };
    format!("{}{}", sample.join(", "), suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_returns_empty_text() {
        assert_eq!(summarize_string_entries(None, None, Some("none")), "none");
        assert_eq!(summarize_string_entries(Some(&[]), None, None), "");
    }

    #[test]
    fn within_limit() {
        let entries: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        assert_eq!(
            summarize_string_entries(Some(&entries), None, None),
            "a, b, c"
        );
    }

    #[test]
    fn exceeds_limit() {
        let entries: Vec<String> = vec!["a".into(), "b".into(), "c".into(), "d".into()];
        assert_eq!(
            summarize_string_entries(Some(&entries), Some(2), None),
            "a, b (+2)"
        );
    }
}
