# gather_files

gather_files は、プロジェクトのソースコードを LLM（Large Language Model）に理解させやすい形式で収集するツールです。

## 目的と特徴

- **LLM との効率的な対話**

  - プロジェクト全体を一度に LLM に理解させることができます
  - 必要なファイルを適切な順序で収集し、フォーマットします
  - 余分なファイルを自動的に除外し、トークン消費を最適化します

- **賢い収集と除外**

  - .gitignore との統合で、不要なファイルを自動除外
  - カスタマイズ可能な除外パターン
  - バイナリファイルの自動スキップ
  - 大規模ディレクトリの自動検出と除外提案

- **使いやすさを重視**
  - シンプルなコマンドで即座に利用開始
  - **自己アップデート (`self-update`)** で常に最新機能を利用
  - 初回実行時のインテリジェントな設定ガイド
  - プロジェクトタイプに応じた設定テンプレート（近日実装予定）
  - VS Code との統合

## インストール

### Cargo でインストール

```bash
# 最新版をインストール / 既存インストールを強制上書き
cargo install --git https://github.com/herring101/gather_files --force
```

> **すでにインストール済みの場合**
>
> - バージョンを固定しない (`--tag` を付けない)
> - `--force` を付与する  
>   ことで再ビルドされ最新版が導入されます。  
>   **もっと簡単に**: 後述の `gather_files self-update` を推奨します。

### スクリプトでインストール

#### Windows

```powershell
irm https://raw.githubusercontent.com/herring101/gather_files/main/install.ps1 | iex
```

#### macOS / Linux

```bash
curl -fsSL https://raw.githubusercontent.com/herring101/gather_files/main/install.sh | sh
```

> スクリプトは毎回 GitHub Releases の **最新タグ** を取得し、既存バイナリを上書きします。  
> したがって再実行するだけでアップデートにもなります。

### アップデートだけしたい場合

```bash
gather_files self-update
```

## 基本的な使い方

### クイックスタート

```bash
# カレントディレクトリのコードを収集
gather_files .

# 特定のディレクトリのコードを収集
gather_files /path/to/project
```

### コマンド

| コマンド                       | 説明                       |
| ------------------------------ | -------------------------- |
| `gather_files <DIR> [OPTIONS]` | 指定ディレクトリを収集     |
| `gather_files self-update`     | 実行ファイルを最新版へ更新 |

### コマンドラインオプション

| オプション         | 短縮形 | 説明                                     | デフォルト値      |
| ------------------ | ------ | ---------------------------------------- | ----------------- |
| --output           | -o     | 出力ファイルのパス                       | gather/output.txt |
| --config-file      | -c     | 設定ファイルのパス                       | .gather           |
| --max-lines        | -m     | 各ファイルから読み込む最大行数           | 1000              |
| --max-file-size    | なし   | スキップするファイルサイズ閾値（バイト） | なし              |
| --patterns         | -p     | 追加の除外パターン（複数指定可）         | なし              |
| --skip-patterns    | -s     | 追加の内容スキップパターン（複数指定可） | なし              |
| --include-patterns | -i     | 含めるファイルパターン（複数指定可）     | なし              |
| --use-gitignore    | なし   | .gitignore の内容を[exclude]に統合       | false             |
| --timestamp        | なし   | 出力ファイル名にタイムスタンプを付与     | false             |
| --no-open          | なし   | VS Code での自動オープンを無効化         | false             |

（※ `self-update` はサブコマンド扱いなので、オプションではありません）

## アップデートの運用例

```bash
# プロジェクトで作業前に最新版へ
gather_files self-update

# もし Cargo インストールを使い続ける場合
cargo install --git https://github.com/herring101/gather_files --force
```

## 設定ファイル (.gather)

プロジェクトルートに **`.gather`** ファイルを置くことで挙動を細かく制御できます。  
初回実行時に自動生成されるテンプレートを編集するか、以下を参考にカスタマイズしてください。

### セクション一覧

| セクション   | 役割                                   |
| ------------ | -------------------------------------- |
| `[settings]` | キー=値形式で一般設定を記述            |
| `[exclude]`  | 収集対象から除外するパス／パターン     |
| `[skip]`     | 内容をスキップしたいファイルのパターン |
| `[include]`  | 収集対象に明示的に含めたいパターン     |

```ini
[settings]
max_lines         = 1000     # 各ファイルの最大読み込み行
max_file_size     = 500000   # スキップ閾値 (bytes)
skip_binary       = yes
output_dir        = gather
use_timestamp     = no
use_gitignore     = yes
open_output       = yes

[exclude]         # 除外パターン
node_modules/
*.log

[skip]            # 内容だけスキップ
*.min.js
*.pdf

[include]         # 必ず含めたいパターン
*.rs
src/**/*.py
```

> **パターン記法メモ**  
> ・ディレクトリは末尾 `/` を付けると配下すべてを対象  
> ・拡張子だけ指定すると `**/*.ext` に展開  
> ・`**`, ワイルドカード, 中括弧展開など一般的な glob が使えます

---

## 開発ロードマップ

将来計画は **[docs/ROADMAP.md`](./docs/ROADMAP.md)** に切り出しました。新機能の提案やアイデアはそちらを参照してください。

---

## コントリビュート

貢献方法の詳細は **[CONTRIBUTING.md](./CONTRIBUTING.md)** をご覧ください。

---

## 変更履歴

最新版の変更点は [CHANGELOG.md](./CHANGELOG.md) を参照してください。

---

## 作者

**herring101**

- GitHub: [@herring101](https://github.com/herring101)
- Twitter / X: [@herring101](https://twitter.com/herring101426)

ツールに関するご意見・フィードバックをお待ちしています！
