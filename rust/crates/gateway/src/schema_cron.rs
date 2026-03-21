use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Cron enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CronRunStatus {
    Ok,
    Error,
    Skipped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CronSortDir {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CronJobsEnabledFilter {
    All,
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CronJobsSortBy {
    NextRunAtMs,
    UpdatedAtMs,
    Name,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CronDeliveryStatus {
    Delivered,
    NotDelivered,
    Unknown,
    NotRequested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CronFailoverReason {
    Auth,
    Format,
    RateLimit,
    Billing,
    Timeout,
    ModelNotFound,
    Unknown,
}

// ---------------------------------------------------------------------------
// CronSchedule
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum CronSchedule {
    At {
        at: String,
    },
    Every {
        #[serde(rename = "everyMs")]
        every_ms: u64,
        #[serde(rename = "anchorMs", skip_serializing_if = "Option::is_none")]
        anchor_ms: Option<u64>,
    },
    Cron {
        expr: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        tz: Option<String>,
        #[serde(rename = "staggerMs", skip_serializing_if = "Option::is_none")]
        stagger_ms: Option<u64>,
    },
}

// ---------------------------------------------------------------------------
// CronPayload
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CronPayload {
    SystemEvent {
        text: String,
    },
    AgentTurn {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fallbacks: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thinking: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        timeout_seconds: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        allow_unsafe_external_content: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        light_context: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        deliver: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        channel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        to: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        best_effort_deliver: Option<bool>,
    },
}

/// Patch version of CronPayload where `message` is optional for updates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CronPayloadPatch {
    SystemEvent {
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    AgentTurn {
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fallbacks: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thinking: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        timeout_seconds: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        allow_unsafe_external_content: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        light_context: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        deliver: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        channel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        to: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        best_effort_deliver: Option<bool>,
    },
}

// ---------------------------------------------------------------------------
// CronFailureAlert / CronDelivery
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronFailureAlert {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>, // "last" or channel name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>, // "announce" | "webhook"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronFailureDestination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum CronDelivery {
    None {
        #[serde(skip_serializing_if = "Option::is_none")]
        channel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        account_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        best_effort: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        failure_destination: Option<CronFailureDestination>,
        #[serde(skip_serializing_if = "Option::is_none")]
        to: Option<String>,
    },
    Announce {
        #[serde(skip_serializing_if = "Option::is_none")]
        channel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        account_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        best_effort: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        failure_destination: Option<CronFailureDestination>,
        #[serde(skip_serializing_if = "Option::is_none")]
        to: Option<String>,
    },
    Webhook {
        #[serde(skip_serializing_if = "Option::is_none")]
        channel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        account_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        best_effort: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        failure_destination: Option<CronFailureDestination>,
        to: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronDeliveryPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_effort: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_destination: Option<CronFailureDestination>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

// ---------------------------------------------------------------------------
// CronJobState
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronJobState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_run_at_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_at_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_at_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_status: Option<CronRunStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<CronRunStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_reason: Option<CronFailoverReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_duration_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consecutive_errors: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_delivered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_delivery_status: Option<CronDeliveryStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_delivery_error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_alert_at_ms: Option<u64>,
}

// ---------------------------------------------------------------------------
// CronJob
// ---------------------------------------------------------------------------

/// Wrapper to allow `failureAlert: false` as a way to disable it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CronFailureAlertOption {
    Disabled(bool), // always false
    Config(CronFailureAlert),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronJob {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_run: Option<bool>,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub schedule: CronSchedule,
    pub session_target: String, // "main" | "isolated" | "current" | "session:..."
    pub wake_mode: String,      // "next-heartbeat" | "now"
    pub payload: CronPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<CronDelivery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_alert: Option<CronFailureAlertOption>,
    pub state: CronJobState,
}

// ---------------------------------------------------------------------------
// Cron method params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<CronJobsEnabledFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<CronJobsSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<CronSortDir>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CronStatusParams {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronAddParams {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_run: Option<bool>,
    pub schedule: CronSchedule,
    pub session_target: String,
    pub wake_mode: String,
    pub payload: CronPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<CronDelivery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_alert: Option<CronFailureAlertOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronJobPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CronSchedule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wake_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<CronPayloadPatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<CronDeliveryPatch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_alert: Option<CronFailureAlertOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Value>, // Partial<CronJobState>
}

/// `cron.update` accepts either `{id, patch}` or `{jobId, patch}`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    pub patch: CronJobPatch,
}

/// `cron.remove` accepts either `{id}` or `{jobId}`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronRemoveParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// `cron.run` accepts either `{id, mode?}` or `{jobId, mode?}`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronRunParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>, // "due" | "force"
}

// ---------------------------------------------------------------------------
// cron.runs (run log)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronRunsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>, // "job" | "all"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<CronRunStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_statuses: Option<Vec<CronDeliveryStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<CronDeliveryStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<CronSortDir>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronRunLogUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_read_tokens: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write_tokens: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CronRunLogEntry {
    pub ts: u64,
    pub job_id: String,
    pub action: String, // "finished"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CronRunStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<CronDeliveryStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_at_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_run_at_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<CronRunLogUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cron_schedule_at() {
        let schedule = CronSchedule::At {
            at: "2024-03-10T09:00:00Z".to_string(),
        };
        let json_val = serde_json::to_value(&schedule).unwrap();
        assert_eq!(json_val["kind"], "at");
        assert_eq!(json_val["at"], "2024-03-10T09:00:00Z");
    }

    #[test]
    fn cron_schedule_cron() {
        let schedule = CronSchedule::Cron {
            expr: "0 9 * * *".to_string(),
            tz: Some("America/New_York".to_string()),
            stagger_ms: None,
        };
        let json_val = serde_json::to_value(&schedule).unwrap();
        assert_eq!(json_val["kind"], "cron");
        assert_eq!(json_val["expr"], "0 9 * * *");
    }

    #[test]
    fn cron_payload_system_event() {
        let payload = CronPayload::SystemEvent {
            text: "heartbeat".to_string(),
        };
        let json_val = serde_json::to_value(&payload).unwrap();
        assert_eq!(json_val["kind"], "systemEvent");
        assert_eq!(json_val["text"], "heartbeat");
    }

    #[test]
    fn cron_payload_agent_turn() {
        let payload = CronPayload::AgentTurn {
            message: "Check status".to_string(),
            model: Some("gpt-4".to_string()),
            fallbacks: None,
            thinking: None,
            timeout_seconds: Some(120),
            allow_unsafe_external_content: None,
            light_context: None,
            deliver: Some(true),
            channel: None,
            to: None,
            best_effort_deliver: None,
        };
        let json_val = serde_json::to_value(&payload).unwrap();
        assert_eq!(json_val["kind"], "agentTurn");
        assert_eq!(json_val["message"], "Check status");
    }

    #[test]
    fn cron_delivery_webhook() {
        let delivery = CronDelivery::Webhook {
            channel: Some("telegram".to_string()),
            account_id: None,
            best_effort: None,
            failure_destination: None,
            to: "https://hooks.example.com".to_string(),
        };
        let json_val = serde_json::to_value(&delivery).unwrap();
        assert_eq!(json_val["mode"], "webhook");
        assert_eq!(json_val["to"], "https://hooks.example.com");
    }

    #[test]
    fn cron_run_log_entry_serde() {
        let entry = CronRunLogEntry {
            ts: 1710000000,
            job_id: "job-1".to_string(),
            action: "finished".to_string(),
            status: Some(CronRunStatus::Ok),
            error: None,
            summary: Some("Done".to_string()),
            delivered: Some(true),
            delivery_status: Some(CronDeliveryStatus::Delivered),
            delivery_error: None,
            session_id: None,
            session_key: None,
            run_at_ms: Some(1709999000),
            duration_ms: Some(5000),
            next_run_at_ms: Some(1710003600),
            model: None,
            provider: None,
            usage: Some(CronRunLogUsage {
                input_tokens: Some(100.0),
                output_tokens: Some(50.0),
                total_tokens: Some(150.0),
                cache_read_tokens: None,
                cache_write_tokens: None,
            }),
            job_name: Some("daily-check".to_string()),
        };
        let json_str = serde_json::to_string(&entry).unwrap();
        let parsed: CronRunLogEntry = serde_json::from_str(&json_str).unwrap();
        assert_eq!(entry, parsed);
    }
}
