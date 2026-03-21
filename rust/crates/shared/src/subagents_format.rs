//! Subagent/usage formatting — mirrors src/shared/subagents-format.ts

/// Format a duration in milliseconds to compact form (e.g., "5m", "2h30m", "1d").
pub fn format_duration_compact(value_ms: Option<f64>) -> String {
    let ms = match value_ms {
        Some(v) if v.is_finite() && v > 0.0 => v,
        _ => return "n/a".to_string(),
    };

    let minutes = (ms / 60_000.0).round().max(1.0) as u64;
    if minutes < 60 {
        return format!("{}m", minutes);
    }

    let hours = minutes / 60;
    let minutes_rem = minutes % 60;
    if hours < 24 {
        return if minutes_rem > 0 {
            format!("{}h{}m", hours, minutes_rem)
        } else {
            format!("{}h", hours)
        };
    }

    let days = hours / 24;
    let hours_rem = hours % 24;
    if hours_rem > 0 {
        format!("{}d{}h", days, hours_rem)
    } else {
        format!("{}d", days)
    }
}

/// Format a token count to short form (e.g., "1.5k", "2m").
pub fn format_token_short(value: Option<f64>) -> Option<String> {
    let v = match value {
        Some(v) if v.is_finite() && v > 0.0 => v,
        _ => return None,
    };
    let n = v.floor() as u64;
    if n < 1_000 {
        return Some(format!("{}", n));
    }
    if n < 10_000 {
        let formatted = format!("{:.1}", n as f64 / 1_000.0);
        let formatted = formatted.trim_end_matches(".0");
        return Some(format!("{}k", formatted));
    }
    if n < 1_000_000 {
        return Some(format!("{}k", (n as f64 / 1_000.0).round() as u64));
    }
    let formatted = format!("{:.1}", n as f64 / 1_000_000.0);
    let formatted = formatted.trim_end_matches(".0");
    Some(format!("{}m", formatted))
}

/// Truncate a line to max_length with ellipsis.
pub fn truncate_line(value: &str, max_length: usize) -> String {
    if value.len() <= max_length {
        return value.to_string();
    }
    format!("{}...", value[..max_length].trim_end())
}

/// Token usage data for formatting.
#[derive(Debug, Clone)]
pub struct TokenUsageLike {
    pub total_tokens: Option<f64>,
    pub input_tokens: Option<f64>,
    pub output_tokens: Option<f64>,
}

/// Resolve total tokens from usage data.
pub fn resolve_total_tokens(entry: Option<&TokenUsageLike>) -> Option<f64> {
    let entry = entry?;
    if let Some(total) = entry.total_tokens {
        if total.is_finite() {
            return Some(total);
        }
    }
    let input = entry.input_tokens.unwrap_or(0.0);
    let output = entry.output_tokens.unwrap_or(0.0);
    let total = input + output;
    if total > 0.0 { Some(total) } else { None }
}

/// Resolve input/output token breakdown.
pub fn resolve_io_tokens(
    entry: Option<&TokenUsageLike>,
) -> Option<(f64, f64, f64)> {
    let entry = entry?;
    let input = entry
        .input_tokens
        .filter(|v| v.is_finite())
        .unwrap_or(0.0);
    let output = entry
        .output_tokens
        .filter(|v| v.is_finite())
        .unwrap_or(0.0);
    let total = input + output;
    if total <= 0.0 {
        return None;
    }
    Some((input, output, total))
}

/// Format token usage for display.
pub fn format_token_usage_display(entry: Option<&TokenUsageLike>) -> String {
    let io = resolve_io_tokens(entry);
    let prompt_cache = resolve_total_tokens(entry);
    let mut parts = Vec::new();

    if let Some((input, output, total)) = io {
        let input_str = format_token_short(Some(input)).unwrap_or_else(|| "0".into());
        let output_str = format_token_short(Some(output)).unwrap_or_else(|| "0".into());
        let total_str = format_token_short(Some(total)).unwrap_or_else(|| "0".into());
        parts.push(format!(
            "tokens {} (in {} / out {})",
            total_str, input_str, output_str
        ));
    } else if let Some(pc) = prompt_cache {
        if pc > 0.0 {
            let pc_str = format_token_short(Some(pc)).unwrap_or_else(|| "0".into());
            parts.push(format!("tokens {} prompt/cache", pc_str));
        }
    }

    if let (Some(pc), Some((_, _, total))) = (prompt_cache, io) {
        if pc > total {
            let pc_str = format_token_short(Some(pc)).unwrap_or_else(|| "0".into());
            parts.push(format!("prompt/cache {}", pc_str));
        }
    }

    parts.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_minutes() {
        assert_eq!(format_duration_compact(Some(300_000.0)), "5m");
    }

    #[test]
    fn duration_hours() {
        assert_eq!(format_duration_compact(Some(7_200_000.0)), "2h");
    }

    #[test]
    fn duration_hours_minutes() {
        assert_eq!(format_duration_compact(Some(5_400_000.0)), "1h30m");
    }

    #[test]
    fn duration_invalid() {
        assert_eq!(format_duration_compact(None), "n/a");
        assert_eq!(format_duration_compact(Some(0.0)), "n/a");
        assert_eq!(format_duration_compact(Some(-100.0)), "n/a");
    }

    #[test]
    fn token_short_values() {
        assert_eq!(format_token_short(Some(500.0)), Some("500".into()));
        assert_eq!(format_token_short(Some(1500.0)), Some("1.5k".into()));
        assert_eq!(format_token_short(Some(50000.0)), Some("50k".into()));
        assert_eq!(format_token_short(Some(1500000.0)), Some("1.5m".into()));
        assert_eq!(format_token_short(None), None);
        assert_eq!(format_token_short(Some(0.0)), None);
    }

    #[test]
    fn truncate() {
        assert_eq!(truncate_line("hello world", 20), "hello world");
        assert_eq!(truncate_line("hello world", 5), "hello...");
    }

    #[test]
    fn format_usage() {
        let usage = TokenUsageLike {
            total_tokens: None,
            input_tokens: Some(1000.0),
            output_tokens: Some(500.0),
        };
        let display = format_token_usage_display(Some(&usage));
        assert!(display.contains("tokens"));
        assert!(display.contains("in"));
        assert!(display.contains("out"));
    }
}
