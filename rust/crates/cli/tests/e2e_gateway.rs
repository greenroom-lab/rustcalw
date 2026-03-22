use std::time::Duration;

/// Spawn the gateway binary on a given port, returning the child process.
fn spawn_gateway(port: u16) -> std::process::Child {
    let bin = assert_cmd::cargo::cargo_bin("rustcalw-cli");
    std::process::Command::new(bin)
        .args(["gateway"])
        .env("OPENCLAW_GATEWAY_PORT", port.to_string())
        .env("RUST_LOG", "warn")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to start gateway process")
}

/// Wait until the gateway health endpoint responds, or timeout.
async fn wait_for_ready(port: u16, timeout: Duration) -> bool {
    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{port}/health");
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if client.get(&url).send().await.is_ok() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    false
}

#[tokio::test]
async fn gateway_health_endpoint_returns_live() {
    let port = 29701u16;
    let mut child = spawn_gateway(port);

    let ready = wait_for_ready(port, Duration::from_secs(10)).await;
    assert!(ready, "gateway did not start within 10 seconds");

    let client = reqwest::Client::new();

    // GET /health
    let resp = client
        .get(format!("http://127.0.0.1:{port}/health"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let ct = resp.headers().get("content-type").unwrap().to_str().unwrap();
    assert_eq!(ct, "application/json; charset=utf-8");

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["ok"], true);
    assert_eq!(body["status"], "live");

    child.kill().ok();
    child.wait().ok();
}

#[tokio::test]
async fn gateway_healthz_endpoint() {
    let port = 29702u16;
    let mut child = spawn_gateway(port);

    let ready = wait_for_ready(port, Duration::from_secs(10)).await;
    assert!(ready, "gateway did not start within 10 seconds");

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("http://127.0.0.1:{port}/healthz"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "live");

    child.kill().ok();
    child.wait().ok();
}

#[tokio::test]
async fn gateway_ready_endpoint() {
    let port = 29703u16;
    let mut child = spawn_gateway(port);

    let ready = wait_for_ready(port, Duration::from_secs(10)).await;
    assert!(ready, "gateway did not start within 10 seconds");

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("http://127.0.0.1:{port}/ready"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["ok"], true);
    assert_eq!(body["status"], "ready");

    child.kill().ok();
    child.wait().ok();
}

#[tokio::test]
async fn gateway_post_health_returns_405() {
    let port = 29704u16;
    let mut child = spawn_gateway(port);

    let ready = wait_for_ready(port, Duration::from_secs(10)).await;
    assert!(ready, "gateway did not start within 10 seconds");

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://127.0.0.1:{port}/health"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 405);

    child.kill().ok();
    child.wait().ok();
}
