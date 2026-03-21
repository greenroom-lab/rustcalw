use serde::{Deserialize, Serialize};
// ---------------------------------------------------------------------------
// ModelChoice
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModelChoice {
    pub id: String,
    pub name: String,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_window: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<bool>,
}

// ---------------------------------------------------------------------------
// AgentSummary / AgentIdentity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentIdentityInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentSummary {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<AgentIdentityInfo>,
}

// ---------------------------------------------------------------------------
// agents.list
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentsListParams {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsListResult {
    pub default_id: String,
    pub main_key: String,
    pub scope: String, // "per-sender" | "global"
    pub agents: Vec<AgentSummary>,
}

// ---------------------------------------------------------------------------
// agents.create
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsCreateParams {
    pub name: String,
    pub workspace: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsCreateResult {
    pub ok: bool,
    pub agent_id: String,
    pub name: String,
    pub workspace: String,
}

// ---------------------------------------------------------------------------
// agents.update
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsUpdateParams {
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsUpdateResult {
    pub ok: bool,
    pub agent_id: String,
}

// ---------------------------------------------------------------------------
// agents.delete
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsDeleteParams {
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_files: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsDeleteResult {
    pub ok: bool,
    pub agent_id: String,
    pub removed_bindings: u64,
}

// ---------------------------------------------------------------------------
// agents.files
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsFileEntry {
    pub name: String,
    pub path: String,
    pub missing: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsFilesListParams {
    pub agent_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsFilesListResult {
    pub agent_id: String,
    pub workspace: String,
    pub files: Vec<AgentsFileEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsFilesGetParams {
    pub agent_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsFilesGetResult {
    pub agent_id: String,
    pub workspace: String,
    pub file: AgentsFileEntry,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsFilesSetParams {
    pub agent_id: String,
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentsFilesSetResult {
    pub ok: bool,
    pub agent_id: String,
    pub workspace: String,
    pub file: AgentsFileEntry,
}

// ---------------------------------------------------------------------------
// models.list
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelsListParams {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModelsListResult {
    pub models: Vec<ModelChoice>,
}

// ---------------------------------------------------------------------------
// skills
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SkillsStatusParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillsBinsParams {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SkillsBinsResult {
    pub bins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SkillsInstallParams {
    pub name: String,
    pub install_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SkillsUpdateParams {
    pub skill_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::HashMap<String, String>>,
}

// ---------------------------------------------------------------------------
// tools.catalog
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ToolsCatalogParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_plugins: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolProfileId {
    Minimal,
    Coding,
    Messaging,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ToolCatalogProfile {
    pub id: ToolProfileId,
    pub label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolSource {
    Core,
    Plugin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ToolCatalogEntry {
    pub id: String,
    pub label: String,
    pub description: String,
    pub source: ToolSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    pub default_profiles: Vec<ToolProfileId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ToolCatalogGroup {
    pub id: String,
    pub label: String,
    pub source: ToolSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    pub tools: Vec<ToolCatalogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ToolsCatalogResult {
    pub agent_id: String,
    pub profiles: Vec<ToolCatalogProfile>,
    pub groups: Vec<ToolCatalogGroup>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn model_choice_roundtrip() {
        let mc = ModelChoice {
            id: "gpt-4".to_string(),
            name: "GPT-4".to_string(),
            provider: "openai".to_string(),
            context_window: Some(128_000),
            reasoning: Some(false),
        };
        let json_str = serde_json::to_string(&mc).unwrap();
        let parsed: ModelChoice = serde_json::from_str(&json_str).unwrap();
        assert_eq!(mc, parsed);
    }

    #[test]
    fn agent_summary_serde() {
        let summary = AgentSummary {
            id: "main".to_string(),
            name: Some("Main Agent".to_string()),
            identity: Some(AgentIdentityInfo {
                name: Some("Main".to_string()),
                theme: None,
                emoji: Some("🤖".to_string()),
                avatar: None,
                avatar_url: None,
            }),
        };
        let json_val = serde_json::to_value(&summary).unwrap();
        assert_eq!(json_val["id"], "main");
        assert_eq!(json_val["identity"]["emoji"], "🤖");
    }

    #[test]
    fn agents_list_result_serde() {
        let result = AgentsListResult {
            default_id: "main".to_string(),
            main_key: "default".to_string(),
            scope: "per-sender".to_string(),
            agents: vec![AgentSummary {
                id: "main".to_string(),
                name: None,
                identity: None,
            }],
        };
        let json_val = serde_json::to_value(&result).unwrap();
        assert_eq!(json_val["defaultId"], "main");
        assert_eq!(json_val["agents"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn tool_profile_id_serde() {
        let id = ToolProfileId::Coding;
        let json_str = serde_json::to_string(&id).unwrap();
        assert_eq!(json_str, "\"coding\"");
    }

    #[test]
    fn tool_catalog_entry_serde() {
        let entry = ToolCatalogEntry {
            id: "web-search".to_string(),
            label: "Web Search".to_string(),
            description: "Search the web".to_string(),
            source: ToolSource::Core,
            plugin_id: None,
            optional: Some(true),
            default_profiles: vec![ToolProfileId::Full, ToolProfileId::Coding],
        };
        let json_val = serde_json::to_value(&entry).unwrap();
        assert_eq!(json_val["source"], "core");
        assert_eq!(json_val["defaultProfiles"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn agents_file_entry_serde() {
        let entry = AgentsFileEntry {
            name: "prompt.md".to_string(),
            path: "/agents/main/prompt.md".to_string(),
            missing: false,
            size: Some(1024),
            updated_at_ms: Some(1710000000000),
            content: Some("Hello world".to_string()),
        };
        let json_val = serde_json::to_value(&entry).unwrap();
        assert_eq!(json_val["name"], "prompt.md");
        assert_eq!(json_val["missing"], false);
    }
}
