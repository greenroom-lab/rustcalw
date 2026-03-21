//! Assistant visible text — mirrors src/shared/text/assistant-visible-text.ts

use crate::text_code_regions::{find_code_regions, is_inside_code};
use crate::text_reasoning_tags::{strip_reasoning_tags_from_text, ReasoningTagMode, ReasoningTagTrim};
use regex::Regex;

/// Strip `<relevant_memories>` tags from text, respecting code blocks.
fn strip_relevant_memories_tags(text: &str) -> String {
    if text.is_empty() {
        return text.to_string();
    }

    let quick_re = Regex::new(r"(?i)<\s*/?\s*relevant[-_]memories\b").unwrap();
    if !quick_re.is_match(text) {
        return text.to_string();
    }

    let tag_re = Regex::new(r"(?i)<\s*(/?\s*)relevant[-_]memories\b[^<>]*>").unwrap();
    let code_regions = find_code_regions(text);

    let mut result = String::new();
    let mut last_index = 0;
    let mut in_memory_block = false;

    for caps in tag_re.captures_iter(text) {
        let full_match = caps.get(0).unwrap();
        let idx = full_match.start();

        if is_inside_code(idx, &code_regions) {
            continue;
        }

        let slash_part = caps.get(1).unwrap().as_str().trim();
        let is_close = slash_part.starts_with('/');

        if !in_memory_block {
            result.push_str(&text[last_index..idx]);
            if !is_close {
                in_memory_block = true;
            }
        } else if is_close {
            in_memory_block = false;
        }

        last_index = full_match.end();
    }

    if !in_memory_block {
        result.push_str(&text[last_index..]);
    }

    result
}

/// Strip internal scaffolding (reasoning tags, memory tags) from assistant text.
pub fn strip_assistant_internal_scaffolding(text: &str) -> String {
    let without_reasoning = strip_reasoning_tags_from_text(
        text,
        Some(ReasoningTagMode::Preserve),
        Some(ReasoningTagTrim::Start),
    );
    strip_relevant_memories_tags(&without_reasoning).trim_start().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_memory_tags() {
        let text = "<relevant_memories>internal</relevant_memories>visible";
        let result = strip_assistant_internal_scaffolding(text);
        assert_eq!(result, "visible");
    }

    #[test]
    fn strips_thinking_and_memory() {
        let text = "<thinking>thought</thinking><relevant_memories>mem</relevant_memories>visible";
        let result = strip_assistant_internal_scaffolding(text);
        assert_eq!(result, "visible");
    }

    #[test]
    fn preserves_normal_text() {
        let text = "just normal text";
        assert_eq!(strip_assistant_internal_scaffolding(text), "just normal text");
    }
}
