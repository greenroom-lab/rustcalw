use std::net::IpAddr;

/// Result of an origin check.
#[derive(Debug, Clone, PartialEq)]
pub enum OriginCheckResult {
    Ok { matched_by: OriginMatchedBy },
    Denied { reason: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum OriginMatchedBy {
    Allowlist,
    HostHeaderFallback,
    LocalLoopback,
}

/// Params for checking a browser Origin header.
pub struct OriginCheckParams<'a> {
    pub request_host: Option<&'a str>,
    pub origin: Option<&'a str>,
    pub allowed_origins: Option<&'a [String]>,
    pub allow_host_header_origin_fallback: bool,
    pub is_local_client: bool,
}

fn parse_origin(origin_raw: Option<&str>) -> Option<(String, String, String)> {
    let trimmed = origin_raw?.trim();
    if trimmed.is_empty() || trimmed == "null" {
        return None;
    }
    let url = url::Url::parse(trimmed).ok()?;
    let origin = url.origin().ascii_serialization().to_ascii_lowercase();
    let host = url
        .host_str()
        .map(|h| {
            if let Some(port) = url.port() {
                format!("{}:{}", h.to_ascii_lowercase(), port)
            } else {
                h.to_ascii_lowercase()
            }
        })
        .unwrap_or_default();
    let hostname = url
        .host_str()
        .unwrap_or("")
        .to_ascii_lowercase();
    Some((origin, host, hostname))
}

fn normalize_host_header(raw: Option<&str>) -> Option<String> {
    let trimmed = raw?.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_ascii_lowercase())
}

fn is_loopback_host(hostname: &str) -> bool {
    if hostname == "localhost" {
        return true;
    }
    if let Ok(ip) = hostname.parse::<IpAddr>() {
        return ip.is_loopback();
    }
    false
}

/// Check whether a browser Origin header is acceptable.
pub fn check_browser_origin(params: &OriginCheckParams) -> OriginCheckResult {
    let parsed = match parse_origin(params.origin) {
        Some(p) => p,
        None => {
            return OriginCheckResult::Denied {
                reason: "origin missing or invalid".to_string(),
            };
        }
    };
    let (origin, host, hostname) = parsed;

    // Check allowlist.
    if let Some(allowed) = params.allowed_origins {
        let allowlist: std::collections::HashSet<String> = allowed
            .iter()
            .map(|v| v.trim().to_ascii_lowercase())
            .filter(|v| !v.is_empty())
            .collect();

        if allowlist.contains("*") || allowlist.contains(&origin) {
            return OriginCheckResult::Ok {
                matched_by: OriginMatchedBy::Allowlist,
            };
        }
    }

    // Host header fallback.
    if params.allow_host_header_origin_fallback {
        if let Some(request_host) = normalize_host_header(params.request_host) {
            if host == request_host {
                return OriginCheckResult::Ok {
                    matched_by: OriginMatchedBy::HostHeaderFallback,
                };
            }
        }
    }

    // Dev fallback for local socket clients only.
    if params.is_local_client && is_loopback_host(&hostname) {
        return OriginCheckResult::Ok {
            matched_by: OriginMatchedBy::LocalLoopback,
        };
    }

    OriginCheckResult::Denied {
        reason: "origin not allowed".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_allows_any_origin() {
        let result = check_browser_origin(&OriginCheckParams {
            request_host: None,
            origin: Some("https://example.com"),
            allowed_origins: Some(&["*".to_string()]),
            allow_host_header_origin_fallback: false,
            is_local_client: false,
        });
        assert_eq!(
            result,
            OriginCheckResult::Ok {
                matched_by: OriginMatchedBy::Allowlist
            }
        );
    }

    #[test]
    fn specific_origin_allowed() {
        let result = check_browser_origin(&OriginCheckParams {
            request_host: None,
            origin: Some("https://myapp.com"),
            allowed_origins: Some(&["https://myapp.com".to_string()]),
            allow_host_header_origin_fallback: false,
            is_local_client: false,
        });
        assert_eq!(
            result,
            OriginCheckResult::Ok {
                matched_by: OriginMatchedBy::Allowlist
            }
        );
    }

    #[test]
    fn missing_origin_denied() {
        let result = check_browser_origin(&OriginCheckParams {
            request_host: None,
            origin: None,
            allowed_origins: None,
            allow_host_header_origin_fallback: false,
            is_local_client: false,
        });
        match result {
            OriginCheckResult::Denied { reason } => {
                assert!(reason.contains("missing"));
            }
            _ => panic!("expected denied"),
        }
    }

    #[test]
    fn null_origin_denied() {
        let result = check_browser_origin(&OriginCheckParams {
            request_host: None,
            origin: Some("null"),
            allowed_origins: None,
            allow_host_header_origin_fallback: false,
            is_local_client: false,
        });
        match result {
            OriginCheckResult::Denied { reason } => {
                assert!(reason.contains("missing"));
            }
            _ => panic!("expected denied"),
        }
    }

    #[test]
    fn host_header_fallback() {
        let result = check_browser_origin(&OriginCheckParams {
            request_host: Some("myapp.com"),
            origin: Some("https://myapp.com"),
            allowed_origins: None,
            allow_host_header_origin_fallback: true,
            is_local_client: false,
        });
        assert_eq!(
            result,
            OriginCheckResult::Ok {
                matched_by: OriginMatchedBy::HostHeaderFallback
            }
        );
    }

    #[test]
    fn localhost_loopback_for_local_client() {
        let result = check_browser_origin(&OriginCheckParams {
            request_host: None,
            origin: Some("http://localhost:3000"),
            allowed_origins: None,
            allow_host_header_origin_fallback: false,
            is_local_client: true,
        });
        assert_eq!(
            result,
            OriginCheckResult::Ok {
                matched_by: OriginMatchedBy::LocalLoopback
            }
        );
    }

    #[test]
    fn ip_loopback_for_local_client() {
        let result = check_browser_origin(&OriginCheckParams {
            request_host: None,
            origin: Some("http://127.0.0.1:8080"),
            allowed_origins: None,
            allow_host_header_origin_fallback: false,
            is_local_client: true,
        });
        assert_eq!(
            result,
            OriginCheckResult::Ok {
                matched_by: OriginMatchedBy::LocalLoopback
            }
        );
    }

    #[test]
    fn loopback_denied_for_non_local_client() {
        let result = check_browser_origin(&OriginCheckParams {
            request_host: None,
            origin: Some("http://localhost:3000"),
            allowed_origins: None,
            allow_host_header_origin_fallback: false,
            is_local_client: false,
        });
        match result {
            OriginCheckResult::Denied { .. } => {}
            _ => panic!("expected denied for non-local client"),
        }
    }

    #[test]
    fn origin_not_in_allowlist() {
        let result = check_browser_origin(&OriginCheckParams {
            request_host: None,
            origin: Some("https://evil.com"),
            allowed_origins: Some(&["https://good.com".to_string()]),
            allow_host_header_origin_fallback: false,
            is_local_client: false,
        });
        match result {
            OriginCheckResult::Denied { reason } => {
                assert!(reason.contains("not allowed"));
            }
            _ => panic!("expected denied"),
        }
    }
}
