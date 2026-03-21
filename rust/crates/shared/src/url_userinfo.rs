//! URL userinfo stripping — mirrors src/shared/net/url-userinfo.ts

use url::Url;

/// Strip username and password from a URL string.
/// Returns the original string if parsing fails.
pub fn strip_url_user_info(value: &str) -> String {
    let Ok(mut parsed) = Url::parse(value) else {
        return value.to_string();
    };
    if parsed.username().is_empty() && parsed.password().is_none() {
        return value.to_string();
    }
    let _ = parsed.set_username("");
    let _ = parsed.set_password(None);
    parsed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_userinfo_unchanged() {
        let url = "https://example.com/path";
        assert_eq!(strip_url_user_info(url), url);
    }

    #[test]
    fn strips_username_and_password() {
        let url = "https://user:pass@example.com/path";
        let result = strip_url_user_info(url);
        assert!(!result.contains("user"));
        assert!(!result.contains("pass"));
        assert!(result.contains("example.com/path"));
    }

    #[test]
    fn strips_username_only() {
        let url = "https://user@example.com/path";
        let result = strip_url_user_info(url);
        assert!(!result.contains("user@"));
        assert!(result.contains("example.com/path"));
    }

    #[test]
    fn invalid_url_returns_original() {
        let url = "not a url";
        assert_eq!(strip_url_user_info(url), url);
    }
}
