use std::net::SocketAddr;

use axum::{
    Router,
    body::Body,
    http::{HeaderValue, Method, StatusCode, header},
    response::Response,
    routing::get,
};
use tokio::{net::TcpListener, sync::watch, task::JoinHandle};
use tower::ServiceBuilder;

// Re-export for convenient access.
pub use crate::server_constants::RUST_GATEWAY_DEFAULT_PORT;

/// Gateway HTTP server.
///
/// Binds to the specified host:port and serves health endpoints.
/// Graceful shutdown via the `shutdown()` method.
pub struct GatewayServer {
    addr: SocketAddr,
    shutdown_tx: watch::Sender<bool>,
    handle: JoinHandle<()>,
}

impl GatewayServer {
    /// Start the gateway server on `host:port`.
    ///
    /// Use port `0` to let the OS pick an available port (useful in tests).
    pub async fn start(host: &str, port: u16) -> anyhow::Result<Self> {
        let listener = TcpListener::bind(format!("{host}:{port}")).await?;
        let addr = listener.local_addr()?;
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let app = build_router();

        let handle = tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    let mut rx = shutdown_rx;
                    let _ = rx.wait_for(|&v| v).await;
                })
                .await
                .ok();
        });

        Ok(Self {
            addr,
            shutdown_tx,
            handle,
        })
    }

    /// The actual address the server is listening on.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// The port the server is listening on.
    pub fn port(&self) -> u16 {
        self.addr.port()
    }

    /// Gracefully shut down the server.
    pub async fn shutdown(self) -> anyhow::Result<()> {
        let _ = self.shutdown_tx.send(true);
        self.handle.await?;
        Ok(())
    }
}

/// Build the axum router with health endpoints.
pub fn build_router() -> Router {
    Router::new()
        .route("/health", get(handle_live).head(handle_live_head))
        .route("/healthz", get(handle_live).head(handle_live_head))
        .route("/ready", get(handle_ready).head(handle_ready_head))
        .route("/readyz", get(handle_ready).head(handle_ready_head))
        .fallback(handle_fallback)
        .layer(ServiceBuilder::new())
}

fn json_response(status: StatusCode, body: &str) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .header(header::CACHE_CONTROL, "no-store")
        .body(Body::from(body.to_owned()))
        .unwrap()
}

fn head_response(status: StatusCode) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .header(header::CACHE_CONTROL, "no-store")
        .body(Body::empty())
        .unwrap()
}

async fn handle_live() -> Response {
    json_response(
        StatusCode::OK,
        r#"{"ok":true,"status":"live"}"#,
    )
}

async fn handle_live_head() -> Response {
    head_response(StatusCode::OK)
}

async fn handle_ready() -> Response {
    json_response(
        StatusCode::OK,
        r#"{"ok":true,"status":"ready"}"#,
    )
}

async fn handle_ready_head() -> Response {
    head_response(StatusCode::OK)
}

async fn handle_fallback(_method: Method, uri: axum::http::Uri) -> Response {
    let path = uri.path();
    let is_health_path = matches!(path, "/health" | "/healthz" | "/ready" | "/readyz");

    if is_health_path {
        // Non-GET/HEAD method on health paths → 405
        Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .header(header::ALLOW, HeaderValue::from_static("GET, HEAD"))
            .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(Body::from(
                r#"{"ok":false,"error":"Method Not Allowed"}"#,
            ))
            .unwrap()
    } else {
        json_response(StatusCode::NOT_FOUND, r#"{"ok":false,"error":"Not Found"}"#)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use tower::ServiceExt;

    async fn oneshot_get(app: Router, path: &str) -> Response {
        app.oneshot(
            axum::http::Request::builder()
                .method(Method::GET)
                .uri(path)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
    }

    async fn oneshot_method(app: Router, method: Method, path: &str) -> Response {
        app.oneshot(
            axum::http::Request::builder()
                .method(method)
                .uri(path)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
    }

    async fn body_string(resp: Response) -> String {
        let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[tokio::test]
    async fn health_returns_live_status() {
        let app = build_router();
        let resp = oneshot_get(app, "/health").await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_string(resp).await;
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(v["ok"], true);
        assert_eq!(v["status"], "live");
    }

    #[tokio::test]
    async fn healthz_returns_live_status() {
        let app = build_router();
        let resp = oneshot_get(app, "/healthz").await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_string(resp).await;
        assert!(body.contains(r#""status":"live""#));
    }

    #[tokio::test]
    async fn ready_returns_ready_status() {
        let app = build_router();
        let resp = oneshot_get(app, "/ready").await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_string(resp).await;
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(v["ok"], true);
        assert_eq!(v["status"], "ready");
    }

    #[tokio::test]
    async fn readyz_returns_ready_status() {
        let app = build_router();
        let resp = oneshot_get(app, "/readyz").await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn head_health_returns_empty_body() {
        let app = build_router();
        let resp = oneshot_method(app, Method::HEAD, "/health").await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_string(resp).await;
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn post_health_returns_method_not_allowed() {
        let app = build_router();
        let resp = oneshot_method(app, Method::POST, "/health").await;
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
        let allow = resp.headers().get(header::ALLOW).unwrap().to_str().unwrap();
        // axum returns "GET,HEAD" (no space); both formats are valid HTTP
        assert!(allow.contains("GET"));
        assert!(allow.contains("HEAD"));
    }

    #[tokio::test]
    async fn unknown_path_returns_not_found() {
        let app = build_router();
        let resp = oneshot_get(app, "/nonexistent").await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn health_has_correct_headers() {
        let app = build_router();
        let resp = oneshot_get(app, "/health").await;
        let ct = resp.headers().get(header::CONTENT_TYPE).unwrap();
        assert_eq!(ct, "application/json; charset=utf-8");
        let cc = resp.headers().get(header::CACHE_CONTROL).unwrap();
        assert_eq!(cc, "no-store");
    }

    #[tokio::test]
    async fn server_starts_and_stops() {
        let server = GatewayServer::start("127.0.0.1", 0).await.unwrap();
        assert_ne!(server.port(), 0);
        server.shutdown().await.unwrap();
    }
}
