# rustcalw

OpenClaw の完全移植（TypeScript → Rust）。ミラー fork 運用。

## ブランチモデル

- `upstream` remote = `openclaw/openclaw`（読取専用、絶対に push しない）
- `origin/main` = `greenroom-lab/rustcalw` の作業ブランチ
- upstream 同期: `git fetch upstream && git merge upstream/main`
- main への固有変更（README, rust/, CLAUDE.md 等）は正常な fork 運用

## リポジトリ構造

```
rustcalw/
├── src/          # OpenClaw オリジナル (TypeScript) — 参照用
├── rust/         # Rust 移植コード
│   ├── Cargo.toml          # ワークスペースルート
│   └── crates/
│       ├── cli/            # CLI エントリポイント (wnc コマンド)
│       ├── config/         # 設定読み込み (← src/config/)
│       ├── providers/      # プロバイダーOAuth・モデル定義 (← src/providers/)
│       ├── gateway/        # ゲートウェイサーバー (← src/gateway/)
│       └── shared/         # 共有型・ユーティリティ (← src/shared/)
├── packages/     # OpenClaw オリジナル — 参照用
└── extensions/   # OpenClaw オリジナル — 参照用
```

## 開発方針

1. **完全移植**: 再実装ではなく 1:1 対応。HTTP ヘッダー、パラメータ名、デフォルト値、エラーハンドリングを全て合わせる
2. **Python 検証 → Rust 実装**: アルゴリズム理解が必要な箇所は先に Python で検証してから Rust に移す
3. **独立モジュールから段階的に**: config → shared → providers → gateway の順で移植
4. **各ステップで upstream との動作互換性を確認**
5. **プロファイル配置**: `~/.openclaw` — OpenClaw と完全同一のパス・構造

## Windows ネイティブ移植の設計原則

1. **OpenClaw 動作完全互換**: 機能削減なし。upstream の全機能を Windows でも動作させる
2. **プラットフォーム分岐の局所化**: `#[cfg]` 分岐は最小限のインターフェース層に閉じ込め、ビジネスロジックはプラットフォーム非依存に保つ
3. **段階的置換**: upstream TypeScript コードを参照しながら 1 モジュールずつ置換。途中段階で TS/Rust が共存する状態を許容

### `#[cfg]` 使用基準

| パターン | 用途 | 例 |
|---------|------|-----|
| `#[cfg(windows)]` + `#[cfg(unix)]` | IPC、シグナル、プロセス管理 | `pid_alive.rs` の `process_exists()` |
| `#[cfg(target_os = "linux")]` | /proc 依存の診断機能 | `pid_alive.rs` の `is_zombie_process()` |
| `#[cfg(not(any(unix, windows)))]` | フォールバック (常に安全なデフォルト) | `process_exists()` → `false` |

### Windows 対応の主要課題と方針

→ 詳細: [docs/windows-compat-matrix.md](docs/windows-compat-matrix.md), [ADR-002](docs/adr/ADR-002-windows-ipc.md)

| 課題 | Rust 移植方針 |
|------|--------------|
| SIGUSR1 (gateway restart) | Windows: Named Event + tokio チャネル / Unix: signal handler |
| Unix domain socket (IPC) | Windows: Named Pipe (`\\.\pipe\rustcalw-*`) / Unix: UDS |
| プロセス管理 | `pid_alive.rs` の `process_exists()` パターンを踏襲 |
| デーモン管理 | プラットフォーム trait で抽象化 |
| ファイルパーミッション | Windows: ACL API / Unix: chmod |
| パス解決 | `paths.rs` の `home_dir()` パターンを踏襲 |

## ビルド・テスト

```bash
cd rust/
cargo build        # ビルド
cargo test         # テスト
cargo run -- gateway  # gateway 起動（開発中）
```

### crate 単位の操作

```bash
cargo test -p rustcalw-shared       # shared のみテスト
cargo test -p rustcalw-config       # config のみテスト
cargo test -p rustcalw-providers    # providers のみテスト
cargo test -p rustcalw-gateway      # gateway のみテスト
cargo clippy -- -D warnings         # lint (警告をエラーとして扱う)
cargo fmt -- --check                # フォーマットチェック
```

### テスト戦略

- **colocated テスト**: `#[cfg(test)] mod tests` でモジュール内に同梱 (shared, gateway のパターン)
- **統合テスト**: `tests/` ディレクトリに配置 (config の `config_io_test.rs`, `live_config_test.rs`)
- **TS テストとの 1:1 対応**: 各 Rust テストは対応する TS テストのケースを網羅
- **Windows CI**: `cargo test` は Windows + Linux マトリックスで実行 (`.github/workflows/rust-ci.yml`)

## TS→Rust 移植ワークフロー

新モジュールを移植する際の標準手順:

1. **TS ソースを読む**: `src/<module>/<file>.ts` の実装を理解
2. **TS テストを確認**: `src/<module>/<file>.test.ts` のテストケースを全て把握
3. **Python 検証** (複雑なアルゴリズムの場合): 動作を Python スクリプトで検証してから Rust に移す
4. **Rust モジュール実装**: 対応する crate に `<module_name>.rs` を作成
5. **テスト移植**: TS テストを 1:1 で `#[cfg(test)] mod tests` に移植
6. **プラットフォーム分岐**: Windows 固有処理が必要な場合は `#[cfg(windows)]` で分岐
7. **`cargo test` で確認**: 全テスト通過を確認
8. **CLAUDE.md 更新**: モジュール対応表に追記

### 命名規則

| TS | Rust | 例 |
|----|------|-----|
| `kebab-case.ts` | `snake_case.rs` | `chat-content.ts` → `chat_content.rs` |
| `interface Foo` | `struct Foo` / `enum Foo` | 文脈に応じて選択 |
| `type Foo = ...` | `type Foo = ...` / `enum Foo` | Union → enum, Alias → type |
| `namespace/index.ts` | `mod.rs` or flat module | 浅い構造を優先 |
| `const FOO_BAR` | `const FOO_BAR` / `pub const` | visibility を意識 |

## 禁止事項・注意事項

### 変更禁止
- upstream ファイル (`src/`, `packages/`, `extensions/`, `skills/`, `apps/` 等) は変更禁止
- 変更対象は `rust/`, `CLAUDE.md`, `AGENTS.md`, `docs/adr/`, `docs/*.md` (rustcalw 固有), `.github/workflows/rust-ci.yml` のみ

### コーディング規約
- `unsafe` の使用は最小限。使用時は安全性の根拠をコメントで明記 (`pid_alive.rs` の `process_exists()` 参照)
- Unix 固有: `libc` crate を使用
- Windows 固有: `windows-sys` crate を使用 (現時点では `tasklist` コマンド呼び出しで代替可)
- `.openclaw` パス構造は upstream 完全準拠。独自パスを追加しない
- `unwrap()` / `expect()` はテストコード以外では原則禁止。`anyhow::Result` を使用
- 文字列は American English (upstream 準拠)

### upstream 同期時の注意
- `git fetch upstream && git merge upstream/main` で同期
- コンフリクト解決: upstream セクション (`src/`, `packages/` 等) は upstream 側を優先
- `rust/`, `CLAUDE.md`, `docs/adr/` はローカル側を保持
- AGENTS.md: upstream セクション (先頭) → rustcalw セクション (末尾) の順序を維持

## OpenClaw モジュール対応表

| OpenClaw (src/) | Rust (rust/crates/) | 状態 |
|-----------------|---------------------|------|
| config/         | config              | 🟢 完了 (types, paths, io, env_substitution) — テスト24件合格 |
| shared/         | shared              | 🟢 完了 (38モジュール, テスト175件合格) — global_singleton, lazy_runtime, process_scoped_map は Rust パターンで代替 |
| providers/      | providers           | 🟢 完了 (5モジュール, テスト15件合格) — OAuth/モデル定義のみ、CLIコマンド統合は cli 側で |
| cli/            | cli                 | 🟡 config サブコマンド実装済 (check, path, deploy-check) |
| gateway/        | gateway             | 🟡 Phase 2 完了 (25モジュール, テスト121件合格) — 型定義・スコープ・ロール・プロトコルフレーム・全スキーマ型・インフラ |
| channels/       | (未作成)            | ⬜ 未着手 |

### shared/ 移植済モジュール一覧

| TS モジュール | Rust モジュール | 内容 |
|--------------|----------------|------|
| requirements.ts | requirements | バイナリ/環境/OS/設定の要件評価 |
| chat-content.ts | chat_content | チャットメッセージテキスト抽出 |
| chat-envelope.ts | chat_envelope | エンベロープヘッダー除去 |
| chat-message-content.ts | chat_message_content | メッセージコンテンツ処理 |
| entry-metadata.ts | entry_metadata | 絵文字/ホームページ解決 |
| entry-status.ts | entry_status | エントリ要件評価 |
| gateway-bind-url.ts | gateway_bind_url | ゲートウェイURL解決 |
| node-list-types.ts | node_list_types | ノードリスト型定義 |
| node-list-parse.ts | node_list_parse | ノードリストJSON解析 |
| node-match.ts | node_match | ノード名/ID/IPマッチング |
| node-resolve.ts | node_resolve | ノード解決（デフォルト選択付き） |
| session-types.ts | session_types | セッション型定義 |
| string-normalization.ts | string_normalization | 文字列正規化 |
| string-sample.ts | string_sample | 文字列サンプリング・要約 |
| frontmatter.ts | frontmatter | フロントマター解析 |
| config-eval.ts | config_eval | 設定パス評価・バイナリ検索 |
| assistant-error-format.ts | assistant_error_format | APIエラー解析・UI表示 |
| assistant-identity-values.ts | assistant_identity_values | ID値バリデーション |
| model-param-b.ts | model_param_b | モデル名からBパラメータ推定 |
| subagents-format.ts | subagents_format | トークン/時間のフォーマット |
| usage-types.ts | usage_types | 使用量型定義 |
| text/code-regions.ts | text_code_regions | コードブロック領域検出 |
| text/reasoning-tags.ts | text_reasoning_tags | 推論タグ除去 |
| text/join-segments.ts | text_join_segments | テキストセグメント結合 |
| text/assistant-visible-text.ts | text_assistant_visible | 内部スキャフォールディング除去 |
| config-ui-hints-types.ts | config_ui_hints_types | UI ヒント型定義 |
| session-usage-timeseries-types.ts | session_usage_timeseries_types | 使用量タイムシリーズ型（usage_types から再エクスポート） |
| text-chunking.ts | text_chunking | テキストチャンク分割 |
| device-auth.ts | device_auth | デバイス認証型・正規化 |
| device-auth-store.ts | device_auth_store | デバイス認証ストア操作 |
| operator-scope-compat.ts | operator_scope_compat | オペレータースコープ互換性 |
| avatar-policy.ts | avatar_policy | アバターパス/URL判定 |
| usage-aggregates.ts | usage_aggregates | レイテンシ集計・使用量集約 |
| pid-alive.ts | pid_alive | プロセス生存確認 |
| tailscale-status.ts | tailscale_status | Tailscale ステータス取得 |
| net/url-userinfo.ts | url_userinfo | URL ユーザー情報除去 |
| net/ip.ts | net_ip | IP アドレス解析・分類・CIDR マッチ |
| net/ipv4.ts | net_ipv4 | IPv4 バリデーションヘルパー |

### providers/ 移植済モジュール一覧

| TS モジュール | Rust モジュール | 内容 |
|--------------|----------------|------|
| (なし — Rust 固有) | oauth_types | OAuth 資格情報型 (OAuthCredentials) |
| github-copilot-models.ts | github_copilot_models | Copilot モデルID一覧・定義ビルダー |
| github-copilot-auth.ts | github_copilot_auth | GitHub OAuth デバイスフロー (RFC 8628) |
| qwen-portal-oauth.ts | qwen_portal_oauth | Qwen (DashScope) トークンリフレッシュ |
| kilocode-shared.ts | kilocode_shared | Kilocode 定数・カタログ・モデル定義ビルダー |

### gateway/ 移植済モジュール一覧

| TS モジュール | Rust モジュール | 内容 |
|--------------|----------------|------|
| protocol/client-info.ts | client_info | クライアントID/モード定数・正規化 |
| method-scopes.ts | method_scopes | オペレータースコープ・メソッド分類・認可 |
| role-policy.ts | role_policy | ゲートウェイロール型・メソッド認可 |
| protocol/connect-error-details.ts | connect_error_details | 接続エラー詳細コード・リカバリアドバイス |
| protocol/schema/error-codes.ts | error_codes | プロトコルエラーコード・ErrorShape |
| protocol/schema/frames.ts + types.ts | protocol_frames | プロトコルフレーム型 (Request/Response/Event/ConnectParams) |
| protocol/schema/primitives.ts | schema_primitives | 基本型 (InputProvenance, SecretRef, SecretInput) |
| protocol/schema/config.ts | schema_config | 設定メソッドパラメータ・UIヒント型 |
| protocol/schema/devices.ts | schema_devices | デバイスペアリングパラメータ・イベント |
| protocol/schema/secrets.ts | schema_secrets | シークレット解決パラメータ・結果 |
| protocol/schema/push.ts | schema_push | プッシュ通知テストパラメータ |
| protocol/schema/sessions.ts | schema_sessions | セッション CRUD・パッチ・使用量パラメータ |
| protocol/schema/agent.ts | schema_agent | エージェント実行・送信・ポール・ウェイクパラメータ |
| protocol/schema/snapshot.ts | schema_snapshot | プレゼンス・スナップショット型 |
| protocol/schema/logs-chat.ts | schema_logs_chat | ログ・チャット送受信パラメータ |
| protocol/schema/wizard.ts | schema_wizard | ウィザードフロー型 |
| protocol/schema/agents-models-skills.ts | schema_agents_models_skills | エージェントCRUD・モデル一覧・スキル・ツールカタログ |
| protocol/schema/channels.ts | schema_channels | Talk/TTS・チャネルステータス型 |
| protocol/schema/nodes.ts | schema_nodes | ノードペアリング・invoke・pending work |
| protocol/schema/exec-approvals.ts | schema_exec_approvals | 実行承認ポリシー・リクエスト/解決型 |
| protocol/schema/cron.ts | schema_cron | Cronジョブ型 (スケジュール・ペイロード・配信・ログ) |
| server/close-reason.ts | close_reason | WebSocketクローズ理由の切り詰め |
| server-constants.ts | server_constants | サーバー定数 (ペイロード上限, タイムアウト等) |
| security-path.ts | security_path | URLパス正規化・セキュリティ検査 |
| origin-check.ts | origin_check | ブラウザOriginヘッダー検査 |

## 関連

- [greenroom-lab/hare](https://github.com/greenroom-lab/hare) — rustcalw の成果を取り込むハレ暫定ランタイム
- 現行ランタイム: `~/.cargo/bin/wnc.exe`（win-native-claw 由来、暫定ビルド）
