# Windows 互換性マトリックス

OpenClaw (src/) の各モジュールにおける Windows ネイティブ対応状況を評価する。

## 評価基準

| 記号 | 意味 |
|------|------|
| ✅ | 完全互換 — プラットフォーム非依存 |
| 🟡 | 条件付き互換 — 既存の Windows 分岐あり、または軽微な対応で動作 |
| 🔴 | 要対応 — Windows 固有の代替実装が必要 |
| ⬜ | 未評価 / 移植対象外 |

## コアモジュール

| src/ モジュール | Windows 互換性 | 移植優先度 | 難易度 | 備考 |
|----------------|---------------|-----------|--------|------|
| shared/ | ✅ | 済 | 低 | 型定義・テキスト処理中心。pid_alive のみ #[cfg] 分岐 |
| config/ | ✅ | 済 | 低 | パス解決で USERPROFILE/HOME 分岐 (paths.rs で対応済み) |
| config/types/ | ✅ | 済 | 低 | 純粋な型定義 |
| providers/ | ✅ | 済 | 低 | OAuth フロー・モデル定義。HTTP ベースで OS 非依存 |
| gateway/protocol/ | ✅ | 済 | 低 | WebSocket フレーム型定義・スキーマ。プラットフォーム非依存 |
| gateway/server/ | 🔴 | 高 | 高 | SIGUSR1 リスタート (run-loop.ts L40-42)、Unix socket IPC |
| gateway/server-reload-handlers.ts | 🔴 | 高 | 高 | SIGUSR1 リスナーカウント (L175-176) |
| cli/ | 🟡 | 中 | 中 | コマンド実行は対応済み。gateway 起動部分に注意 |
| cli/gateway-cli/run-loop.ts | 🔴 | 高 | 高 | SIGTERM, SIGINT, SIGUSR1 の 3 シグナルでライフサイクル管理 |
| commands/ | 🟡 | 中 | 低 | 大半はプラットフォーム非依存 |
| channels/ | 🟡 | 中 | 中 | チャネル統合ロジック。プロトコルは OS 非依存だがプロセス起動に注意 |
| routing/ | ✅ | 低 | 低 | メッセージルーティングロジック。OS 非依存 |
| agents/ | 🟡 | 中 | 中 | サンドボックス (Docker)、ツール実行にプラットフォーム依存あり |
| agents/sandbox/ | 🔴 | 低 | 高 | Docker コンテナ管理。Windows ではオプショナル |
| plugins/ | 🟡 | 低 | 中 | プラグインロード・jiti。Node.js 依存 |
| plugin-sdk/ | ✅ | 低 | 低 | TypeBox スキーマ定義。OS 非依存 |
| sessions/ | ✅ | 中 | 低 | セッション管理ロジック。OS 非依存 |
| infra/ | 🔴 | 高 | 高 | exec-approvals (Unix socket)、restart (SIGUSR1) |
| infra/exec-approvals.ts | 🔴 | 高 | 高 | DEFAULT_SOCKET = "~/.openclaw/exec-approvals.sock" |
| infra/jsonl-socket.ts | 🔴 | 高 | 高 | Unix socket JSONL プロトコル |
| infra/restart.ts | 🟡 | 高 | 中 | launchctl/systemd/schtasks 3 分岐。schtasks は実装済み |
| daemon/ | 🟡 | 中 | 中 | schtasks.ts (Windows) 実装済み |
| process/ | 🟡 | 中 | 中 | kill-tree.ts: taskkill 対応済み。supervisor: detached 分岐あり |
| process/exec.ts | 🟡 | 中 | 中 | .cmd/.bat 解決、cmd.exe エスケープ対応済み |
| security/ | 🟡 | 中 | 中 | windows-acl.ts (icacls) 実装済み |
| secrets/ | 🟡 | 低 | 低 | ファイルベース。パーミッション管理に注意 |
| memory/ | ✅ | 低 | 低 | ストレージロジック。OS 非依存 |
| cron/ | ✅ | 低 | 低 | croner ベースのスケジューリング |
| i18n/ | ✅ | 低 | 低 | 国際化テキスト。OS 非依存 |
| web-search/ | ✅ | 低 | 低 | HTTP API ベース。OS 非依存 |
| media/ | 🟡 | 低 | 中 | FFmpeg 依存あり |
| browser/ | 🟡 | 低 | 中 | Playwright/Chromium。Windows 対応あり |
| canvas-host/ | ✅ | 低 | 低 | HTTP サーブ。OS 非依存 |
| tui/ | ✅ | 低 | 低 | ターミナル UI。Node.js の readline/chalk ベース |
| wizard/ | ✅ | 低 | 低 | インタラクティブセットアップ |
| pairing/ | ✅ | 低 | 低 | デバイスペアリングプロトコル |

## 高影響の Windows 非互換箇所

### SIGUSR1 依存 (26ファイル)
gateway のグレースフルリスタートの中核メカニズム。Windows に SIGUSR1 は存在しない。

**影響ファイル** (主要):
- `src/cli/gateway-cli/run-loop.ts` — SIGTERM, SIGINT, SIGUSR1 の 3 シグナルハンドラ
- `src/gateway/server-reload-handlers.ts` — SIGUSR1 リスナーカウント
- `src/infra/restart.ts` — SIGUSR1 をグレースフル再起動トリガーとして使用

**Rust 移植方針**: → ADR-002 参照

### Unix domain socket / IPC (27ファイル)
exec-approvals と内部通信に使用。

**影響ファイル** (主要):
- `src/infra/exec-approvals.ts` — `~/.openclaw/exec-approvals.sock`
- `src/infra/jsonl-socket.ts` — JSONL over Unix socket
- `src/gateway/server/` — gateway WebSocket サーバー

**Rust 移植方針**: → ADR-002 参照

### Shell script 依存 (70ファイル)
ビルド補助・テストスクリプト。大半は macOS/Linux 専用。

**影響**: Rust ネイティブ実装では不要。CI で必要な場合は cross-platform スクリプトに置換。

## extensions/ の Windows 互換性

| カテゴリ | 代表的拡張 | 互換性 | 備考 |
|---------|-----------|--------|------|
| モデルプロバイダー | openai, anthropic, google, ollama | ✅ | HTTP API ベース |
| チャネル (メッセージング) | discord, telegram, slack | ✅ | WebSocket/HTTP ベース |
| チャネル (ネイティブ) | imessage, signal | 🔴 | macOS/Linux 固有バイナリ依存 |
| メモリ | memory-core, memory-lancedb | 🟡 | LanceDB のネイティブビルドに注意 |
| 音声 | elevenlabs, talk-voice | ✅ | HTTP API ベース |
| デバイス | device-pair, phone-control | 🟡 | Bluetooth/USB 依存の可能性 |
