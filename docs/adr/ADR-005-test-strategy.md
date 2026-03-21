# ADR-005: テスト戦略 — TS テストとの 1:1 対応

- **状態**: 承認
- **日付**: 2026-03-21
- **関連**: [ADR-001](ADR-001-mirror-strategy.md), [ADR-004](ADR-004-crate-structure.md)

## コンテキスト

- OpenClaw は Vitest で広範なテストスイートを持つ (V8 coverage 70%)
- Rust 移植の正確性は、TS テストケースの完全再現で担保する必要がある
- テストの粒度と配置パターンを統一する必要がある

## 決定

### テスト配置パターン

1. **colocated テスト** (デフォルト): `#[cfg(test)] mod tests` でソースファイル末尾に配置
   - 対象: 単一モジュールのユニットテスト
   - 例: `shared/src/chat_content.rs`, `gateway/src/error_codes.rs`

2. **統合テスト**: `crate/tests/*.rs` に配置
   - 対象: 複数モジュールにまたがるテスト、外部ファイルを使うテスト
   - 例: `config/tests/config_io_test.rs`, `config/tests/live_config_test.rs`

3. **テストフィクスチャ**: `crate/tests/fixtures/` に配置
   - 例: `config/tests/fixtures/full.json`, `config/tests/fixtures/minimal.json`

### TS テストとの対応ルール

- 各 Rust テスト関数名は TS テストの `it("description")` を snake_case に変換
- テストケースの入力値と期待値は TS テストから忠実にコピー
- TS テストが `describe` でグループ化されている場合、Rust では `mod` でネストするか、テスト関数名にプレフィックスを付与
- プラットフォーム固有のテストは `#[cfg(windows)]` / `#[cfg(unix)]` で分岐

### CI での実行

- `.github/workflows/rust-ci.yml` で Windows + Linux マトリックス
- `cargo test` で全テスト実行
- `cargo clippy -- -D warnings` で lint
- `cargo fmt -- --check` でフォーマット

### テスト数の追跡

- CLAUDE.md のモジュール対応表でテスト数を記録
- 現在の合計: 335件 (shared 175 + config 24 + providers 15 + gateway 121)

## 根拠

- 1:1 対応により、移植の正確性を機械的に検証可能
- colocated テストは Rust の標準慣行であり、モジュールと一緒に管理しやすい
- Windows + Linux マトリックスにより、プラットフォーム分岐のリグレッションを早期検出

## 影響

- 新モジュール移植時は必ず対応する TS テストを確認し、全ケースを移植する義務がある
- テスト数は品質指標の一つとして CLAUDE.md で追跡する
- TS 側でテストが追加された場合、対応する Rust テストも追加する
