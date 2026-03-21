use std::collections::HashSet;

/// Result of canonicalizing a path for security checks.
#[derive(Debug, Clone)]
pub struct SecurityPathCanonicalization {
    pub canonical_path: String,
    pub candidates: Vec<String>,
    pub decode_passes: usize,
    pub decode_pass_limit_reached: bool,
    pub malformed_encoding: bool,
    pub raw_normalized_path: String,
}

const MAX_PATH_DECODE_PASSES: usize = 32;

fn normalize_path_separators(pathname: &str) -> String {
    // Collapse multiple slashes to one.
    let mut result = String::with_capacity(pathname.len());
    let mut prev_was_slash = false;
    for ch in pathname.chars() {
        if ch == '/' {
            if !prev_was_slash {
                result.push('/');
            }
            prev_was_slash = true;
        } else {
            result.push(ch);
            prev_was_slash = false;
        }
    }
    // Remove trailing slashes (but keep "/" itself).
    if result.len() > 1 {
        let trimmed = result.trim_end_matches('/');
        if trimmed.is_empty() {
            return "/".to_string();
        }
        return trimmed.to_string();
    }
    result
}

fn normalize_protected_prefix(prefix: &str) -> String {
    let lower = prefix.to_ascii_lowercase();
    let result = normalize_path_separators(&lower);
    if result.is_empty() {
        "/".to_string()
    } else {
        result
    }
}

/// Resolve `.` and `..` segments in a path.
fn resolve_dot_segments(pathname: &str) -> String {
    let mut segments: Vec<&str> = Vec::new();
    for seg in pathname.split('/') {
        match seg {
            "." => {}
            ".." => {
                segments.pop();
            }
            s => segments.push(s),
        }
    }
    let result = segments.join("/");
    if pathname.starts_with('/') && !result.starts_with('/') {
        format!("/{}", result)
    } else {
        result
    }
}

fn normalize_path_for_security(pathname: &str) -> String {
    let resolved = resolve_dot_segments(pathname);
    let lower = resolved.to_ascii_lowercase();
    let result = normalize_path_separators(&lower);
    if result.is_empty() {
        "/".to_string()
    } else {
        result
    }
}

fn push_normalized_candidate(
    candidates: &mut Vec<String>,
    seen: &mut HashSet<String>,
    value: &str,
) {
    let normalized = normalize_path_for_security(value);
    if seen.contains(&normalized) {
        return;
    }
    seen.insert(normalized.clone());
    candidates.push(normalized);
}

/// Build all canonical path candidates by iteratively URL-decoding.
pub fn build_canonical_path_candidates(
    pathname: &str,
    max_decode_passes: usize,
) -> (Vec<String>, usize, bool, bool) {
    let mut candidates = Vec::new();
    let mut seen = HashSet::new();
    push_normalized_candidate(&mut candidates, &mut seen, pathname);

    let mut decoded = pathname.to_string();
    let mut malformed_encoding = false;
    let mut decode_passes = 0;

    for _ in 0..max_decode_passes {
        match urlencoding::decode(&decoded) {
            Ok(next_decoded) => {
                if next_decoded.as_ref() == decoded {
                    break;
                }
                decode_passes += 1;
                decoded = next_decoded.into_owned();
                push_normalized_candidate(&mut candidates, &mut seen, &decoded);
            }
            Err(_) => {
                malformed_encoding = true;
                break;
            }
        }
    }

    let mut decode_pass_limit_reached = false;
    if !malformed_encoding {
        match urlencoding::decode(&decoded) {
            Ok(next) => {
                decode_pass_limit_reached = next.as_ref() != decoded;
            }
            Err(_) => {
                malformed_encoding = true;
            }
        }
    }

    (
        candidates,
        decode_passes,
        decode_pass_limit_reached,
        malformed_encoding,
    )
}

/// Get the most-decoded canonical path variant.
pub fn canonicalize_path_variant(pathname: &str) -> String {
    let (candidates, _, _, _) =
        build_canonical_path_candidates(pathname, MAX_PATH_DECODE_PASSES);
    candidates.last().cloned().unwrap_or_else(|| "/".to_string())
}

fn prefix_match(pathname: &str, prefix: &str) -> bool {
    pathname == prefix
        || pathname.starts_with(&format!("{}/", prefix))
        // Fail closed when malformed %-encoding follows the protected prefix.
        || pathname.starts_with(&format!("{}%", prefix))
}

/// Full canonicalization with anomaly detection.
pub fn canonicalize_path_for_security(pathname: &str) -> SecurityPathCanonicalization {
    let (candidates, decode_passes, decode_pass_limit_reached, malformed_encoding) =
        build_canonical_path_candidates(pathname, MAX_PATH_DECODE_PASSES);

    let canonical_path = candidates.last().cloned().unwrap_or_else(|| "/".to_string());
    let raw_normalized = normalize_path_separators(&pathname.to_ascii_lowercase());
    let raw_normalized_path = if raw_normalized.is_empty() {
        "/".to_string()
    } else {
        raw_normalized
    };

    SecurityPathCanonicalization {
        canonical_path,
        candidates,
        decode_passes,
        decode_pass_limit_reached,
        malformed_encoding,
        raw_normalized_path,
    }
}

/// Returns `true` if canonicalization produced anomalies (malformed encoding
/// or decode-depth limit reached).
pub fn has_security_path_canonicalization_anomaly(pathname: &str) -> bool {
    let canonical = canonicalize_path_for_security(pathname);
    canonical.malformed_encoding || canonical.decode_pass_limit_reached
}

/// Check whether a path falls under any of the protected prefixes.
pub fn is_path_protected_by_prefixes(pathname: &str, prefixes: &[&str]) -> bool {
    let canonical = canonicalize_path_for_security(pathname);
    let normalized_prefixes: Vec<String> = prefixes
        .iter()
        .map(|p| normalize_protected_prefix(p))
        .collect();

    if canonical.candidates.iter().any(|candidate| {
        normalized_prefixes
            .iter()
            .any(|prefix| prefix_match(candidate, prefix))
    }) {
        return true;
    }

    // Fail closed when canonicalization depth cannot be fully resolved.
    if canonical.decode_pass_limit_reached {
        return true;
    }

    if !canonical.malformed_encoding {
        return false;
    }

    normalized_prefixes
        .iter()
        .any(|prefix| prefix_match(&canonical.raw_normalized_path, prefix))
}

pub const PROTECTED_PLUGIN_ROUTE_PREFIXES: &[&str] = &["/api/channels"];

pub fn is_protected_plugin_route_path(pathname: &str) -> bool {
    is_path_protected_by_prefixes(pathname, PROTECTED_PLUGIN_ROUTE_PREFIXES)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_collapses_slashes() {
        assert_eq!(normalize_path_separators("///a///b//"), "/a/b");
    }

    #[test]
    fn normalize_preserves_root() {
        assert_eq!(normalize_path_separators("/"), "/");
    }

    #[test]
    fn resolve_dot_segments_basic() {
        assert_eq!(resolve_dot_segments("/a/b/../c"), "/a/c");
        assert_eq!(resolve_dot_segments("/a/./b"), "/a/b");
        assert_eq!(resolve_dot_segments("/a/b/../../c"), "/c");
    }

    #[test]
    fn canonicalize_basic_path() {
        assert_eq!(canonicalize_path_variant("/api/channels"), "/api/channels");
    }

    #[test]
    fn canonicalize_encoded_path() {
        assert_eq!(
            canonicalize_path_variant("/api/%63hannels"),
            "/api/channels"
        );
    }

    #[test]
    fn canonicalize_double_encoded_path() {
        assert_eq!(
            canonicalize_path_variant("/api/%2563hannels"),
            "/api/channels"
        );
    }

    #[test]
    fn protected_prefix_match() {
        assert!(is_protected_plugin_route_path("/api/channels"));
        assert!(is_protected_plugin_route_path("/api/channels/telegram"));
        assert!(is_protected_plugin_route_path("/API/Channels"));
    }

    #[test]
    fn protected_prefix_no_match() {
        assert!(!is_protected_plugin_route_path("/api/other"));
        assert!(!is_protected_plugin_route_path("/health"));
    }

    #[test]
    fn protected_encoded_prefix() {
        assert!(is_protected_plugin_route_path("/api/%63hannels"));
    }

    #[test]
    fn dot_traversal_protection() {
        assert!(is_protected_plugin_route_path("/api/foo/../channels"));
    }

    #[test]
    fn no_anomaly_for_normal_path() {
        assert!(!has_security_path_canonicalization_anomaly("/api/channels"));
    }
}
