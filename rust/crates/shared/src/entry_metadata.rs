//! Entry metadata resolution — mirrors src/shared/entry-metadata.ts

/// Frontmatter fields that can carry emoji/homepage.
pub struct FrontmatterInfo {
    pub emoji: Option<String>,
    pub homepage: Option<String>,
    pub website: Option<String>,
    pub url: Option<String>,
}

/// Metadata fields that can carry emoji/homepage.
pub struct MetadataInfo {
    pub emoji: Option<String>,
    pub homepage: Option<String>,
}

/// Resolved emoji and homepage from metadata and frontmatter.
pub struct ResolvedEmojiHomepage {
    pub emoji: Option<String>,
    pub homepage: Option<String>,
}

/// Resolve emoji and homepage from metadata and frontmatter, preferring metadata.
/// Mirrors `resolveEmojiAndHomepage` from src/shared/entry-metadata.ts.
pub fn resolve_emoji_and_homepage(
    metadata: Option<&MetadataInfo>,
    frontmatter: Option<&FrontmatterInfo>,
) -> ResolvedEmojiHomepage {
    let emoji = metadata
        .and_then(|m| m.emoji.clone())
        .or_else(|| frontmatter.and_then(|f| f.emoji.clone()));

    let homepage_raw = metadata
        .and_then(|m| m.homepage.clone())
        .or_else(|| frontmatter.and_then(|f| f.homepage.clone()))
        .or_else(|| frontmatter.and_then(|f| f.website.clone()))
        .or_else(|| frontmatter.and_then(|f| f.url.clone()));

    let homepage = homepage_raw.and_then(|h| {
        let trimmed = h.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    });

    ResolvedEmojiHomepage { emoji, homepage }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefers_metadata_emoji() {
        let meta = MetadataInfo {
            emoji: Some("🤖".into()),
            homepage: None,
        };
        let fm = FrontmatterInfo {
            emoji: Some("🦀".into()),
            homepage: None,
            website: None,
            url: None,
        };
        let result = resolve_emoji_and_homepage(Some(&meta), Some(&fm));
        assert_eq!(result.emoji, Some("🤖".into()));
    }

    #[test]
    fn falls_back_to_frontmatter() {
        let fm = FrontmatterInfo {
            emoji: Some("🦀".into()),
            homepage: None,
            website: Some("https://example.com".into()),
            url: None,
        };
        let result = resolve_emoji_and_homepage(None, Some(&fm));
        assert_eq!(result.emoji, Some("🦀".into()));
        assert_eq!(result.homepage, Some("https://example.com".into()));
    }

    #[test]
    fn trims_homepage() {
        let meta = MetadataInfo {
            emoji: None,
            homepage: Some("  https://example.com  ".into()),
        };
        let result = resolve_emoji_and_homepage(Some(&meta), None);
        assert_eq!(result.homepage, Some("https://example.com".into()));
    }

    #[test]
    fn empty_homepage_becomes_none() {
        let meta = MetadataInfo {
            emoji: None,
            homepage: Some("   ".into()),
        };
        let result = resolve_emoji_and_homepage(Some(&meta), None);
        assert_eq!(result.homepage, None);
    }
}
