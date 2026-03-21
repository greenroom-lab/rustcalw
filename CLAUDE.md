# rustcalw

OpenClaw の Windows ネイティブ化。fork 運用。

## 最優先目標

**OpenClaw (TypeScript) を Windows 上でビルド・起動・動作確認できる状態にすること。**

Rust 移植はその後。まず TS 版が Windows で完全に動く状態を確立し、E2E テストで疎通を確認する。その E2E テストを維持しながらモジュール単位で Rust に差し替えていく。

### フェーズ

1. **Windows ビルド対応** — TS 版 OpenClaw を Windows でビルド・起動できるようにする（最小限のパッチ）
2. **E2E テスト確立** — 実際の openclaw 構成で gateway を起動し、リクエスト→応答を検証するテストスイート
3. **段階的 Rust 化** — E2E テスト疎通を維持しながらモジュール単位で Rust に差し替え

### 現状 (Phase 1 完了・Phase 2 進行中)

- ✅ Phase 1: `pnpm build` が Windows で成功（symlink→コピー/junction 修正済み）
- ✅ Phase 1: gateway が Windows で起動・応答確認済み（`node openclaw.mjs gateway --dev --allow-unconfigured`）
- ✅ Phase 2: E2E スモークテスト (`test/windows-gateway-smoke.e2e.test.ts`) 3/3 通過
- ✅ Phase 2: 既存 gateway テスト 140/157 ファイル通過 (1542/1587 テスト)
- 🔧 Phase 2: 残り 17 テストファイルの Windows 対応（session/plugin 関連）
- Rust 側は型定義・ユニットテストのみ（サーバー実装なし）

## ブランチモデル

- `upstream` remote = `openclaw/openclaw`（読取専用、絶対に push しない）
- `origin/main` = `greenroom-lab/rustcalw` の作業ブランチ
- upstream 同期: `git fetch upstream && git merge upstream/main`
- main への固有変更（README, rust/, CLAUDE.md 等）は正常な fork 運用

## リポジトリ構造

```
rustcalw/
├── src/          # OpenClaw オリジナル (TypeScript) — Windows 対応パッチの対象
├── scripts/      # ビルドスクリプト — Windows 互換性修正の対象
├── packages/     # OpenClaw パッケージ
├── extensions/   # OpenClaw 拡張
├── rust/         # Rust 移植コード (Phase 3 以降で使用)
│   └── crates/   # cli, config, shared, providers, gateway
├── package.json  # pnpm ワークスペース
└── openclaw.mjs  # CLI エントリポイント (Node.js >= 22.12)
```

## OpenClaw (TS) ビルド・起動

```bash
# 依存インストール
pnpm install

# ビルド (Windows 対応済み)
pnpm build

# 開発モード起動
pnpm dev

# gateway 起動 (dev モード)
node openclaw.mjs gateway --dev --allow-unconfigured

# gateway 起動 (設定済み)
node openclaw.mjs gateway
```

### Windows ビルド対応 (修正済み)

- `scripts/stage-bundled-plugin-runtime.mjs` の symlink を Windows 互換に修正
  - ファイル symlink → `fs.copyFileSync` にフォールバック
  - ディレクトリ symlink → 絶対パス junction に変更
  - 既存 symlink のコピー → `fs.realpathSync` + copy にフォールバック

### E2E テスト

```bash
# Windows スモークテスト (gateway 起動→health→status→presence)
pnpm exec vitest run test/windows-gateway-smoke.e2e.test.ts --config vitest.e2e.config.ts

# gateway テスト全体
pnpm test:gateway

# E2E テスト全体
pnpm test:e2e
```

## Rust 移植コード

Phase 3 以降で使用。現時点では型定義とユニットテスト。

```bash
cd rust/
cargo build        # ビルド
cargo test         # ユニットテスト (347件)
cargo clippy -- -D warnings
cargo fmt -- --check
```

### Rust 移植済みモジュール

| OpenClaw (src/) | Rust (rust/crates/) | 内容 |
|-----------------|---------------------|------|
| config/         | config              | 設定読み込み (types, paths, io, env_substitution) |
| shared/         | shared              | 共有ユーティリティ (38モジュール) |
| providers/      | providers           | OAuth・モデル定義 (5モジュール) |
| gateway/        | gateway             | 型定義・スキーマのみ (25モジュール、サーバー実装なし) |
| cli/            | cli                 | config サブコマンドのみ (check, path, deploy-check) |

## upstream 変更方針

- Windows 対応のための最小限の変更は許容する（ビルドスクリプト修正等）
- 変更は Windows 互換性に必要なものに限定し、オリジナルの動作を壊さない
- upstream 同期時のコンフリクト解決: 固有変更箇所はローカル保持、それ以外は upstream 優先

## コーディング規約 (Rust)

- `unsafe` は最小限、安全性の根拠をコメントで明記
- `unwrap()` / `expect()` はテストコード以外では原則禁止、`anyhow::Result` を使用
- `.openclaw` パス構造は upstream 完全準拠
- 文字列は American English (upstream 準拠)

## 関連

- [greenroom-lab/hare](https://github.com/greenroom-lab/hare) — 暫定ランタイム (将来 rustcalw の成果を取り込み)
