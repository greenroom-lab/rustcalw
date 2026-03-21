//! GitHub Copilot model definitions — mirrors src/providers/github-copilot-models.ts

use rustcalw_config::types::models::{ModelCost, ModelDefinitionConfig};

const DEFAULT_CONTEXT_WINDOW: u64 = 128_000;
const DEFAULT_MAX_TOKENS: u64 = 8_192;

/// Copilot model ids vary by plan/org and can change.
/// We keep this list intentionally broad; if a model isn't available Copilot will
/// return an error and users can remove it from their config.
const DEFAULT_MODEL_IDS: &[&str] = &[
    "claude-sonnet-4.6",
    "claude-sonnet-4.5",
    "gpt-4o",
    "gpt-4.1",
    "gpt-4.1-mini",
    "gpt-4.1-nano",
    "o1",
    "o1-mini",
    "o3-mini",
];

/// Returns the default list of Copilot model IDs.
pub fn get_default_copilot_model_ids() -> Vec<String> {
    DEFAULT_MODEL_IDS.iter().map(|s| s.to_string()).collect()
}

/// Build a `ModelDefinitionConfig` for a Copilot model.
///
/// Uses OpenAI-compatible responses API while keeping the provider id as
/// "github-copilot" (pi-ai uses that to attach Copilot-specific headers).
pub fn build_copilot_model_definition(model_id: &str) -> anyhow::Result<ModelDefinitionConfig> {
    let id = model_id.trim();
    if id.is_empty() {
        anyhow::bail!("Model id required");
    }
    Ok(ModelDefinitionConfig {
        id: id.to_string(),
        name: id.to_string(),
        api: Some("openai-responses".to_string()),
        reasoning: false,
        input: vec!["text".to_string(), "image".to_string()],
        cost: Some(ModelCost {
            input: 0.0,
            output: 0.0,
            cache_read: 0.0,
            cache_write: 0.0,
        }),
        context_window: DEFAULT_CONTEXT_WINDOW,
        max_tokens: DEFAULT_MAX_TOKENS,
        headers: None,
        compat: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_ids_non_empty() {
        let ids = get_default_copilot_model_ids();
        assert!(!ids.is_empty());
        assert!(ids.contains(&"gpt-4o".to_string()));
        assert!(ids.contains(&"claude-sonnet-4.6".to_string()));
    }

    #[test]
    fn build_model_basic() {
        let def = build_copilot_model_definition("gpt-4o").unwrap();
        assert_eq!(def.id, "gpt-4o");
        assert_eq!(def.name, "gpt-4o");
        assert_eq!(def.api.as_deref(), Some("openai-responses"));
        assert!(!def.reasoning);
        assert_eq!(def.context_window, 128_000);
        assert_eq!(def.max_tokens, 8_192);
        assert_eq!(def.input, vec!["text", "image"]);
    }

    #[test]
    fn build_model_trims_whitespace() {
        let def = build_copilot_model_definition("  o1-mini  ").unwrap();
        assert_eq!(def.id, "o1-mini");
    }

    #[test]
    fn build_model_empty_errors() {
        assert!(build_copilot_model_definition("").is_err());
        assert!(build_copilot_model_definition("   ").is_err());
    }

    #[test]
    fn build_model_zero_cost() {
        let def = build_copilot_model_definition("gpt-4o").unwrap();
        let cost = def.cost.unwrap();
        assert_eq!(cost.input, 0.0);
        assert_eq!(cost.output, 0.0);
        assert_eq!(cost.cache_read, 0.0);
        assert_eq!(cost.cache_write, 0.0);
    }
}
