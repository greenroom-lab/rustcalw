//! Tailscale status — mirrors src/shared/tailscale-status.ts

use std::future::Future;
use std::pin::Pin;

/// Result of running a command.
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub code: Option<i32>,
    pub stdout: String,
}

/// Command runner function type.
pub type CommandRunner = Box<
    dyn Fn(Vec<String>, u64) -> Pin<Box<dyn Future<Output = Result<CommandResult, String>> + Send>>
        + Send
        + Sync,
>;

const TAILSCALE_STATUS_COMMAND_CANDIDATES: &[&str] = &[
    "tailscale",
    "/Applications/Tailscale.app/Contents/MacOS/Tailscale",
];

/// Parse a possibly noisy JSON object from a string — find the first `{` and
/// last `}` and attempt to parse the substring.
fn parse_possibly_noisy_json_object(raw: &str) -> Option<serde_json::Value> {
    let start = raw.find('{')?;
    let end = raw.rfind('}')?;
    if end <= start {
        return None;
    }
    serde_json::from_str(&raw[start..=end]).ok()
}

/// Extract the Tailnet hostname from `tailscale status --json` output.
fn extract_tailnet_host_from_status_json(raw: &str) -> Option<String> {
    let parsed = parse_possibly_noisy_json_object(raw)?;
    let self_obj = parsed.get("Self")?;

    // Try DNSName first
    if let Some(dns) = self_obj.get("DNSName").and_then(|v| v.as_str()) {
        if !dns.is_empty() {
            return Some(dns.trim_end_matches('.').to_string());
        }
    }

    // Fall back to first TailscaleIPs entry
    let ips = self_obj.get("TailscaleIPs")?.as_array()?;
    ips.first()
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Resolve the local Tailnet hostname by running `tailscale status --json`.
pub async fn resolve_tailnet_host_with_runner(
    run_command: Option<&CommandRunner>,
) -> Option<String> {
    let runner = run_command?;

    for candidate in TAILSCALE_STATUS_COMMAND_CANDIDATES {
        let args = vec![
            candidate.to_string(),
            "status".to_string(),
            "--json".to_string(),
        ];
        let result = match runner(args, 5000).await {
            Ok(r) => r,
            Err(_) => continue,
        };
        if result.code != Some(0) {
            continue;
        }
        let raw = result.stdout.trim();
        if raw.is_empty() {
            continue;
        }
        if let Some(host) = extract_tailnet_host_from_status_json(raw) {
            return Some(host);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_json_clean() {
        let raw = r#"{"Self":{"DNSName":"myhost.tail.ts.net."}}"#;
        let host = extract_tailnet_host_from_status_json(raw);
        assert_eq!(host.as_deref(), Some("myhost.tail.ts.net"));
    }

    #[test]
    fn parse_json_with_noise() {
        let raw = "some noise\n{\"Self\":{\"DNSName\":\"host.example.com.\"}}\nmore noise";
        let host = extract_tailnet_host_from_status_json(raw);
        assert_eq!(host.as_deref(), Some("host.example.com"));
    }

    #[test]
    fn parse_json_fallback_to_ip() {
        let raw = r#"{"Self":{"DNSName":"","TailscaleIPs":["100.64.0.1","fd7a::1"]}}"#;
        let host = extract_tailnet_host_from_status_json(raw);
        assert_eq!(host.as_deref(), Some("100.64.0.1"));
    }

    #[test]
    fn parse_json_no_self() {
        let raw = r#"{"Version":"1.2.3"}"#;
        let host = extract_tailnet_host_from_status_json(raw);
        assert!(host.is_none());
    }

    #[test]
    fn parse_invalid_json() {
        let host = extract_tailnet_host_from_status_json("not json at all");
        assert!(host.is_none());
    }
}
