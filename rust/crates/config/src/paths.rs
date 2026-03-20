//! Config path resolution — mirrors src/config/paths.ts

use std::path::PathBuf;

/// Returns the OpenClaw config directory (~/.openclaw)
/// Matches the original OpenClaw path structure exactly.
pub fn config_dir() -> PathBuf {
    let home = dirs_next()
        .expect("could not determine home directory");
    home.join(".openclaw")
}

/// Returns the workspace directory (~/.openclaw/workspace)
pub fn workspace_dir() -> PathBuf {
    config_dir().join("workspace")
}

fn dirs_next() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        std::env::var_os("USERPROFILE").map(PathBuf::from)
    }
    #[cfg(not(windows))]
    {
        std::env::var_os("HOME").map(PathBuf::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_dir_ends_with_openclaw() {
        let dir = config_dir();
        assert!(dir.ends_with(".openclaw"));
    }

    #[test]
    fn workspace_dir_is_under_config() {
        let ws = workspace_dir();
        assert!(ws.starts_with(config_dir()));
        assert!(ws.ends_with("workspace"));
    }
}
