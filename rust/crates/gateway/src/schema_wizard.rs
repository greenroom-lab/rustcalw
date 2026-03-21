use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// WizardRunStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WizardRunStatus {
    Running,
    Done,
    Cancelled,
    Error,
}

// ---------------------------------------------------------------------------
// Wizard method params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WizardStartParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>, // "local" | "remote"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WizardAnswer {
    pub step_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WizardNextParams {
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<WizardAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WizardCancelParams {
    pub session_id: String,
}

pub type WizardStatusParams = WizardCancelParams;

// ---------------------------------------------------------------------------
// WizardStep
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WizardStepOption {
    pub value: Value,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WizardStep {
    pub id: String,
    #[serde(rename = "type")]
    pub step_type: String, // "note" | "select" | "text" | "confirm" | "multiselect" | "progress" | "action"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<WizardStepOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executor: Option<String>, // "gateway" | "client"
}

// ---------------------------------------------------------------------------
// Wizard results
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WizardNextResult {
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<WizardStep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WizardRunStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WizardStartResult {
    pub session_id: String,
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<WizardStep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WizardRunStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WizardStatusResult {
    pub status: WizardRunStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn wizard_run_status_serde() {
        let status = WizardRunStatus::Running;
        let json_str = serde_json::to_string(&status).unwrap();
        assert_eq!(json_str, "\"running\"");
    }

    #[test]
    fn wizard_step_serde() {
        let step = WizardStep {
            id: "step-1".to_string(),
            step_type: "select".to_string(),
            title: Some("Choose provider".to_string()),
            message: None,
            options: Some(vec![
                WizardStepOption {
                    value: json!("openai"),
                    label: "OpenAI".to_string(),
                    hint: None,
                },
                WizardStepOption {
                    value: json!("anthropic"),
                    label: "Anthropic".to_string(),
                    hint: Some("Recommended".to_string()),
                },
            ]),
            initial_value: None,
            placeholder: None,
            sensitive: None,
            executor: None,
        };
        let json_val = serde_json::to_value(&step).unwrap();
        assert_eq!(json_val["type"], "select");
        assert_eq!(json_val["options"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn wizard_start_result_roundtrip() {
        let result = WizardStartResult {
            session_id: "wiz-1".to_string(),
            done: false,
            step: Some(WizardStep {
                id: "step-1".to_string(),
                step_type: "note".to_string(),
                title: Some("Welcome".to_string()),
                message: Some("Let's get started".to_string()),
                options: None,
                initial_value: None,
                placeholder: None,
                sensitive: None,
                executor: None,
            }),
            status: Some(WizardRunStatus::Running),
            error: None,
        };
        let json_str = serde_json::to_string(&result).unwrap();
        let parsed: WizardStartResult = serde_json::from_str(&json_str).unwrap();
        assert_eq!(result, parsed);
    }
}
