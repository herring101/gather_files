# gather_files

gather_files は、プロジェクトのソースコードを LLM（Large Language Model）に理解させやすい形式で収集するツールです。  
**CLI コマンドは v0.3 から `gather` に変更されました。**  
**v0.3.1 以降では “本文を省略したファイル” をツリー上に `[omitted:<reason>]` と表記し、ファイル内容は出力されません。**

---

## 目的と特徴

- **LLM との効率的な対話**
  - プロジェクト全体を一度に LLM に理解させることができます
  - 必要なファイルを適切な順序で収集し、フォーマットします
  - **v0.3.1**: 省略したファイルはツリーに `[omitted:binary]` などと明示
- **賢い収集と除外**
  - `.gitignore` と統合して不要ファイルを自動除外
  - カスタマイズ可能な除外 / 省略パターン
  - バイナリや巨大ファイルを自動スキップし、ツリーに注釈
- **使いやすさを重視**
  - シンプルなコマンドですぐに使い始められます
  - **自己アップデート (`self-update`)** で常に最新版を利用
  - 初回実行時にはインテリジェントな設定ガイドを表示
  - VS Code との統合を予定

---

## インストール

### Cargo でインストール

```bash
cargo install --git https://github.com/herring101/gather_files --bin gather --force
```

> **すでにインストール済みの場合**
>
> - バージョンを固定しない（`--tag` を付けない）
> - `--force` を付与する  
>   だけで再ビルドされ最新版が導入されます。  
>   **もっと簡単に**: 後述の `gather self-update` を推奨します。

### スクリプトでインストール

#### Windows

```powershell
irm https://raw.githubusercontent.com/herring101/gather_files/main/install.ps1 | iex
```

#### macOS / Linux

```bash
curl -fsSL https://raw.githubusercontent.com/herring101/gather_files/main/install.sh | sh
```

> スクリプトは GitHub Releases の **最新タグ** を取得し、既存バイナリを置き換えます。  
> 再実行するだけでアップデートになります。

### アップデートだけしたい場合

```bash
gather self-update
```

---

## 基本的な使い方

### クイックスタート

```bash
# カレントディレクトリのコードを収集
gather .

# 特定ディレクトリのコードを収集
gather /path/to/project
```

### 出力フォーマット概要

```text
project-root/
    src/
        main.rs
        utils.rs
    target/                  [omitted:known-large-dir]
    binary/logo.png          [omitted:binary]
    README.md
```

- `[...]` が付いた行は **ツリーにだけ現れ、本文ブロックは出力されません**。
- `<reason>` には `binary` / `too-large` / `pattern` などが入ります。

### コマンド

| コマンド                 | 説明                       |
| ------------------------ | -------------------------- |
| `gather <DIR> [OPTIONS]` | 指定ディレクトリを収集     |
| `gather self-update`     | 実行バイナリを最新版へ更新 |

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
| --use-gitignore    | なし   | `.gitignore` の内容を `[exclude]` に統合 | false             |
| --timestamp        | なし   | 出力ファイル名にタイムスタンプを付与     | false             |
| --no-open          | なし   | VS Code での自動オープンを無効化         | false             |

> `self-update` はサブコマンドなのでオプションではありません。

---

## アップデート運用例

```bash
# 作業前に最新版を取得
gather self-update

# Cargo からアップデートする場合
cargo install --git https://github.com/herring101/gather_files --bin gather --force
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

[skip]              # 内容だけスキップ
*.min.js
*.pdf

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

## 開発ロードマップ

今後の計画は **[`docs/ROADMAP.md`](./docs/ROADMAP.md)** にまとめています。  
新機能の提案やアイデアはぜひ Issue / Discussion でお寄せください。

---

## コントリビュート

詳細な貢献方法は **[`CONTRIBUTING.md`](./CONTRIBUTING.md)** を参照してください。

---

## 変更履歴

最新版の変更点は **[`CHANGELOG.md`](./CHANGELOG.md)** を確認してください。

---

## 作者

**herring101**

- GitHub: [@herring101](https://github.com/herring101)
- Twitter / X: [@herring101](https://twitter.com/herring101426)

ツールに関するご意見・フィードバックをお待ちしています！
