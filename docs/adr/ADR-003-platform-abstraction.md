# ADR-003: プラットフォーム抽象化レイヤー設計

- **状態**: 承認
- **日付**: 2026-03-21
- **関連**: [ADR-002](ADR-002-windows-ipc.md), [ADR-004](ADR-004-crate-structure.md)

## コンテキスト

rustcalw は Windows と Unix の両プラットフォームで動作する必要がある。プラットフォーム固有コードの管理戦略が必要。

### 既存のプラットフォーム分岐パターン

#### pid_alive.rs — 3 層分岐
```rust
#[cfg(unix)]
fn process_exists(pid: u32) -> bool {
    unsafe { libc::kill(pid as i32, 0) == 0 }
}

#[cfg(windows)]
fn process_exists(pid: u32) -> bool {
    // tasklist コマンドで確認
}

#[cfg(not(any(unix, windows)))]
fn process_exists(_pid: u32) -> bool { false }
```

#### paths.rs — 2 層分岐
```rust
fn dirs_next() -> Option<PathBuf> {
    #[cfg(windows)]
    { std::env::var_os("USERPROFILE").map(PathBuf::from) }
    #[cfg(not(windows))]
    { std::env::var_os("HOME").map(PathBuf::from) }
}
```

#### upstream TypeScript — runtime 分岐
```typescript
// src/daemon/service.ts
const GATEWAY_SERVICE_REGISTRY = {
  darwin: { install: installLaunchAgent },
  linux:  { install: installSystemdService },
  win32:  { install: installScheduledTask },
};
```

## 決定

### 2 段階アプローチ

#### 現在 (Phase 1-3): 各 crate 内で `#[cfg]` 分岐

プラットフォーム固有コードが少ない間は、各 crate 内で直接 `#[cfg]` 分岐を使用:

```rust
// crate 内の個別モジュールで分岐
#[cfg(windows)]
fn platform_specific_impl() { /* Windows 実装 */ }

#[cfg(unix)]
fn platform_specific_impl() { /* Unix 実装 */ }
```

**適用基準**: 分岐箇所が 1 crate あたり 3 箇所以下

#### 将来 (Phase 4+): `rustcalw-platform` crate

プラットフォーム固有コードが増えた段階で専用 crate を導入:

```
rust/crates/
├── platform/        ← NEW: プラットフォーム抽象化レイヤー
│   ├── src/
│   │   ├── lib.rs
│   │   ├── signal.rs    # シグナル/イベント (ADR-002)
│   │   ├── ipc.rs       # IPC: Unix socket / Named Pipe (ADR-002)
│   │   ├── process.rs   # プロセス管理 (pid_alive の抽出)
│   │   └── daemon.rs    # デーモン: launchd / systemd / schtasks
│   └── Cargo.toml
```

### 抽象化の 4 つの軸

| 軸 | Unix | Windows | 抽象化 trait/fn |
|----|------|---------|----------------|
| **Signal** | SIGUSR1, SIGTERM, SIGINT | Named Event, ctrl_c | `wait_for_reload()`, `wait_for_shutdown()` |
| **IPC** | Unix domain socket | Named Pipe | `IpcListener`, `IpcStream` |
| **Process** | kill(pid, 0), /proc | tasklist, OpenProcess | `is_pid_alive()`, `kill_tree()` |
| **Daemon** | launchd, systemd | schtasks | `install_service()`, `remove_service()` |

### `#[cfg]` 使用ルール

1. **ビジネスロジックに `#[cfg]` を入れない**: 分岐はインターフェース層 (platform モジュール) に閉じ込める
2. **フォールバックを必ず用意**: `#[cfg(not(any(unix, windows)))]` で安全なデフォルト
3. **テストは両プラットフォームで実行**: CI マトリックス (ubuntu-latest + windows-latest)
4. **`unsafe` は最小限**: 可能なら外部コマンド呼び出し (tasklist 等) で代替

## 根拠

1. **YAGNI**: 現時点でのプラットフォーム分岐は `pid_alive.rs` と `paths.rs` の 2 箇所のみ。専用 crate は過剰
2. **段階的導入**: Phase 3 で gateway サーバーを実装する際に分岐箇所が増え、自然に `platform` crate の必要性が生まれる
3. **既存パターンの踏襲**: `pid_alive.rs` のパターンが実証済みで安定
4. **upstream との対応**: TypeScript の `GATEWAY_SERVICE_REGISTRY` パターンを Rust の trait + `#[cfg]` で表現

## 影響

- Phase 3 までは `platform` crate を作成しない。各 crate 内で `#[cfg]` 分岐
- Phase 4 で `platform` crate を導入する際、既存の `pid_alive.rs` と `paths.rs` の分岐を移行
- `platform` crate は他の全 crate から依存される基盤 crate になる
- 移行時のチェックリスト:
  1. 分岐箇所を `platform` crate に集約
  2. 各 crate から `platform` への依存を追加
  3. テストが Windows + Linux で通過することを確認
