use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wnc", about = "rustcalw — OpenClaw Rust port")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the gateway server
    Gateway,
    /// Config management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Check (validate) the current config file
    Check,
    /// Show the resolved config path
    Path,
    /// Verify config round-trip: load → serialize → reload → compare
    DeployCheck,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Gateway => {
            let port = std::env::var("OPENCLAW_GATEWAY_PORT")
                .ok()
                .and_then(|s| s.parse::<u16>().ok())
                .or_else(|| {
                    rustcalw_config::load_config()
                        .ok()
                        .and_then(|s| s.config.gateway)
                        .and_then(|g| g.port)
                })
                .unwrap_or(rustcalw_gateway::server_constants::RUST_GATEWAY_DEFAULT_PORT);

            tracing::info!("rustcalw gateway starting on port {port}...");

            let server =
                rustcalw_gateway::server::GatewayServer::start("127.0.0.1", port).await?;

            tracing::info!(
                "rustcalw gateway listening on http://127.0.0.1:{}",
                server.port()
            );

            tokio::signal::ctrl_c().await?;
            tracing::info!("shutting down...");
            server.shutdown().await?;
        }
        Commands::Config { action } => match action {
            ConfigAction::Check => {
                let snapshot = rustcalw_config::load_config()?;
                if snapshot.exists {
                    println!("Config file: {}", snapshot.path);
                    println!("Valid: {}", snapshot.valid);
                    if let Some(hash) = &snapshot.hash {
                        println!("Hash: {hash}");
                    }

                    // Show key config details
                    let cfg = &snapshot.config;
                    if let Some(gw) = &cfg.gateway {
                        let port = gw.port.unwrap_or(rustcalw_config::io::DEFAULT_GATEWAY_PORT);
                        let mode = gw.mode.as_deref().unwrap_or("local");
                        println!("Gateway: port={port}, mode={mode}");
                    }
                    if let Some(agents) = &cfg.agents {
                        let count = agents.list.as_ref().map_or(0, |l| l.len());
                        println!("Agents: {count} configured");
                    }
                    if let Some(models) = &cfg.models {
                        let count = models.providers.as_ref().map_or(0, |p| p.len());
                        println!("Model providers: {count}");
                    }
                    if let Some(channels) = &cfg.channels {
                        let mut active = vec![];
                        if channels.discord.is_some() {
                            active.push("discord");
                        }
                        if channels.telegram.is_some() {
                            active.push("telegram");
                        }
                        if channels.slack.is_some() {
                            active.push("slack");
                        }
                        if channels.whatsapp.is_some() {
                            active.push("whatsapp");
                        }
                        if !active.is_empty() {
                            println!("Channels: {}", active.join(", "));
                        }
                    }

                    if !snapshot.warnings.is_empty() {
                        println!("\nWarnings:");
                        for w in &snapshot.warnings {
                            println!("  - {}: {}", w.path, w.message);
                        }
                    }
                    if !snapshot.issues.is_empty() {
                        println!("\nIssues:");
                        for issue in &snapshot.issues {
                            println!("  - {}: {}", issue.path, issue.message);
                        }
                    }

                    println!("\nConfig loaded successfully.");
                } else {
                    println!("Config file not found: {}", snapshot.path);
                    println!("Create it with: mkdir -p ~/.openclaw && echo '{{}}' > ~/.openclaw/openclaw.json");
                }
            }
            ConfigAction::Path => {
                let path = rustcalw_config::resolve_config_path();
                println!("{}", path.display());
            }
            ConfigAction::DeployCheck => {
                let snapshot = rustcalw_config::load_config()?;
                if !snapshot.exists {
                    eprintln!("Config file not found: {}", snapshot.path);
                    std::process::exit(1);
                }

                // Round-trip: serialize back to JSON
                let json = serde_json::to_string_pretty(&snapshot.config)
                    .expect("config should serialize to JSON");

                // Parse again
                let config2: rustcalw_config::types::openclaw::OpenClawConfig =
                    serde_json::from_str(&json)
                        .expect("round-tripped JSON should parse back");

                // Compare key fields
                let mut errors = vec![];

                // Gateway port
                let port1 = snapshot.config.gateway.as_ref().and_then(|g| g.port);
                let port2 = config2.gateway.as_ref().and_then(|g| g.port);
                if port1 != port2 {
                    errors.push(format!("gateway.port: {port1:?} != {port2:?}"));
                }

                // Model providers count
                let p1 = snapshot.config.models.as_ref().and_then(|m| m.providers.as_ref()).map(|p| p.len());
                let p2 = config2.models.as_ref().and_then(|m| m.providers.as_ref()).map(|p: &std::collections::HashMap<_, _>| p.len());
                if p1 != p2 {
                    errors.push(format!("models.providers count: {p1:?} != {p2:?}"));
                }

                // YAML round-trip
                let yaml = serde_yaml::to_string(&snapshot.config)
                    .expect("config should serialize to YAML");
                let _config3: rustcalw_config::types::openclaw::OpenClawConfig =
                    serde_yaml::from_str(&yaml)
                        .expect("YAML round-trip should parse back");

                if errors.is_empty() {
                    println!("Deploy check passed.");
                    println!("  Config: {}", snapshot.path);
                    println!("  JSON round-trip: OK");
                    println!("  YAML round-trip: OK");
                    let providers: Vec<String> = snapshot.config.models
                        .as_ref()
                        .and_then(|m| m.providers.as_ref())
                        .map(|p| p.keys().cloned().collect())
                        .unwrap_or_default();
                    println!("  Providers: {}", providers.join(", "));
                } else {
                    eprintln!("Deploy check FAILED:");
                    for e in &errors {
                        eprintln!("  - {e}");
                    }
                    std::process::exit(1);
                }
            }
        },
    }

    Ok(())
}
