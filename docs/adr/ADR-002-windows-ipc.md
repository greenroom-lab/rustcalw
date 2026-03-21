# ADR-002: Windows IPC 設計 — SIGUSR1 代替と Named Pipe

- **状態**: 承認
- **日付**: 2026-03-21
- **関連**: [ADR-003](ADR-003-platform-abstraction.md), [ADR-001](ADR-001-mirror-strategy.md)

## コンテキスト

OpenClaw の gateway は以下の Unix 固有メカニズムに依存している:

### SIGUSR1 (26ファイルで使用)
- `src/cli/gateway-cli/run-loop.ts`: SIGTERM, SIGINT, SIGUSR1 の 3 シグナルで gateway ライフサイクル管理
- `src/gateway/server-reload-handlers.ts`: SIGUSR1 リスナーカウントによる reload 検知
- `src/infra/restart.ts`: SIGUSR1 をグレースフル再起動のトリガーとして使用
- Windows には SIGUSR1 が存在しない

### Unix domain socket (27ファイルで使用)
- `src/infra/exec-approvals.ts`: `~/.openclaw/exec-approvals.sock` でデフォルト IPC
- `src/infra/jsonl-socket.ts`: JSONL over Unix domain socket プロトコル
- Windows 10 1803+ で Unix socket が部分サポートされるが、パス長制限あり

### 既存の Windows 対応の先例
- `src/daemon/schtasks.ts`: Windows Task Scheduler によるデーモン管理 (launchd/systemd と並ぶ 3 プラットフォーム分岐)
- `src/process/kill-tree.ts`: taskkill /T /PID による Windows プロセスツリー終了
- `rust/crates/shared/src/pid_alive.rs`: `#[cfg(windows)]` で tasklist コマンド呼び出し

## 決定

### SIGUSR1 代替: プラットフォーム Signal trait

```rust
// rust/crates/platform/src/signal.rs (将来)

#[cfg(unix)]
pub async fn wait_for_reload() -> anyhow::Result<()> {
    use tokio::signal::unix::{signal, SignalKind};
    let mut sig = signal(SignalKind::user_defined1())?;
    sig.recv().await;
    Ok(())
}

#[cfg(windows)]
pub async fn wait_for_reload() -> anyhow::Result<()> {
    // Windows Named Event による reload 通知
    // CreateEventW で名前付きイベントを作成し、SetEvent で通知
    // 暫定実装: tokio::signal::ctrl_c() + 設定ファイル変更検知
    todo!("Named Event implementation")
}
```

**段階的実装**:
1. Phase 3 (gateway サーバー実装) で `tokio::signal::ctrl_c()` を最低限の shutdown トリガーとして実装
2. reload は設定ファイルの変更検知 (`notify` crate) で代替
3. 将来的に Windows Named Event (`CreateEventW` / `OpenEventW`) で SIGUSR1 相当を実装

### Unix socket 代替: プラットフォーム IPC trait

```rust
// rust/crates/platform/src/ipc.rs (将来)

pub trait IpcListener: Send + Sync {
    async fn accept(&self) -> anyhow::Result<Box<dyn IpcStream>>;
}

pub trait IpcStream: AsyncRead + AsyncWrite + Send + Sync {}

#[cfg(unix)]
pub fn create_listener(path: &str) -> anyhow::Result<impl IpcListener> {
    // tokio::net::UnixListener
}

#[cfg(windows)]
pub fn create_listener(name: &str) -> anyhow::Result<impl IpcListener> {
    // tokio::net::windows::named_pipe::ServerOptions
    // パイプ名: \\.\pipe\rustcalw-<name>
}
```

**命名規則**:
- Unix: `~/.openclaw/<name>.sock` (upstream 互換)
- Windows: `\\.\pipe\rustcalw-<name>` (Named Pipe)

## 根拠

1. **Named Event + Named Pipe は Windows の標準 IPC**: .NET/Win32 アプリケーションで広く使われており、安定性が高い
2. **tokio エコシステムとの統合**: `tokio::net::windows::named_pipe` が公式サポート
3. **段階的実装**: Phase 3 では `ctrl_c()` + `notify` で最低限動作させ、IPC は後続フェーズで本格実装
4. **trait 抽象化**: ビジネスロジックはプラットフォームを意識しない (CLAUDE.md の設計原則と整合)

## 影響

- `platform` crate の追加は Phase 4 以降。Phase 3 では gateway crate 内に `#[cfg]` 分岐を直接配置
- exec-approvals の IPC は Phase 4 で channels/infra と同時に実装
- テストでは Unix socket / Named Pipe の両方をカバーする必要がある (CI マトリックス)
- upstream の socket パス (`~/.openclaw/exec-approvals.sock`) との互換性を維持
