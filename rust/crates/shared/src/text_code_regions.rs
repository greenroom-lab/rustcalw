//! Code region detection — mirrors src/shared/text/code-regions.ts

use regex::Regex;

/// A region within text that is code (fenced or inline).
#[derive(Debug, Clone)]
pub struct CodeRegion {
    pub start: usize,
    pub end: usize,
}

/// Find all code regions (fenced and inline) in text.
pub fn find_code_regions(text: &str) -> Vec<CodeRegion> {
    let mut regions: Vec<CodeRegion> = Vec::new();

    // Fenced code blocks: find opening ``` or ~~~ at start of line,
    // then find matching closing fence on its own line.
    // Rust regex doesn't support backreferences, so we do this manually.
    let fence_open_re = Regex::new(r"(?m)^(```|~~~)[^\n]*\n").unwrap();
    let mut search_from = 0;
    while search_from < text.len() {
        let haystack = &text[search_from..];
        let open_match = match fence_open_re.find(haystack) {
            Some(m) => m,
            None => break,
        };

        let fence_marker = if haystack[open_match.start()..].starts_with("```") {
            "```"
        } else {
            "~~~"
        };

        let block_start = search_from + open_match.start();
        let content_start = search_from + open_match.end();

        // Find closing fence: same marker at start of line
        let close_pattern = format!(r"(?m)^{}", regex::escape(fence_marker));
        let close_re = Regex::new(&close_pattern).unwrap();

        let block_end = if let Some(close_match) = close_re.find(&text[content_start..]) {
            let close_abs = content_start + close_match.start() + close_match.as_str().len();
            // Include trailing newline if present
            if close_abs < text.len() && text.as_bytes()[close_abs] == b'\n' {
                close_abs + 1
            } else {
                close_abs
            }
        } else {
            text.len()
        };

        regions.push(CodeRegion {
            start: block_start,
            end: block_end,
        });
        search_from = block_end;
    }

    // Inline code: `...`
    let inline_re = Regex::new(r"`+[^`]+`+").unwrap();
    for m in inline_re.find_iter(text) {
        let start = m.start();
        let end = m.end();
        let inside_fenced = regions.iter().any(|r| start >= r.start && end <= r.end);
        if !inside_fenced {
            regions.push(CodeRegion { start, end });
        }
    }

    regions.sort_by_key(|r| r.start);
    regions
}

/// Check if a position falls inside a code region.
pub fn is_inside_code(pos: usize, regions: &[CodeRegion]) -> bool {
    regions.iter().any(|r| pos >= r.start && pos < r.end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_fenced_code() {
        let text = "before\n```\ncode here\n```\nafter";
        let regions = find_code_regions(text);
        assert!(!regions.is_empty());
        assert!(is_inside_code(regions[0].start + 1, &regions));
        assert!(!is_inside_code(0, &regions));
    }

    #[test]
    fn finds_inline_code() {
        let text = "some `inline code` here";
        let regions = find_code_regions(text);
        assert_eq!(regions.len(), 1);
        assert!(is_inside_code(6, &regions));
    }

    #[test]
    fn empty_text() {
        assert!(find_code_regions("").is_empty());
    }
}
