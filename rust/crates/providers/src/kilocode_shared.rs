//! Kilocode shared constants and types — mirrors src/providers/kilocode-shared.ts

use rustcalw_config::types::models::{ModelCost, ModelDefinitionConfig};
use serde::{Deserialize, Serialize};

pub const KILOCODE_BASE_URL: &str = "https://api.kilo.ai/api/gateway/";
pub const KILOCODE_DEFAULT_MODEL_ID: &str = "kilo/auto";
pub const KILOCODE_DEFAULT_MODEL_NAME: &str = "Kilo Auto";
pub const KILOCODE_DEFAULT_CONTEXT_WINDOW: u64 = 1_000_000;
pub const KILOCODE_DEFAULT_MAX_TOKENS: u64 = 128_000;

pub const KILOCODE_DEFAULT_COST: ModelCost = ModelCost {
    input: 0.0,
    output: 0.0,
    cache_read: 0.0,
    cache_write: 0.0,
};

/// A catalog entry for a Kilocode model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KilocodeModelCatalogEntry {
    pub id: String,
    pub name: String,
    pub reasoning: bool,
    pub input: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_window: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
}

/// Default model reference string: `kilocode/{KILOCODE_DEFAULT_MODEL_ID}`.
pub fn kilocode_default_model_ref() -> String {
    format!("kilocode/{KILOCODE_DEFAULT_MODEL_ID}")
}

/// Static fallback catalog — used by the sync setup path and as a
/// fallback when dynamic model discovery from the gateway API fails.
pub fn kilocode_model_catalog() -> Vec<KilocodeModelCatalogEntry> {
    vec![KilocodeModelCatalogEntry {
        id: KILOCODE_DEFAULT_MODEL_ID.to_string(),
        name: KILOCODE_DEFAULT_MODEL_NAME.to_string(),
        reasoning: true,
        input: vec!["text".to_string(), "image".to_string()],
        context_window: Some(KILOCODE_DEFAULT_CONTEXT_WINDOW),
        max_tokens: Some(KILOCODE_DEFAULT_MAX_TOKENS),
    }]
}

/// Build a `ModelDefinitionConfig` from a catalog entry, applying Kilocode defaults.
pub fn build_kilocode_model_definition(entry: &KilocodeModelCatalogEntry) -> ModelDefinitionConfig {
    ModelDefinitionConfig {
        id: entry.id.clone(),
        name: entry.name.clone(),
        api: Some("openai-responses".to_string()),
        reasoning: entry.reasoning,
        input: entry.input.clone(),
        cost: Some(KILOCODE_DEFAULT_COST),
        context_window: entry
            .context_window
            .unwrap_or(KILOCODE_DEFAULT_CONTEXT_WINDOW),
        max_tokens: entry.max_tokens.unwrap_or(KILOCODE_DEFAULT_MAX_TOKENS),
        headers: None,
        compat: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_ref() {
        assert_eq!(kilocode_default_model_ref(), "kilocode/kilo/auto");
    }

    #[test]
    fn catalog_has_default_entry() {
        let catalog = kilocode_model_catalog();
        assert_eq!(catalog.len(), 1);
        assert_eq!(catalog[0].id, KILOCODE_DEFAULT_MODEL_ID);
        assert!(catalog[0].reasoning);
    }

    #[test]
    fn build_definition_from_catalog() {
        let catalog = kilocode_model_catalog();
        let def = build_kilocode_model_definition(&catalog[0]);
        assert_eq!(def.id, "kilo/auto");
        assert_eq!(def.name, "Kilo Auto");
        assert_eq!(def.api.as_deref(), Some("openai-responses"));
        assert_eq!(def.context_window, 1_000_000);
        assert_eq!(def.max_tokens, 128_000);
        assert!(def.reasoning);
        let cost = def.cost.unwrap();
        assert_eq!(cost.input, 0.0);
    }
}
