# ADR-001: ミラー戦略と段階的移植アプローチ

- **状態**: 承認
- **日付**: 2026-03-21
- **関連**: [ADR-004](ADR-004-crate-structure.md), [ADR-005](ADR-005-test-strategy.md)

## コンテキスト

- OpenClaw (openclaw/openclaw) は急速に開発が進む大規模 TypeScript プロジェクト (326k+ スター)
- Windows ネイティブ対応のため Rust への段階的移植が必要
- upstream の更新を継続的に取り込みつつ、独自の Rust 実装を進める必要がある
- fork と独立リポジトリの2つの選択肢がある

## 決定

- GitHub fork ではなく独立リポジトリとしてミラー運用
- `upstream` remote = `openclaw/openclaw` (読取専用)
- `origin/main` = 作業ブランチ
- `rust/` ディレクトリに Rust コードを隔離し、upstream コードと共存
- 移植順序: config → shared → providers → gateway → channels → agents
- 各フェーズで upstream との動作互換性を確認

## 根拠

- fork だと GitHub UI 上で upstream との差分が大きくなり管理困難
- 独立リポジトリなら Dependabot や CI を自由に設定可能
- `rust/` 隔離により upstream merge 時のコンフリクトを最小化
- 移植順序は依存関係チェーン (shared → config → providers → gateway) に沿う

## 影響

- upstream 同期は手動 (`git fetch upstream && git merge upstream/main`)
- CLAUDE.md, AGENTS.md は upstream とコンフリクトするため merge 時に注意が必要
- Dependabot が upstream の依存関係更新 PR を自動生成する

### 現在の移植状態 (2026-03-20)

- config: 完了 (types, paths, io, env_substitution) — テスト24件
- shared: 完了 (38モジュール) — テスト175件
- providers: 完了 (5モジュール) — テスト15件
- gateway: Phase 2 完了 (25モジュール) — テスト121件
- cli: config サブコマンド実装済
- channels: 未着手
