//! Requirement evaluation — mirrors src/shared/requirements.ts

use serde::{Deserialize, Serialize};

/// Binary/env/OS/config requirements for a skill or command.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Requirements {
    #[serde(default)]
    pub bins: Vec<String>,
    #[serde(default, rename = "anyBins")]
    pub any_bins: Vec<String>,
    #[serde(default)]
    pub env: Vec<String>,
    #[serde(default)]
    pub config: Vec<String>,
    #[serde(default)]
    pub os: Vec<String>,
}

impl Requirements {
    pub fn is_empty(&self) -> bool {
        self.bins.is_empty()
            && self.any_bins.is_empty()
            && self.env.is_empty()
            && self.config.is_empty()
            && self.os.is_empty()
    }
}

/// Result of checking a single config path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementConfigCheck {
    pub path: String,
    pub satisfied: bool,
}

/// Metadata attached to skill/plugin manifests.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequirementsMetadata {
    pub requires: Option<PartialRequirements>,
    pub os: Option<Vec<String>>,
}

/// Partial requirements (bins/anyBins/env/config only — no os).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PartialRequirements {
    pub bins: Option<Vec<String>>,
    #[serde(rename = "anyBins")]
    pub any_bins: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub config: Option<Vec<String>>,
}

/// Remote node requirement context.
/// Note: closures are not Clone/Debug, so this struct uses manual impls.
pub struct RequirementRemote {
    pub has_bin: Option<Box<dyn Fn(&str) -> bool + Send + Sync>>,
    pub has_any_bin: Option<Box<dyn Fn(&[String]) -> bool + Send + Sync>>,
    pub platforms: Option<Vec<String>>,
}

impl std::fmt::Debug for RequirementRemote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequirementRemote")
            .field("has_bin", &self.has_bin.is_some())
            .field("has_any_bin", &self.has_any_bin.is_some())
            .field("platforms", &self.platforms)
            .finish()
    }
}

impl Default for RequirementRemote {
    fn default() -> Self {
        Self {
            has_bin: None,
            has_any_bin: None,
            platforms: None,
        }
    }
}

/// Filter bins that are missing locally and remotely.
pub fn resolve_missing_bins(
    required: &[String],
    has_local_bin: &dyn Fn(&str) -> bool,
    has_remote_bin: Option<&dyn Fn(&str) -> bool>,
) -> Vec<String> {
    required
        .iter()
        .filter(|bin| {
            if has_local_bin(bin) {
                return false;
            }
            if let Some(remote) = has_remote_bin {
                if remote(bin) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect()
}

/// Filter anyBins that are missing (at least one must be present).
pub fn resolve_missing_any_bins(
    required: &[String],
    has_local_bin: &dyn Fn(&str) -> bool,
    has_remote_any_bin: Option<&dyn Fn(&[String]) -> bool>,
) -> Vec<String> {
    if required.is_empty() {
        return vec![];
    }
    if required.iter().any(|bin| has_local_bin(bin)) {
        return vec![];
    }
    if let Some(remote) = has_remote_any_bin {
        if remote(required) {
            return vec![];
        }
    }
    required.to_vec()
}

/// Filter OS platforms that are missing.
pub fn resolve_missing_os(
    required: &[String],
    local_platform: &str,
    remote_platforms: Option<&[String]>,
) -> Vec<String> {
    if required.is_empty() {
        return vec![];
    }
    if required.iter().any(|p| p == local_platform) {
        return vec![];
    }
    if let Some(remotes) = remote_platforms {
        if remotes.iter().any(|p| required.contains(p)) {
            return vec![];
        }
    }
    required.to_vec()
}

/// Filter env vars that are missing.
pub fn resolve_missing_env(
    required: &[String],
    is_satisfied: &dyn Fn(&str) -> bool,
) -> Vec<String> {
    required
        .iter()
        .filter(|name| !is_satisfied(name))
        .cloned()
        .collect()
}

/// Build config check results.
pub fn build_config_checks(
    required: &[String],
    is_satisfied: &dyn Fn(&str) -> bool,
) -> Vec<RequirementConfigCheck> {
    required
        .iter()
        .map(|path_str| RequirementConfigCheck {
            path: path_str.clone(),
            satisfied: is_satisfied(path_str),
        })
        .collect()
}

/// Result of evaluating requirements.
#[derive(Debug, Clone)]
pub struct RequirementsEvaluation {
    pub missing: Requirements,
    pub eligible: bool,
    pub config_checks: Vec<RequirementConfigCheck>,
}

/// Evaluate a full set of requirements.
pub fn evaluate_requirements(
    always: bool,
    required: &Requirements,
    has_local_bin: &dyn Fn(&str) -> bool,
    local_platform: &str,
    is_env_satisfied: &dyn Fn(&str) -> bool,
    is_config_satisfied: &dyn Fn(&str) -> bool,
    has_remote_bin: Option<&dyn Fn(&str) -> bool>,
    has_remote_any_bin: Option<&dyn Fn(&[String]) -> bool>,
    remote_platforms: Option<&[String]>,
) -> RequirementsEvaluation {
    let missing_bins = resolve_missing_bins(&required.bins, has_local_bin, has_remote_bin);
    let missing_any_bins =
        resolve_missing_any_bins(&required.any_bins, has_local_bin, has_remote_any_bin);
    let missing_os = resolve_missing_os(&required.os, local_platform, remote_platforms);
    let missing_env = resolve_missing_env(&required.env, is_env_satisfied);
    let config_checks = build_config_checks(&required.config, is_config_satisfied);
    let missing_config: Vec<String> = config_checks
        .iter()
        .filter(|c| !c.satisfied)
        .map(|c| c.path.clone())
        .collect();

    let missing = if always {
        Requirements::default()
    } else {
        Requirements {
            bins: missing_bins,
            any_bins: missing_any_bins,
            env: missing_env,
            config: missing_config,
            os: missing_os,
        }
    };

    let eligible = always || missing.is_empty();

    RequirementsEvaluation {
        missing,
        eligible,
        config_checks,
    }
}

/// Evaluate requirements from metadata.
pub fn evaluate_requirements_from_metadata(
    always: bool,
    metadata: Option<&RequirementsMetadata>,
    has_local_bin: &dyn Fn(&str) -> bool,
    local_platform: &str,
    is_env_satisfied: &dyn Fn(&str) -> bool,
    is_config_satisfied: &dyn Fn(&str) -> bool,
    has_remote_bin: Option<&dyn Fn(&str) -> bool>,
    has_remote_any_bin: Option<&dyn Fn(&[String]) -> bool>,
    remote_platforms: Option<&[String]>,
) -> (Requirements, RequirementsEvaluation) {
    let required = Requirements {
        bins: metadata
            .and_then(|m| m.requires.as_ref())
            .and_then(|r| r.bins.clone())
            .unwrap_or_default(),
        any_bins: metadata
            .and_then(|m| m.requires.as_ref())
            .and_then(|r| r.any_bins.clone())
            .unwrap_or_default(),
        env: metadata
            .and_then(|m| m.requires.as_ref())
            .and_then(|r| r.env.clone())
            .unwrap_or_default(),
        config: metadata
            .and_then(|m| m.requires.as_ref())
            .and_then(|r| r.config.clone())
            .unwrap_or_default(),
        os: metadata
            .and_then(|m| m.os.clone())
            .unwrap_or_default(),
    };

    let eval = evaluate_requirements(
        always,
        &required,
        has_local_bin,
        local_platform,
        is_env_satisfied,
        is_config_satisfied,
        has_remote_bin,
        has_remote_any_bin,
        remote_platforms,
    );

    (required, eval)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_missing_bins_filters_present() {
        let required = vec!["git".into(), "node".into(), "cargo".into()];
        let has = |bin: &str| bin == "git" || bin == "cargo";
        let missing = resolve_missing_bins(&required, &has, None);
        assert_eq!(missing, vec!["node".to_string()]);
    }

    #[test]
    fn resolve_missing_any_bins_satisfied_locally() {
        let required = vec!["ffmpeg".into(), "avconv".into()];
        let has = |bin: &str| bin == "ffmpeg";
        let missing = resolve_missing_any_bins(&required, &has, None);
        assert!(missing.is_empty());
    }

    #[test]
    fn resolve_missing_any_bins_none_present() {
        let required = vec!["ffmpeg".into(), "avconv".into()];
        let has = |_: &str| false;
        let missing = resolve_missing_any_bins(&required, &has, None);
        assert_eq!(missing, required);
    }

    #[test]
    fn resolve_missing_os_matches_local() {
        let required = vec!["linux".into(), "darwin".into()];
        let missing = resolve_missing_os(&required, "linux", None);
        assert!(missing.is_empty());
    }

    #[test]
    fn resolve_missing_os_no_match() {
        let required = vec!["darwin".into()];
        let missing = resolve_missing_os(&required, "win32", None);
        assert_eq!(missing, vec!["darwin".to_string()]);
    }

    #[test]
    fn evaluate_requirements_always_clears_missing() {
        let required = Requirements {
            bins: vec!["missing-bin".into()],
            ..Default::default()
        };
        let eval = evaluate_requirements(
            true,
            &required,
            &|_| false,
            "linux",
            &|_| false,
            &|_| false,
            None,
            None,
            None,
        );
        assert!(eval.eligible);
        assert!(eval.missing.is_empty());
    }
}
