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

## ビルド・テスト

```bash
cd rust/
cargo build        # ビルド
cargo test         # テスト
cargo run -- gateway  # gateway 起動（開発中）
```

## OpenClaw モジュール対応表

| OpenClaw (src/) | Rust (rust/crates/) | 状態 |
|-----------------|---------------------|------|
| config/         | config              | 🟡 型定義完了 (types.*, paths) — IO未着手 |
| shared/         | shared              | 🟡 コア型・ユーティリティ実装済 (requirements, chat_*, entry_metadata, gateway_bind_url, node_list_types, session_types, string_normalization) |
| cli/            | cli                 | 🔧 スキャフォールド |
| gateway/        | (未作成)            | ⬜ 未着手 |
| providers/      | (未作成)            | ⬜ 未着手 |
| channels/       | (未作成)            | ⬜ 未着手 |

## 関連

- [greenroom-lab/hare](https://github.com/greenroom-lab/hare) — rustcalw の成果を取り込むハレ暫定ランタイム
- 現行ランタイム: `~/.cargo/bin/wnc.exe`（win-native-claw 由来、暫定ビルド）
