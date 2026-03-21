//! Text chunking — mirrors src/shared/text-chunking.ts

/// Split `text` into chunks of at most `limit` characters, using
/// `resolve_break_index` to find a preferred break position within each window.
///
/// The callback receives a window `&str` of up to `limit` chars and returns a
/// preferred byte-offset to break at. If the returned index is not in `1..=limit`
/// or is not finite, the chunk is cut at `limit`.
pub fn chunk_text_by_break_resolver<F>(text: &str, limit: usize, resolve_break_index: F) -> Vec<String>
where
    F: Fn(&str) -> Option<usize>,
{
    if text.is_empty() {
        return Vec::new();
    }
    if limit == 0 || text.len() <= limit {
        return vec![text.to_string()];
    }

    let mut chunks: Vec<String> = Vec::new();
    let mut remaining = text;

    while remaining.len() > limit {
        let window = &remaining[..limit];
        let candidate = resolve_break_index(window);
        let break_idx = match candidate {
            Some(idx) if idx > 0 && idx <= limit => idx,
            _ => limit,
        };

        let raw_chunk = &remaining[..break_idx];
        let chunk = raw_chunk.trim_end();
        if !chunk.is_empty() {
            chunks.push(chunk.to_string());
        }

        let broke_on_separator =
            break_idx < remaining.len() && remaining.as_bytes()[break_idx].is_ascii_whitespace();
        let next_start = std::cmp::min(remaining.len(), break_idx + usize::from(broke_on_separator));
        remaining = remaining[next_start..].trim_start();
    }

    if !remaining.is_empty() {
        chunks.push(remaining.to_string());
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        let result = chunk_text_by_break_resolver("", 10, |_| None);
        assert!(result.is_empty());
    }

    #[test]
    fn text_within_limit() {
        let result = chunk_text_by_break_resolver("hello", 10, |_| None);
        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn zero_limit_returns_whole() {
        let result = chunk_text_by_break_resolver("hello", 0, |_| None);
        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn splits_at_break_resolver() {
        // Break at last space within window (limit 12 includes trailing space)
        let result = chunk_text_by_break_resolver("hello world foo bar", 12, |window| {
            window.rfind(' ')
        });
        assert_eq!(result, vec!["hello world", "foo bar"]);
    }

    #[test]
    fn splits_at_break_resolver_tight() {
        // With limit 11, rfind(' ') on "hello world" returns 5 (the space)
        let result = chunk_text_by_break_resolver("hello world foo bar", 11, |window| {
            window.rfind(' ')
        });
        assert_eq!(result, vec!["hello", "world foo", "bar"]);
    }

    #[test]
    fn hard_break_when_no_space() {
        let result = chunk_text_by_break_resolver("abcdefghij", 5, |_| None);
        assert_eq!(result, vec!["abcde", "fghij"]);
    }

    #[test]
    fn trims_trailing_whitespace() {
        let result = chunk_text_by_break_resolver("abc   def", 5, |window| {
            window.rfind(' ')
        });
        // "abc" (trimmed from "abc  "), then "def"
        assert_eq!(result, vec!["abc", "def"]);
    }
}
