//! Text segment joining — mirrors src/shared/text/join-segments.ts

/// Concatenate two optional text segments with a separator.
pub fn concat_optional_text_segments(
    left: Option<&str>,
    right: Option<&str>,
    separator: Option<&str>,
) -> Option<String> {
    let sep = separator.unwrap_or("\n\n");
    match (left, right) {
        (Some(l), Some(r)) => Some(format!("{}{}{}", l, sep, r)),
        (None, Some(r)) => Some(r.to_string()),
        (Some(l), None) => Some(l.to_string()),
        (None, None) => None,
    }
}

/// Join present (non-empty) text segments with a separator.
pub fn join_present_text_segments(
    segments: &[Option<&str>],
    separator: Option<&str>,
    trim: bool,
) -> Option<String> {
    let sep = separator.unwrap_or("\n\n");
    let values: Vec<String> = segments
        .iter()
        .filter_map(|s| *s)
        .map(|s| if trim { s.trim().to_string() } else { s.to_string() })
        .filter(|s| !s.is_empty())
        .collect();

    if values.is_empty() {
        None
    } else {
        Some(values.join(sep))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_both() {
        assert_eq!(
            concat_optional_text_segments(Some("a"), Some("b"), None),
            Some("a\n\nb".into())
        );
    }

    #[test]
    fn concat_left_only() {
        assert_eq!(
            concat_optional_text_segments(Some("a"), None, None),
            Some("a".into())
        );
    }

    #[test]
    fn concat_none() {
        assert_eq!(concat_optional_text_segments(None, None, None), None);
    }

    #[test]
    fn join_filters_empty() {
        let segments = vec![Some("a"), None, Some(""), Some("b")];
        assert_eq!(
            join_present_text_segments(&segments, Some(", "), false),
            Some("a, b".into())
        );
    }

    #[test]
    fn join_all_empty() {
        let segments: Vec<Option<&str>> = vec![None, None];
        assert_eq!(join_present_text_segments(&segments, None, false), None);
    }
}
