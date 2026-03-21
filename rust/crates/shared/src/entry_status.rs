//! Entry status evaluation — mirrors src/shared/entry-status.ts

use crate::entry_metadata::{resolve_emoji_and_homepage, FrontmatterInfo, MetadataInfo};
use crate::requirements::{
    evaluate_requirements_from_metadata, RequirementConfigCheck, RequirementRemote, Requirements,
    RequirementsMetadata,
};

/// Result of evaluating entry metadata requirements.
#[derive(Debug, Clone)]
pub struct EntryMetadataRequirementsResult {
    pub emoji: Option<String>,
    pub homepage: Option<String>,
    pub required: Requirements,
    pub missing: Requirements,
    pub requirements_satisfied: bool,
    pub config_checks: Vec<RequirementConfigCheck>,
}

/// Evaluate entry metadata requirements including emoji/homepage resolution.
pub fn evaluate_entry_metadata_requirements(
    always: bool,
    metadata: Option<&RequirementsMetadata>,
    metadata_emoji_homepage: Option<&MetadataInfo>,
    frontmatter: Option<&FrontmatterInfo>,
    has_local_bin: &dyn Fn(&str) -> bool,
    local_platform: &str,
    remote: Option<&RequirementRemote>,
    is_env_satisfied: &dyn Fn(&str) -> bool,
    is_config_satisfied: &dyn Fn(&str) -> bool,
) -> EntryMetadataRequirementsResult {
    let resolved = resolve_emoji_and_homepage(metadata_emoji_homepage, frontmatter);

    let has_remote_bin: Option<&dyn Fn(&str) -> bool> =
        remote.and_then(|r| r.has_bin.as_ref().map(|f| f.as_ref() as &dyn Fn(&str) -> bool));
    let has_remote_any_bin: Option<&dyn Fn(&[String]) -> bool> =
        remote.and_then(|r| r.has_any_bin.as_ref().map(|f| f.as_ref() as &dyn Fn(&[String]) -> bool));
    let remote_platforms: Option<&[String]> =
        remote.and_then(|r| r.platforms.as_deref());

    let (required, eval) = evaluate_requirements_from_metadata(
        always,
        metadata,
        has_local_bin,
        local_platform,
        is_env_satisfied,
        is_config_satisfied,
        has_remote_bin,
        has_remote_any_bin,
        remote_platforms,
    );

    EntryMetadataRequirementsResult {
        emoji: resolved.emoji,
        homepage: resolved.homepage,
        required,
        missing: eval.missing,
        requirements_satisfied: eval.eligible,
        config_checks: eval.config_checks,
    }
}

/// Evaluate entry metadata requirements for the current platform.
pub fn evaluate_entry_metadata_requirements_for_current_platform(
    always: bool,
    metadata: Option<&RequirementsMetadata>,
    metadata_emoji_homepage: Option<&MetadataInfo>,
    frontmatter: Option<&FrontmatterInfo>,
    has_local_bin: &dyn Fn(&str) -> bool,
    remote: Option<&RequirementRemote>,
    is_env_satisfied: &dyn Fn(&str) -> bool,
    is_config_satisfied: &dyn Fn(&str) -> bool,
) -> EntryMetadataRequirementsResult {
    let platform = if cfg!(target_os = "windows") {
        "win32"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else {
        "linux"
    };

    evaluate_entry_metadata_requirements(
        always,
        metadata,
        metadata_emoji_homepage,
        frontmatter,
        has_local_bin,
        platform,
        remote,
        is_env_satisfied,
        is_config_satisfied,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn satisfied_when_always() {
        let meta = RequirementsMetadata {
            requires: Some(crate::requirements::PartialRequirements {
                bins: Some(vec!["missing".into()]),
                ..Default::default()
            }),
            os: None,
        };
        let result = evaluate_entry_metadata_requirements(
            true,
            Some(&meta),
            None,
            None,
            &|_| false,
            "linux",
            None,
            &|_| false,
            &|_| false,
        );
        assert!(result.requirements_satisfied);
    }

    #[test]
    fn resolves_emoji_from_metadata() {
        let meta_info = MetadataInfo {
            emoji: Some("🤖".into()),
            homepage: None,
        };
        let result = evaluate_entry_metadata_requirements(
            true,
            None,
            Some(&meta_info),
            None,
            &|_| false,
            "linux",
            None,
            &|_| false,
            &|_| false,
        );
        assert_eq!(result.emoji, Some("🤖".into()));
    }
}
