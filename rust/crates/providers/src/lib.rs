//! Provider modules — mirrors src/providers/
//!
//! Contains OAuth flows, model definitions, and shared constants for
//! supported LLM providers (GitHub Copilot, Qwen/DashScope, Kilocode).

pub mod github_copilot_auth;
pub mod github_copilot_models;
pub mod kilocode_shared;
pub mod oauth_types;
pub mod qwen_portal_oauth;
