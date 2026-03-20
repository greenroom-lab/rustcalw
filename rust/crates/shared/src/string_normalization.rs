//! String normalization utilities — mirrors src/shared/string-normalization.ts

use regex::Regex;
use std::sync::LazyLock;

/// Trim and filter non-empty strings from a list of values.
pub fn normalize_string_entries(list: &[String]) -> Vec<String> {
    list.iter()
        .map(|e| e.trim().to_string())
        .filter(|e| !e.is_empty())
        .collect()
}

/// Trim, lowercase, and filter non-empty strings from a list of values.
pub fn normalize_string_entries_lower(list: &[String]) -> Vec<String> {
    normalize_string_entries(list)
        .into_iter()
        .map(|e| e.to_lowercase())
        .collect()
}

static WHITESPACE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());
static SLUG_INVALID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[^a-z0-9#@._+\-]+").unwrap());
static MULTI_DASH_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-{2,}").unwrap());
static LEADING_TRAILING_DASH_DOT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[.\-]+|[.\-]+$").unwrap());

static AT_HASH_PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[@#]+").unwrap());
static WHITESPACE_UNDERSCORE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\s_]+").unwrap());
static AT_HASH_INVALID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[^a-z0-9\-]+").unwrap());
static LEADING_TRAILING_DASH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^-+|-+$").unwrap());

/// Normalize a string into a hyphen-separated slug.
pub fn normalize_hyphen_slug(raw: Option<&str>) -> String {
    let trimmed = raw.unwrap_or("").trim().to_lowercase();
    if trimmed.is_empty() {
        return String::new();
    }
    let dashed = WHITESPACE_RE.replace_all(&trimmed, "-");
    let cleaned = SLUG_INVALID_RE.replace_all(&dashed, "-");
    let deduped = MULTI_DASH_RE.replace_all(&cleaned, "-");
    LEADING_TRAILING_DASH_DOT
        .replace_all(&deduped, "")
        .to_string()
}

/// Normalize a string into a slug, stripping leading @/# prefixes.
pub fn normalize_at_hash_slug(raw: Option<&str>) -> String {
    let trimmed = raw.unwrap_or("").trim().to_lowercase();
    if trimmed.is_empty() {
        return String::new();
    }
    let without_prefix = AT_HASH_PREFIX_RE.replace_all(&trimmed, "");
    let dashed = WHITESPACE_UNDERSCORE_RE.replace_all(&without_prefix, "-");
    let cleaned = AT_HASH_INVALID_RE.replace_all(&dashed, "-");
    let deduped = MULTI_DASH_RE.replace_all(&cleaned, "-");
    LEADING_TRAILING_DASH
        .replace_all(&deduped, "")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_string_entries_trims_and_filters() {
        let input = vec![
            "  hello  ".into(),
            "".into(),
            "  ".into(),
            "world".into(),
        ];
        assert_eq!(normalize_string_entries(&input), vec!["hello", "world"]);
    }

    #[test]
    fn normalize_hyphen_slug_basic() {
        assert_eq!(normalize_hyphen_slug(Some("Hello World")), "hello-world");
        assert_eq!(normalize_hyphen_slug(Some("  A B  C  ")), "a-b-c");
        assert_eq!(normalize_hyphen_slug(None), "");
        assert_eq!(normalize_hyphen_slug(Some("")), "");
    }

    #[test]
    fn normalize_at_hash_slug_strips_prefix() {
        assert_eq!(normalize_at_hash_slug(Some("@user_name")), "user-name");
        assert_eq!(normalize_at_hash_slug(Some("#channel name")), "channel-name");
    }
}
