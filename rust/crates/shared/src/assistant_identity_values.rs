//! Identity value coercion — mirrors src/shared/assistant-identity-values.ts

/// Trims and validates a string identity value, truncating to max_length.
pub fn coerce_identity_value(value: Option<&str>, max_length: usize) -> Option<String> {
    let trimmed = value?.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.len() <= max_length {
        Some(trimmed.to_string())
    } else {
        Some(trimmed[..max_length].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_none_for_none() {
        assert_eq!(coerce_identity_value(None, 50), None);
    }

    #[test]
    fn returns_none_for_empty() {
        assert_eq!(coerce_identity_value(Some(""), 50), None);
        assert_eq!(coerce_identity_value(Some("   "), 50), None);
    }

    #[test]
    fn trims_whitespace() {
        assert_eq!(
            coerce_identity_value(Some("  hello  "), 50),
            Some("hello".into())
        );
    }

    #[test]
    fn truncates_to_max_length() {
        assert_eq!(
            coerce_identity_value(Some("abcdefgh"), 5),
            Some("abcde".into())
        );
    }

    #[test]
    fn passes_through_short_values() {
        assert_eq!(
            coerce_identity_value(Some("hi"), 50),
            Some("hi".into())
        );
    }
}
