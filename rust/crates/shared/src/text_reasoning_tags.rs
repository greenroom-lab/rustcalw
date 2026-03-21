//! Reasoning tag stripping — mirrors src/shared/text/reasoning-tags.ts

use crate::text_code_regions::{find_code_regions, is_inside_code};
use regex::Regex;

/// Mode for handling unclosed reasoning tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasoningTagMode {
    /// Remove content when unclosed tag found.
    Strict,
    /// Preserve content when unclosed tag found.
    Preserve,
}

/// Trim mode for result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasoningTagTrim {
    None,
    Start,
    Both,
}

/// Strip reasoning/thinking tags from text, respecting code blocks.
pub fn strip_reasoning_tags_from_text(
    text: &str,
    mode: Option<ReasoningTagMode>,
    trim: Option<ReasoningTagTrim>,
) -> String {
    if text.is_empty() {
        return text.to_string();
    }

    let quick_re = Regex::new(r"(?i)<\s*/?\s*(?:think(?:ing)?|thought|antthinking|final)\b").unwrap();
    if !quick_re.is_match(text) {
        return text.to_string();
    }

    let mode = mode.unwrap_or(ReasoningTagMode::Strict);
    let trim_mode = trim.unwrap_or(ReasoningTagTrim::Both);

    let mut cleaned = text.to_string();

    // Strip <final> tags first
    let final_tag_re = Regex::new(r"(?i)<\s*/?\s*final\b[^<>]*>").unwrap();
    if final_tag_re.is_match(&cleaned) {
        let pre_code_regions = find_code_regions(&cleaned);
        let matches: Vec<(usize, usize)> = final_tag_re
            .find_iter(&cleaned)
            .map(|m| (m.start(), m.end()))
            .collect();

        // Remove non-code final tags in reverse order
        for &(start, end) in matches.iter().rev() {
            if !is_inside_code(start, &pre_code_regions) {
                cleaned = format!("{}{}", &cleaned[..start], &cleaned[end..]);
            }
        }
    }

    let code_regions = find_code_regions(&cleaned);
    let thinking_tag_re =
        Regex::new(r"(?i)<\s*(/?\s*)(?:think(?:ing)?|thought|antthinking)\b[^<>]*>").unwrap();

    let mut result = String::new();
    let mut last_index = 0;
    let mut in_thinking = false;

    for caps in thinking_tag_re.captures_iter(&cleaned) {
        let full_match = caps.get(0).unwrap();
        let idx = full_match.start();
        let slash_part = caps.get(1).unwrap().as_str().trim();
        let is_close = slash_part.starts_with('/');

        if is_inside_code(idx, &code_regions) {
            continue;
        }

        if !in_thinking {
            result.push_str(&cleaned[last_index..idx]);
            if !is_close {
                in_thinking = true;
            }
        } else if is_close {
            in_thinking = false;
        }

        last_index = full_match.end();
    }

    if !in_thinking || mode == ReasoningTagMode::Preserve {
        result.push_str(&cleaned[last_index..]);
    }

    match trim_mode {
        ReasoningTagTrim::None => result,
        ReasoningTagTrim::Start => result.trim_start().to_string(),
        ReasoningTagTrim::Both => result.trim().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_thinking_tags() {
        let text = "<thinking>internal</thinking>visible";
        let result = strip_reasoning_tags_from_text(text, None, Some(ReasoningTagTrim::None));
        assert_eq!(result, "visible");
    }

    #[test]
    fn preserves_code_blocks() {
        let text = "```\n<thinking>in code</thinking>\n```\nvisible";
        let result = strip_reasoning_tags_from_text(text, None, Some(ReasoningTagTrim::None));
        assert!(result.contains("<thinking>in code</thinking>"));
    }

    #[test]
    fn no_tags_returns_unchanged() {
        let text = "just normal text";
        assert_eq!(
            strip_reasoning_tags_from_text(text, None, None),
            "just normal text"
        );
    }

    #[test]
    fn strict_mode_removes_unclosed() {
        let text = "<thinking>unclosed content";
        let result =
            strip_reasoning_tags_from_text(text, Some(ReasoningTagMode::Strict), Some(ReasoningTagTrim::None));
        assert_eq!(result, "");
    }

    #[test]
    fn preserve_mode_keeps_unclosed() {
        let text = "<thinking>unclosed content";
        let result = strip_reasoning_tags_from_text(
            text,
            Some(ReasoningTagMode::Preserve),
            Some(ReasoningTagTrim::None),
        );
        assert_eq!(result, "unclosed content");
    }
}
