use serde::{Deserialize, Serialize};
// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

pub const CHAT_SEND_SESSION_KEY_MAX_LENGTH: usize = 512;
pub const SESSION_LABEL_MAX_LENGTH: usize = 512;

/// Regex patterns for validation (informational — actual validation is at
/// the protocol boundary).
pub const SECRET_PROVIDER_ALIAS_PATTERN: &str = r"^[a-z][a-z0-9_-]{0,63}$";
pub const ENV_SECRET_REF_ID_PATTERN: &str = r"^[A-Z][A-Z0-9_]{0,127}$";

// ---------------------------------------------------------------------------
// InputProvenance
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputProvenanceKind {
    ExternalUser,
    InterSession,
    InternalSystem,
}

pub const INPUT_PROVENANCE_KIND_VALUES: &[InputProvenanceKind] = &[
    InputProvenanceKind::ExternalUser,
    InputProvenanceKind::InterSession,
    InputProvenanceKind::InternalSystem,
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InputProvenance {
    pub kind: InputProvenanceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_session_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_tool: Option<String>,
}

// ---------------------------------------------------------------------------
// SecretRef
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SecretRefSource {
    Env,
    File,
    Exec,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SecretRef {
    pub source: SecretRefSource,
    pub provider: String,
    pub id: String,
}

/// A secret input is either a plain string value or a structured reference.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SecretInput {
    Plain(String),
    Ref(SecretRef),
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn input_provenance_kind_serde() {
        let kind = InputProvenanceKind::ExternalUser;
        let json_str = serde_json::to_string(&kind).unwrap();
        assert_eq!(json_str, "\"external_user\"");
        let parsed: InputProvenanceKind = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed, kind);
    }

    #[test]
    fn input_provenance_roundtrip() {
        let prov = InputProvenance {
            kind: InputProvenanceKind::InterSession,
            origin_session_id: Some("sess-1".to_string()),
            source_session_key: None,
            source_channel: None,
            source_tool: None,
        };
        let json_str = serde_json::to_string(&prov).unwrap();
        let parsed: InputProvenance = serde_json::from_str(&json_str).unwrap();
        assert_eq!(prov, parsed);
    }

    #[test]
    fn secret_ref_source_serde() {
        let src = SecretRefSource::Env;
        let json_str = serde_json::to_string(&src).unwrap();
        assert_eq!(json_str, "\"env\"");
    }

    #[test]
    fn secret_input_plain_string() {
        let input: SecretInput = serde_json::from_value(json!("my-secret")).unwrap();
        match input {
            SecretInput::Plain(s) => assert_eq!(s, "my-secret"),
            _ => panic!("expected Plain"),
        }
    }

    #[test]
    fn secret_input_ref() {
        let input: SecretInput = serde_json::from_value(json!({
            "source": "env",
            "provider": "default",
            "id": "MY_SECRET"
        }))
        .unwrap();
        match input {
            SecretInput::Ref(r) => {
                assert_eq!(r.source, SecretRefSource::Env);
                assert_eq!(r.provider, "default");
                assert_eq!(r.id, "MY_SECRET");
            }
            _ => panic!("expected Ref"),
        }
    }

    #[test]
    fn secret_ref_roundtrip() {
        let secret_ref = SecretRef {
            source: SecretRefSource::File,
            provider: "vault".to_string(),
            id: "value".to_string(),
        };
        let json_str = serde_json::to_string(&secret_ref).unwrap();
        let parsed: SecretRef = serde_json::from_str(&json_str).unwrap();
        assert_eq!(secret_ref, parsed);
    }
}
