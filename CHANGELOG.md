# Changelog (index)

| バージョン系列 | ファイル                                         |
| -------------- | ------------------------------------------------ |
| **0.3.x**      | 現在のファイル（このページ）                     |
| **0.2.x**      | [docs/changes/0.2.x.md](./docs/changes/0.2.x.md) |

> 旧来の CHANGELOG はマイナー系列ごとに `docs/changes/` 配下へ移動しました。  
> 過去分はそちらを参照してください。

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
