# Changelog (index)

| バージョン系列 | ファイル                                         |
| -------------- | ------------------------------------------------ |
| **0.3.x**      | 現在のファイル（このページ）                     |
| **0.2.x**      | [docs/changes/0.2.x.md](./docs/changes/0.2.x.md) |

> 旧来の CHANGELOG はマイナー系列ごとに `docs/changes/` 配下へ移動しました。  
> 過去分はそちらを参照してください。

---

## [v0.4.0] – 2025-04-30

### ✨ Added

- **アウトライン抽出モード**
  - `--mode outline` で Rust ファイルから公開シンボルを一覧化
  - `--outline-format md|json` で Markdown / JSON 出力を選択
- **プラグイン設計** – `outline::provider::OutlineProvider` トレイトを追加  
  今後の多言語対応を見据えた拡張ポイントを用意

### ♻️ Changed

- **モジュール分割**
  - 旧 `lib.rs` の gather ロジックを `src/gather.rs` へ移動
  - `src/outline/` を新設し、Rust 実装を `rust.rs` に実装
  - `lib.rs` は薄いファサードに整理
- README をアウトライン機能を含む最新版へ更新

### ✅ Tests / Coverage

- `outline::rust` のユニットテストを追加
- E2E テスト `tests/outline_cli.rs` を新規作成
- ラインカバレッジ 80 % 以上を維持

---

## [v0.3.2] – 2025-04-29

### 🛠 Fixed

- `.gitignore` にディレクトリ名だけ書いた場合に配下が除外されないバグを修正

### ✅ Tests / Coverage

- `utils` / `walker` にテスト追加、ラインカバレッジ 80 %+ に到達

---

## [v0.3.1] – 2025-04-29

### ✨ Added

- **ツリー表示の `[omitted:<reason>]` ラベル**
  - 内容を省略したファイル／ディレクトリをツリー上で一目で判別可能に
  - 省略理由: `binary`, `too-large`, `pattern`
- **スキャン出力のスリム化**
  - 省略ファイルの “(略)” ブロックを完全に削除
  - トークン消費を大幅削減

### ♻️ Changed

- `scanner::run` ロジックをリファクタ
  - 2 パス構成で _omitted map_ を生成 → ツリーに注釈付与
  - `ProcessCounter` のカウント名称を整理（機能は互換）
- バージョンを **0.3.1** へ更新

### 🛠 Fixed

- `.gather` `[skip]` パターンが空でも “(略)” が出力される冗長動作を解消

---

## [v0.3.0] – 2025-??-??

### ✨ Added

- **コマンド名リネーム**: `gather_files` → **`gather`**
  - `Cargo.toml` で `[[bin]] name = "gather"`
  - 旧バイナリ名は配布終了（インストールスクリプトは互換シンリンクを作成）
- **ドキュメント更新**
  - README / CONTRIBUTING / tests など、全コマンド例を `gather` に置換
- **リリースワークフロー**
  - アセット名を `gather-<platform>` に刷新
  - 自己アップデート先も新バイナリを指すよう修正

### ♻️ Changed

- `src/updater.rs` の `bin_name` とターゲット文字列を変更
- install スクリプト (`install.sh`, `install.ps1`) を新ファイル名へ更新
- `release.yml` の Upload-Asset ステップを修正

### 🛠 Fixed

- YAML 的に誤っていた `env:` 行をブロック形式へ修正

### ⚠️ Migration Notes

- 旧コマンド `gather_files` は v0.2 系までの互換です。  
  v0.3 以降は **`gather`** をご利用ください。
- Cargo での再インストール例
  ```bash
  cargo install --git https://github.com/herring101/gather_files --bin gather --force
  ```
