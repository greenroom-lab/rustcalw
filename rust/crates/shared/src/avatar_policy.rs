//! Avatar policy — mirrors src/shared/avatar-policy.ts

use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use std::sync::LazyLock;

/// Maximum allowed avatar file size in bytes (2 MiB).
pub const AVATAR_MAX_BYTES: usize = 2 * 1024 * 1024;

static LOCAL_AVATAR_EXTENSIONS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [".png", ".jpg", ".jpeg", ".gif", ".webp", ".svg"]
        .into_iter()
        .collect()
});

static AVATAR_MIME_BY_EXT: LazyLock<std::collections::HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        [
            (".png", "image/png"),
            (".jpg", "image/jpeg"),
            (".jpeg", "image/jpeg"),
            (".webp", "image/webp"),
            (".gif", "image/gif"),
            (".svg", "image/svg+xml"),
            (".bmp", "image/bmp"),
            (".tif", "image/tiff"),
            (".tiff", "image/tiff"),
        ]
        .into_iter()
        .collect()
    });

static AVATAR_DATA_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)^data:").unwrap());
static AVATAR_IMAGE_DATA_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^data:image/").unwrap());
static AVATAR_HTTP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^https?://").unwrap());
static AVATAR_SCHEME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^[a-z][a-z0-9+.\-]*:").unwrap());
static WINDOWS_ABS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z]:[/\\]").unwrap());
static AVATAR_PATH_EXT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\.(png|jpe?g|gif|webp|svg|ico)$").unwrap());

/// Resolve the MIME type for a file path based on its extension.
pub fn resolve_avatar_mime(file_path: &str) -> &'static str {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{}", e.to_lowercase()));
    match ext {
        Some(ref e) => AVATAR_MIME_BY_EXT.get(e.as_str()).copied().unwrap_or("application/octet-stream"),
        None => "application/octet-stream",
    }
}

/// Check if the value is a data: URL.
pub fn is_avatar_data_url(value: &str) -> bool {
    AVATAR_DATA_RE.is_match(value)
}

/// Check if the value is a data:image/ URL.
pub fn is_avatar_image_data_url(value: &str) -> bool {
    AVATAR_IMAGE_DATA_RE.is_match(value)
}

/// Check if the value is an HTTP(S) URL.
pub fn is_avatar_http_url(value: &str) -> bool {
    AVATAR_HTTP_RE.is_match(value)
}

/// Check if the value has any URI scheme.
pub fn has_avatar_uri_scheme(value: &str) -> bool {
    AVATAR_SCHEME_RE.is_match(value)
}

/// Check if the value looks like a Windows absolute path (e.g. `C:\...`).
pub fn is_windows_absolute_path(value: &str) -> bool {
    WINDOWS_ABS_RE.is_match(value)
}

/// Check if the value is a workspace-relative avatar path.
pub fn is_workspace_relative_avatar_path(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }
    if value.starts_with('~') {
        return false;
    }
    if has_avatar_uri_scheme(value) && !is_windows_absolute_path(value) {
        return false;
    }
    true
}

/// Check if a target path is within a root directory.
pub fn is_path_within_root(root_dir: &str, target_path: &str) -> bool {
    let root = Path::new(root_dir);
    let target = Path::new(target_path);
    // canonicalize-free check: use strip_prefix to see if target is under root
    match target.strip_prefix(root) {
        Ok(relative) => {
            let rel_str = relative.to_string_lossy();
            // Equal paths yield empty relative
            if rel_str.is_empty() {
                return true;
            }
            !rel_str.starts_with("..")
        }
        Err(_) => false,
    }
}

/// Check if the value looks like a file path to an avatar image.
pub fn looks_like_avatar_path(value: &str) -> bool {
    if value.contains('/') || value.contains('\\') {
        return true;
    }
    AVATAR_PATH_EXT_RE.is_match(value)
}

/// Check if a file path has a supported local avatar extension.
pub fn is_supported_local_avatar_extension(file_path: &str) -> bool {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{}", e.to_lowercase()));
    match ext {
        Some(ref e) => LOCAL_AVATAR_EXTENSIONS.contains(e.as_str()),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_mime_known() {
        assert_eq!(resolve_avatar_mime("photo.png"), "image/png");
        assert_eq!(resolve_avatar_mime("photo.JPG"), "image/jpeg");
        assert_eq!(resolve_avatar_mime("icon.svg"), "image/svg+xml");
    }

    #[test]
    fn resolve_mime_unknown() {
        assert_eq!(resolve_avatar_mime("file.txt"), "application/octet-stream");
    }

    #[test]
    fn data_url_detection() {
        assert!(is_avatar_data_url("data:image/png;base64,abc"));
        assert!(is_avatar_image_data_url("data:image/png;base64,abc"));
        assert!(!is_avatar_image_data_url("data:text/plain,abc"));
    }

    #[test]
    fn http_url_detection() {
        assert!(is_avatar_http_url("https://example.com/img.png"));
        assert!(is_avatar_http_url("http://example.com/img.png"));
        assert!(!is_avatar_http_url("ftp://example.com/img.png"));
    }

    #[test]
    fn scheme_detection() {
        assert!(has_avatar_uri_scheme("https://x"));
        assert!(has_avatar_uri_scheme("ftp://x"));
        assert!(!has_avatar_uri_scheme("/local/path"));
    }

    #[test]
    fn windows_path_detection() {
        assert!(is_windows_absolute_path("C:\\Users\\foo"));
        assert!(is_windows_absolute_path("D:/images/pic.png"));
        assert!(!is_windows_absolute_path("/unix/path"));
    }

    #[test]
    fn workspace_relative() {
        assert!(is_workspace_relative_avatar_path("images/avatar.png"));
        assert!(!is_workspace_relative_avatar_path(""));
        assert!(!is_workspace_relative_avatar_path("~/.config/avatar.png"));
        assert!(!is_workspace_relative_avatar_path("https://example.com/a.png"));
        // Windows absolute path has a scheme-like prefix but is still a path
        assert!(is_workspace_relative_avatar_path("C:\\Users\\pic.png"));
    }

    #[test]
    fn looks_like_path() {
        assert!(looks_like_avatar_path("images/avatar.png"));
        assert!(looks_like_avatar_path("avatar.png"));
        assert!(!looks_like_avatar_path("random_string"));
    }

    #[test]
    fn supported_extension() {
        assert!(is_supported_local_avatar_extension("a.png"));
        assert!(is_supported_local_avatar_extension("a.JPEG"));
        assert!(!is_supported_local_avatar_extension("a.bmp"));
        assert!(!is_supported_local_avatar_extension("a.txt"));
    }
}
