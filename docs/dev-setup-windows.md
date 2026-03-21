# Windows 開発環境セットアップ

rustcalw の開発に必要な環境構築手順 (Windows 11)。

## 前提条件

- Windows 11 (Windows 10 1903+ でも動作するが推奨は 11)
- Git for Windows (bash 同梱)
- 管理者権限 (Visual Studio Build Tools インストール時)

## 1. Rust ツールチェーン

```powershell
# rustup インストール (未インストールの場合)
winget install Rustlang.Rustup

# ツールチェーン設定
rustup default stable-x86_64-pc-windows-msvc
rustup component add clippy rustfmt
```

### Visual Studio Build Tools

Rust の MSVC ターゲットには C++ ビルドツールが必要:

```powershell
# winget でインストール
winget install Microsoft.VisualStudio.2022.BuildTools

# または Visual Studio Installer で以下のワークロードを選択:
# - "C++ によるデスクトップ開発" (Desktop development with C++)
```

## 2. リポジトリクローン

```bash
git clone https://github.com/greenroom-lab/rustcalw.git
cd rustcalw

# upstream リモート追加
git remote add upstream https://github.com/openclaw/openclaw.git
git fetch upstream
```

## 3. 初回ビルド・テスト

```bash
cd rust
cargo build        # 全 crate ビルド
cargo test         # 全テスト実行
cargo clippy -- -D warnings  # lint チェック
```

正常にビルド・テストが通れば環境構築完了。

## 4. (オプション) Node.js / pnpm — upstream TS テスト実行用

upstream の TypeScript コードを参照・テスト実行する場合:

```powershell
# Node.js 22+ インストール
winget install OpenJS.NodeJS.LTS

# pnpm インストール
corepack enable
corepack prepare pnpm@latest --activate

# 依存インストール
cd rustcalw
pnpm install
```

## 5. エディタ設定

### VS Code (推奨)

必須拡張:
- `rust-analyzer` — Rust 言語サポート
- `Even Better TOML` — Cargo.toml 編集

推奨拡張:
- `Error Lens` — インラインエラー表示
- `CodeLLDB` — Rust デバッガ

### settings.json 推奨設定

```json
{
  "rust-analyzer.cargo.buildScripts.enable": true,
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.linkedProjects": ["rust/Cargo.toml"]
}
```

## 6. 環境変数

| 変数 | 用途 | 推奨値 |
|------|------|--------|
| `RUST_LOG` | ログレベル | `debug` (開発時) |
| `RUST_BACKTRACE` | バックトレース | `1` (開発時) |
| `CARGO_TARGET_DIR` | ビルド出力先 | (デフォルトの `rust/target` で OK) |

## 7. よくある問題

### `link.exe` が見つからない
→ Visual Studio Build Tools の "C++ によるデスクトップ開発" ワークロードをインストールしてください。

### `cargo test` でテストがタイムアウトする
→ Windows Defender のリアルタイムスキャンが `target/` ディレクトリをスキャンしている可能性。除外設定を推奨:
```powershell
Add-MpPreference -ExclusionPath "C:\Users\<user>\rustcalw\rust\target"
```

### upstream merge 時のコンフリクト
→ `rust/`, `CLAUDE.md`, `docs/adr/` はローカル側を優先。それ以外は upstream を優先。

## 8. 日常の開発フロー

```bash
# 1. upstream 同期
git fetch upstream
git merge upstream/main

# 2. Rust コード編集
cd rust

# 3. ビルド・テスト
cargo build
cargo test -p rustcalw-<crate名>  # crate 単位テスト

# 4. lint・フォーマット
cargo clippy -- -D warnings
cargo fmt

# 5. コミット
git add rust/ CLAUDE.md docs/
git commit -m "feat(rust): <変更内容>"
```
