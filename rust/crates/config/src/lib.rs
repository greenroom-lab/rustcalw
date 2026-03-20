//! rustcalw-config: OpenClaw config loading (1:1 port of src/config/)

pub mod env_substitution;
pub mod io;
pub mod paths;
pub mod types;

pub use io::{load_config, read_config_file, resolve_config_path, resolve_gateway_port};
pub use paths::config_dir;
pub use types::OpenClawConfig;
