//! Config type definitions — mirrors src/config/types.ts and its sub-modules.
//!
//! Organized into focused sub-modules matching the TypeScript structure.

pub mod base;
pub mod secrets;
pub mod models;
pub mod agents;
pub mod gateway;
pub mod messages;
pub mod channels;
pub mod hooks;
pub mod openclaw;

// Re-export the root config type for convenience.
pub use openclaw::OpenClawConfig;
pub use openclaw::ConfigFileSnapshot;
pub use openclaw::ConfigValidationIssue;
