//! Gateway bind URL resolution — mirrors src/shared/gateway-bind-url.ts

/// Gateway bind URL resolution result.
#[derive(Debug, Clone)]
pub enum GatewayBindUrlResult {
    Ok {
        url: String,
        source: String,
    },
    Error {
        error: String,
    },
    /// bind mode is "loopback" or "auto" — caller handles locally.
    None,
}

/// Resolve gateway bind URL based on bind mode.
/// Mirrors `resolveGatewayBindUrl` from src/shared/gateway-bind-url.ts.
pub fn resolve_gateway_bind_url(
    bind: Option<&str>,
    custom_bind_host: Option<&str>,
    scheme: &str,
    port: u16,
    pick_tailnet_host: &dyn Fn() -> Option<String>,
    pick_lan_host: &dyn Fn() -> Option<String>,
) -> GatewayBindUrlResult {
    let bind = bind.unwrap_or("loopback");

    match bind {
        "custom" => {
            let host = custom_bind_host.map(|h| h.trim()).filter(|h| !h.is_empty());
            match host {
                Some(h) => GatewayBindUrlResult::Ok {
                    url: format!("{scheme}://{h}:{port}"),
                    source: "gateway.bind=custom".into(),
                },
                None => GatewayBindUrlResult::Error {
                    error: "gateway.bind=custom requires gateway.customBindHost.".into(),
                },
            }
        }
        "tailnet" => match pick_tailnet_host() {
            Some(host) => GatewayBindUrlResult::Ok {
                url: format!("{scheme}://{host}:{port}"),
                source: "gateway.bind=tailnet".into(),
            },
            None => GatewayBindUrlResult::Error {
                error: "gateway.bind=tailnet set, but no tailnet IP was found.".into(),
            },
        },
        "lan" => match pick_lan_host() {
            Some(host) => GatewayBindUrlResult::Ok {
                url: format!("{scheme}://{host}:{port}"),
                source: "gateway.bind=lan".into(),
            },
            None => GatewayBindUrlResult::Error {
                error: "gateway.bind=lan set, but no private LAN IP was found.".into(),
            },
        },
        _ => GatewayBindUrlResult::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_bind_with_host() {
        let result = resolve_gateway_bind_url(
            Some("custom"),
            Some("192.168.1.100"),
            "ws",
            18789,
            &|| None,
            &|| None,
        );
        match result {
            GatewayBindUrlResult::Ok { url, source } => {
                assert_eq!(url, "ws://192.168.1.100:18789");
                assert_eq!(source, "gateway.bind=custom");
            }
            _ => panic!("expected Ok"),
        }
    }

    #[test]
    fn custom_bind_without_host() {
        let result = resolve_gateway_bind_url(
            Some("custom"),
            None,
            "ws",
            18789,
            &|| None,
            &|| None,
        );
        matches!(result, GatewayBindUrlResult::Error { .. });
    }

    #[test]
    fn loopback_returns_none() {
        let result = resolve_gateway_bind_url(
            Some("loopback"),
            None,
            "ws",
            18789,
            &|| None,
            &|| None,
        );
        matches!(result, GatewayBindUrlResult::None);
    }

    #[test]
    fn default_bind_is_loopback() {
        let result = resolve_gateway_bind_url(None, None, "ws", 18789, &|| None, &|| None);
        matches!(result, GatewayBindUrlResult::None);
    }
}
