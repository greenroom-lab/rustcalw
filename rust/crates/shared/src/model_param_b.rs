//! Model parameter B extraction — mirrors src/shared/model-param-b.ts

use regex::Regex;

/// Infer the "B" parameter (billions) from a model ID or name.
/// Returns the largest B value found, or None.
pub fn infer_param_b_from_id_or_name(text: &str) -> Option<f64> {
    let raw = text.to_lowercase();
    let re = Regex::new(r"(?:^|[^a-z0-9])[a-z]?(\d+(?:\.\d+)?)b(?:[^a-z0-9]|$)").unwrap();

    let mut best: Option<f64> = None;
    for caps in re.captures_iter(&raw) {
        if let Some(m) = caps.get(1) {
            if let Ok(value) = m.as_str().parse::<f64>() {
                if value.is_finite() && value > 0.0 {
                    best = Some(best.map_or(value, |b: f64| b.max(value)));
                }
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_simple_b() {
        assert_eq!(infer_param_b_from_id_or_name("qwen-3.5b"), Some(3.5));
    }

    #[test]
    fn extracts_largest_b() {
        assert_eq!(
            infer_param_b_from_id_or_name("model-7b-instruct-32b"),
            Some(32.0)
        );
    }

    #[test]
    fn no_b_param() {
        assert_eq!(infer_param_b_from_id_or_name("gpt-4-turbo"), None);
    }

    #[test]
    fn integer_b() {
        assert_eq!(infer_param_b_from_id_or_name("llama-70b"), Some(70.0));
    }
}
