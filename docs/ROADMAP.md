# gather_files – 開発ロードマップ（ドラフト）

---

## 1. アウトライン / グラフ

### 1.1 アウトライン抽出

| 言語                              | 粒度                                       | 優先度 |
| --------------------------------- | ------------------------------------------ | ------ |
| Rust                              | `pub` item                                 | ★★★    |
| Python                            | `class` / `def`                            | ★★★    |
| Markdown                          | 見出し                                     | ★★★    |
| JavaScript / TypeScript (Node.js) | `export` / `import` / top‑level `function` | ★★☆    |
| C#                                | `namespace` / `class` / `method`           | ★☆☆    |

- 共通トレイト `OutlineProvider` を定義し、多言語実装をプラグイン式で追加
- Tree‑sitter (多言語) + `syn` (Rust) ベースで解析
- CLI: `--mode outline` / `--outline-format md|json`

### 1.2 依存グラフ

| 機能                                       | 優先度 |
| ------------------------------------------ | ------ |
| ファイルレベル依存グラフ（.dot 出力）      | ★★★    |
| `graph.command` フックで外部ツール呼び出し | ★★☆    |
| Rust コールグラフ (rust‑analyzer 呼び出し) | ★☆☆    |

---

## 2. UX / CLI 改善

- 進捗表示を **indicatif** に置換
- `--preset <lang>` で .gather テンプレ適用
- `clap_complete` によるシェル補完スクリプト生成

---

## 3. パフォーマンス & 安定性

- `rayon` で並列読み込み
- 巨大バッファを避けて逐次書き込み
- `.gather` パーサを手書き → `serde` TOML へ

---

## 4. エコシステム連携

| 機能                                                                                      | 優先度 |
| ----------------------------------------------------------------------------------------- | ------ |
| **VS Code 拡張** – コマンドパレットから gather を実行し、アウトライン／グラフをプレビュー | ★★☆    |
| **OpenAI ファイル API 直接アップロード** – 収集後に `files.create` までワンストップ       | ★★☆    |
| **Neovim プラグイン** – `:GatherFiles` ラッパを Lua で実装                                | ★☆☆    |

---

## 運用ルール

1. **実装完了** した項目は CHANGELOG.md へ移動。
2. ROADMAP.md には「今後やること」だけを残す。
3. 詳細タスクは GitHub Issue に `roadmap:` ラベルで管理。

---

> 🚩 **フィードバック歓迎！**
> 気になる項目があれば Issue / PR / Discussion でご連絡ください。
