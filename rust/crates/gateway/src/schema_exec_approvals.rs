use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// ExecApprovals allowlist & policy
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecApprovalsAllowlistEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resolved_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecApprovalsDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_fallback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_allow_skills: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecApprovalsAgent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_fallback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_allow_skills: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowlist: Option<Vec<ExecApprovalsAllowlistEntry>>,
}

// ---------------------------------------------------------------------------
// ExecApprovals file schema
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecApprovalsSocket {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecApprovalsFile {
    pub version: u64, // always 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<ExecApprovalsSocket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<ExecApprovalsDefaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<HashMap<String, ExecApprovalsAgent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecApprovalsSnapshot {
    pub path: String,
    pub exists: bool,
    pub hash: String,
    pub file: ExecApprovalsFile,
}

// ---------------------------------------------------------------------------
// ExecApprovals method params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecApprovalsGetParams {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecApprovalsSetParams {
    pub file: ExecApprovalsFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecApprovalsNodeGetParams {
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecApprovalsNodeSetParams {
    pub node_id: String,
    pub file: ExecApprovalsFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_hash: Option<String>,
}

// ---------------------------------------------------------------------------
// ExecApproval request/resolve
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SystemRunPlanMutableFileOperand {
    pub argv_index: u64,
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SystemRunPlan {
    pub argv: Vec<String>,
    pub cwd: Option<String>,
    pub command_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_preview: Option<Option<String>>,
    pub agent_id: Option<String>,
    pub session_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutable_file_operand: Option<Option<SystemRunPlanMutableFileOperand>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecApprovalRequestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_argv: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_run_plan: Option<SystemRunPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_path: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_source_channel: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_source_to: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_source_account_id: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_source_thread_id: Option<Option<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_phase: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecApprovalResolveParams {
    pub id: String,
    pub decision: String,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exec_approvals_file_roundtrip() {
        let file = ExecApprovalsFile {
            version: 1,
            socket: None,
            defaults: Some(ExecApprovalsDefaults {
                security: Some("sandbox".to_string()),
                ask: Some("auto".to_string()),
                ask_fallback: None,
                auto_allow_skills: Some(true),
            }),
            agents: None,
        };
        let json_str = serde_json::to_string(&file).unwrap();
        let parsed: ExecApprovalsFile = serde_json::from_str(&json_str).unwrap();
        assert_eq!(file, parsed);
    }

    #[test]
    fn exec_approvals_snapshot_serde() {
        let snap = ExecApprovalsSnapshot {
            path: "/home/user/.openclaw/exec-approvals.json".to_string(),
            exists: true,
            hash: "abc123".to_string(),
            file: ExecApprovalsFile {
                version: 1,
                socket: None,
                defaults: None,
                agents: Some({
                    let mut m = HashMap::new();
                    m.insert(
                        "main".to_string(),
                        ExecApprovalsAgent {
                            security: None,
                            ask: None,
                            ask_fallback: None,
                            auto_allow_skills: None,
                            allowlist: Some(vec![ExecApprovalsAllowlistEntry {
                                id: Some("e-1".to_string()),
                                pattern: "npm *".to_string(),
                                last_used_at: None,
                                last_used_command: None,
                                last_resolved_path: None,
                            }]),
                        },
                    );
                    m
                }),
            },
        };
        let json_val = serde_json::to_value(&snap).unwrap();
        assert_eq!(json_val["exists"], true);
    }

    #[test]
    fn exec_approval_request_params_serde() {
        let params = ExecApprovalRequestParams {
            id: Some("req-1".to_string()),
            command: Some("npm install".to_string()),
            command_argv: Some(vec!["npm".to_string(), "install".to_string()]),
            system_run_plan: None,
            env: None,
            cwd: Some(Some("/home/user/project".to_string())),
            node_id: None,
            host: None,
            security: None,
            ask: None,
            agent_id: None,
            resolved_path: None,
            session_key: None,
            turn_source_channel: None,
            turn_source_to: None,
            turn_source_account_id: None,
            turn_source_thread_id: None,
            timeout_ms: Some(30000),
            two_phase: None,
        };
        let json_val = serde_json::to_value(&params).unwrap();
        assert_eq!(json_val["command"], "npm install");
    }
}
