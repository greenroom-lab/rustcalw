use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Push notification test
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApnsEnvironment {
    Sandbox,
    Production,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PushTransport {
    Direct,
    Relay,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PushTestParams {
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ApnsEnvironment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PushTestResult {
    pub ok: bool,
    pub status: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub token_suffix: String,
    pub topic: String,
    pub environment: ApnsEnvironment,
    pub transport: PushTransport,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_test_params_roundtrip() {
        let params = PushTestParams {
            node_id: "node-1".to_string(),
            title: Some("Hello".to_string()),
            body: None,
            environment: Some(ApnsEnvironment::Sandbox),
        };
        let json_str = serde_json::to_string(&params).unwrap();
        let parsed: PushTestParams = serde_json::from_str(&json_str).unwrap();
        assert_eq!(params, parsed);
    }

    #[test]
    fn push_test_result_serde() {
        let result = PushTestResult {
            ok: true,
            status: 200,
            apns_id: Some("uuid-123".to_string()),
            reason: None,
            token_suffix: "...abc".to_string(),
            topic: "com.app.push".to_string(),
            environment: ApnsEnvironment::Production,
            transport: PushTransport::Direct,
        };
        let json_val = serde_json::to_value(&result).unwrap();
        assert_eq!(json_val["ok"], true);
        assert_eq!(json_val["environment"], "production");
        assert_eq!(json_val["transport"], "direct");
    }
}
