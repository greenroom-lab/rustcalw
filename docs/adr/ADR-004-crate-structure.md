# ADR-004: Rust crate 構成と依存関係戦略

- **状態**: 承認
- **日付**: 2026-03-21
- **関連**: [ADR-001](ADR-001-mirror-strategy.md), [ADR-005](ADR-005-test-strategy.md)

## コンテキスト

- OpenClaw の TypeScript コードは `src/` 配下に 50+ ディレクトリのフラットな構造
- Rust では適切な crate 分割がコンパイル時間とモジュール性に影響
- 外部依存関係の選定は長期メンテナンスに直結

## 決定

### crate 構成

```
rust/
├── Cargo.toml (workspace root)
└── crates/
    ├── shared/     ← src/shared/ の 1:1 移植
    ├── config/     ← src/config/ の移植 (shared に依存)
    ├── providers/  ← src/providers/ の OAuth・モデル定義 (config に依存)
    ├── gateway/    ← src/gateway/ の移植 (shared に依存)
    └── cli/        ← src/cli/ + src/commands/ のエントリポイント (全 crate に依存)
```

### 依存関係グラフ

```
cli ──→ config ──→ shared
 │         ↑
 ├──→ gateway ──→ shared
 │
 └──→ providers ──→ config
```

### workspace dependencies による統一管理

- 全 crate で使う依存は `[workspace.dependencies]` で一元管理
- 現在の外部依存: anyhow, clap (derive), regex, serde (+derive), serde_json, serde_yaml, tokio (full), tracing, tracing-subscriber (env-filter), tempfile

### edition 2024 の選択

- 最新の Rust エディションを使用し、新しい言語機能を活用
- `gen` キーワード予約等の将来互換性を確保

## 根拠

- TypeScript の `src/<module>/` を Rust の `crates/<module>/` に 1:1 対応させることで、移植の追跡が容易
- workspace dependencies パターンにより依存バージョンの不一致を防止
- crate 分割は OpenClaw のモジュール境界に合わせることで、将来の機能追加時に自然に拡張可能

## 影響

- 新しい OpenClaw モジュール領域を移植する際は、既存 crate への追加か新 crate 作成かを判断する必要がある
- 目安: 10+ モジュールかつ独立した関心事 → 新 crate、それ以下 → 既存 crate に追加
- 将来追加が予想される crate: `channels`, `agents`, `plugins`, `daemon`, `platform` (抽象化レイヤー)
