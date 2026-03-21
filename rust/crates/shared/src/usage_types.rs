//! Usage types — mirrors src/shared/usage-types.ts

use serde::{Deserialize, Serialize};

/// Single point in a session usage time series.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUsageTimePoint {
    pub timestamp: u64,
    pub input: u64,
    pub output: u64,
    pub cache_read: u64,
    pub cache_write: u64,
    pub total_tokens: u64,
    pub cost: f64,
    pub cumulative_tokens: u64,
    pub cumulative_cost: f64,
}

/// Session usage time series.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUsageTimeSeries {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub points: Vec<SessionUsageTimePoint>,
}

/// Origin metadata for a session usage entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUsageOrigin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<serde_json::Value>,
}

/// Single session usage entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUsageEntry {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<SessionUsageOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_override: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_override: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub usage: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_weight: Option<serde_json::Value>,
}

/// Daily usage breakdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyUsage {
    pub date: String,
    pub tokens: u64,
    pub cost: f64,
    pub messages: u64,
    pub tool_calls: u64,
    pub errors: u64,
}

/// Aggregated usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionsUsageAggregates {
    pub messages: serde_json::Value,
    pub tools: serde_json::Value,
    pub by_model: Vec<serde_json::Value>,
    pub by_provider: Vec<serde_json::Value>,
    pub by_agent: Vec<serde_json::Value>,
    pub by_channel: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_latency: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_daily: Option<Vec<serde_json::Value>>,
    pub daily: Vec<DailyUsage>,
}

/// Complete usage result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionsUsageResult {
    pub updated_at: u64,
    pub start_date: String,
    pub end_date: String,
    pub sessions: Vec<SessionUsageEntry>,
    pub totals: serde_json::Value,
    pub aggregates: SessionsUsageAggregates,
}
