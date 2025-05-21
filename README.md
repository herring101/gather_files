# gather_files

_gather_files_ は、プロジェクトのソースコードを **LLM（Large Language Model）** に理解させやすい形で収集・整形する CLI ツールです

> v0.3 からコマンド名は **`gather`** にリネームされました。
>
> **v0.4.0‑alpha 以降** では **アウトライン抽出モード**(`--mode outline`) を新たに搭載しています。

---

## 特徴

| 機能カテゴリ                          | 概要                                                                                                                                                             |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **ファイル収集 (gather モード)**      | `.gitignore` & 独自設定 `.gather` を組み合わせ、必要ファイルだけを再帰的に収集・整形します。スキップされたファイルはツリー上に `[omitted:<reason>]` として注釈。 |
| **アウトライン抽出 (outline モード)** | **NEW!** Rust ファイル (`.rs`) から公開シンボル (`pub struct` / `fn` など) を抽出し、Markdown または JSON で一覧を生成します。今後多言語対応予定。               |
| **自己アップデート**                  | `gather self-update` で GitHub Releases から最新バイナリをダウンロードし実行ファイルを置換。                                                                     |
| **インストールスクリプト**            | macOS / Linux / Windows 用のワンライナーを同梱。                                                                                                                 |

---

## インストール

### Cargo でインストール

```bash
cargo install --git https://github.com/herring101/gather_files --bin gather --force
```

### スクリプトでインストール

- **Windows**
  ```powershell
  irm https://raw.githubusercontent.com/herring101/gather_files/main/install.ps1 | iex
  ```
- **macOS / Linux**
  ```bash
  curl -fsSL https://raw.githubusercontent.com/herring101/gather_files/main/install.sh | sh
  ```

---

## クイックスタート

### 1) プロジェクトを丸ごと収集

```bash
# gather モード（既定）
gather .
```

### 2) Rust 公開シンボルのアウトラインだけ欲しい

```bash
# outline モード（Markdown 出力）
gather --mode outline .

# JSON 形式で出力したい場合
gather --mode outline --outline-format json .
```

生成されたファイルは `gather/output.txt`（または `output_<timestamp>.txt`）に保存され、
`code` コマンドが存在すれば VS Code で自動的に開きます。

---

## コマンドラインオプション（抜粋）

| オプション               | 短縮形 | モード  | 説明                              | 既定値              |
| ------------------------ | ------ | ------- | --------------------------------- | ------------------- |
| `--mode <MODE>`          | なし   | 共通    | `gather` / `outline` を切替       | `gather`            |
| `--outline-format <FMT>` | なし   | outline | `md` / `json` を選択              | `md`                |
| `--output <FILE>`        | `-o`   | gather  | 出力ファイルパス                  | `gather/output.txt` |
| `--max-lines <N>`        | `-m`   | gather  | 各ファイル読み込み上限行          | 1000                |
| `--use-gitignore`        | なし   | gather  | `.gitignore` を除外パターンに統合 | false               |

> そのほかのフラグは `gather --help` を参照してください。

---

## アウトライン出力例（Markdown）

```markdown
### src/lib.rs

- **mod** scanner
- **struct** GatherOptions
- **fn** gather

### src/main.rs

- **fn** main
```

JSON 形式を選んだ場合はファイルごとにオブジェクトが 1 行ずつ並びます。

```json
{"file":"src/lib.rs","symbols":[{"kind":"mod","ident":"scanner"}, ...]}
```

---

## 設定ファイル (.gather)

プロジェクトルートに **`.gather`** ファイルを置くことで挙動を詳細に制御できます。  
初回実行時に自動生成されるテンプレートを編集するか、以下を参考にカスタマイズしてください。

### セクション一覧

| セクション   | 役割                                                 |
| ------------ | ---------------------------------------------------- |
| `[settings]` | キー=値 形式で一般設定を記述                         |
| `[exclude]`  | 収集対象から**完全に除外**するパス／パターン         |
| `[skip]`     | **ツリーにだけ残し、本文を省略**するファイルパターン |
| `[outline]`  | 本文の代わりにアウトラインを出力するパターン         |
| `[include]`  | 収集対象に明示的に含めたいパターン                   |

```ini
[settings]
max_lines         = 1000       # 各ファイルの最大読み込み行
max_file_size     = 500000     # スキップ閾値 (bytes)
skip_binary       = yes
output_dir        = gather
use_timestamp     = no
open_output       = yes
use_gitignore     = yes

[exclude]           # 除外パターン
node_modules/
*.log

[skip]              # 本文を省略しツリーにのみ残すパターン
*.min.js
*.pdf

[outline]           # 本文の代わりにアウトラインを出力
*.rs

[include]           # 必ず含めたいパターン
*.rs
src/**/*.py
```

> **パターン記法メモ**
>
> - ディレクトリは末尾 `/` を付けると配下すべてを対象
> - 拡張子 `.rs` のみを指定すると `**/*.rs` に展開
> - `**`, `*`, 中括弧展開など一般的な glob が使用可能

---

## 変更履歴

最新版の変更点は **[`CHANGELOG.md`](./CHANGELOG.md)** を参照してください。

---

## 今後のロードマップ

- outline プラグインの多言語対応 (Python / Markdown / TS)
- 依存グラフ出力 (`--mode graph` 予定)
- VS Code 拡張プレビュー

詳細は **[`docs/ROADMAP.md`](./docs/ROADMAP.md)** をご覧ください。

---

## 作者

**herring101**  
GitHub: <https://github.com/herring101>
Twitter: <https://twitter.com/herring101>

ツールに関するご意見・フィードバックをお待ちしています！
