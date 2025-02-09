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

- **使いやすさを重視**
  - シンプルなコマンドで即座に利用開始
  - プロジェクトタイプに応じた設定テンプレート（近日実装予定）
  - VS Code との統合

## インストール

### Cargo を使用したインストール

```bash
# GitHubから直接インストール
cargo install --git https://github.com/herring101/gather_files

# 特定のバージョンを指定してインストール
cargo install --git https://github.com/herring101/gather_files --tag v0.2.0
```

### スクリプトを使用したインストール

#### Windows

```powershell
# PowerShell を管理者権限で実行
irm https://raw.githubusercontent.com/herring101/gather_files/main/install.ps1 | iex
```

#### macOS / Linux

```bash
curl -fsSL https://raw.githubusercontent.com/herring101/gather_files/main/install.sh | sh
```

## 基本的な使い方

### クイックスタート

```bash
# カレントディレクトリのコードを収集
gather_files .

# 特定のディレクトリのコードを収集
gather_files /path/to/project
```

### コマンドラインオプション

| オプション           | 短縮形 | 説明                                     | デフォルト値      |
| -------------------- | ------ | ---------------------------------------- | ----------------- |
| --output             | -o     | 出力ファイルのパス                       | gather/output.txt |
| --config-file        | -c     | 設定ファイルのパス                       | .gather           |
| --max-lines          | -m     | 各ファイルから読み込む最大行数           | 1000              |
| --max-file-size      | なし   | スキップするファイルサイズ閾値（バイト） | なし              |
| --patterns           | -p     | 追加の除外パターン（複数指定可）         | なし              |
| --skip-patterns      | -s     | 追加の内容スキップパターン（複数指定可） | なし              |
| --include-extensions | -i     | 含める拡張子（複数指定可）               | なし              |
| --use-gitignore      | なし   | .gitignore の内容を[exclude]に統合       | false             |
| --timestamp          | なし   | 出力ファイル名にタイムスタンプを付与     | false             |
| --no-open            | なし   | VS Code での自動オープンを無効化         | false             |

### 使用例

```bash
# 出力先を指定して最大行数を制限
gather_files . -o output.txt --max-lines 500

# .gitignoreを使用し、特定の拡張子のみを含める
gather_files . --use-gitignore -i .rs -i .toml

# カスタム除外パターンを追加
gather_files . -p "*.tmp" -p "build/"
```

## 設定ファイル (.gather)

プロジェクトルートに`.gather`ファイルを配置することで、収集の設定をカスタマイズできます：

### 設定ファイルのオプション説明

#### [settings]セクション

| 設定キー      | 説明                                       | デフォルト値 |
| ------------- | ------------------------------------------ | ------------ |
| max_lines     | 各ファイルから読み込む最大行数             | 1000         |
| max_file_size | スキップするファイルサイズ閾値（バイト）   | なし         |
| skip_binary   | バイナリファイルをスキップするか           | false        |
| output_dir    | 出力先ディレクトリ                         | gather       |
| use_timestamp | 出力ファイル名にタイムスタンプを付与するか | false        |
| use_gitignore | .gitignore の内容を[exclude]に統合するか   | false        |
| open_output   | VSCode で出力ファイルを自動で開くか        | true         |

#### [exclude]セクション

除外するファイルやディレクトリのパターンを指定します。
ディレクトリの場合は末尾に`/`を付けることで、そのディレクトリ以下をすべて除外します。

```ini
[exclude]
.git/          # .gitディレクトリ以下をすべて除外
node_modules/  # node_modulesディレクトリ以下をすべて除外
*.log         # すべてのlogファイルを除外
temp_*        # temp_で始まるすべてのファイルを除外
```

#### [skip]セクション

内容の出力をスキップするファイルパターンを指定します。
マッチしたファイルは`(略)`として出力されます。

```ini
[skip]
*.pdf       # PDFファイルの内容をスキップ
*.min.js    # 圧縮済みJavaScriptファイルをスキップ
```

#### [include]セクション

含める拡張子を指定します。
指定がない場合は、すべての拡張子が対象となります。

```ini
[include]
.rs         # Rustファイル
.py         # Pythonファイル
.js         # JavaScriptファイル
```

### パターンの書き方

ファイルパターンはグロブ形式で指定します：

- `*` : 任意の文字列
- `?` : 任意の 1 文字
- `[abc]` : a, b, c のいずれかの 1 文字
- `[!abc]` : a, b, c 以外の 1 文字

例：

- `*.txt` : すべての txt ファイル
- `secret_*.log` : secret\_ で始まるすべての log ファイル
- `logs/` : logs ディレクトリ以下をすべて
- `src/**/*.rs` : src ディレクトリ以下のすべての Rust ファイル

````

## LLMとの使用例

1. コードの収集
```bash
cd your-project
gather .
````

2. 生成されたファイル（例：gather/output.txt）を LLM に送信

3. プロジェクトの文脈を理解した LLM と対話
   - バグ修正の提案
   - リファクタリングの提案
   - 新機能の実装方法の相談
   - コードレビュー

## 開発ロードマップ

### 近日実装予定の機能

- プロジェクトタイプの自動検出
- 言語/フレームワーク別の最適化テンプレート
- トークン数の最適化機能
- 差分モード（git diff ベース）
- プリセット管理システム

### 長期的な目標

- LLM プロバイダーとの直接統合
- IDE/エディタプラグイン
- 依存関係グラフの生成
- インタラクティブモード
- プロジェクト分析レポート

## コントリビュート

プルリクエストや課題の報告を歓迎します。以下の分野で特に協力を求めています：

- 新しいプロジェクトテンプレートの追加
- パフォーマンスの最適化
- テストの追加
- ドキュメントの改善

## ライセンス

MIT

## 作者

@herring101

## 更新履歴

詳細な更新履歴は[CHANGELOG.md](./CHANGELOG.md)を参照してください。
