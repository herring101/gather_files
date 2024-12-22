# Gather Files

ファイルを再帰的に収集してテキスト出力するツールです。

## 特徴

- ディレクトリを再帰的に走査してファイル内容を収集
- 設定ファイル(.gather)でカスタマイズ可能
- バイナリファイルの自動スキップ
- 柔軟な除外パターン設定
- タイムスタンプ付きの出力オプション

## インストール

### Option 1: cargoを使用してインストール

```bash
cargo install --git https://github.com/herring101/gather_files
```

### Option 2: インストールスクリプトを使用

#### Windows
```powershell
irm https://raw.githubusercontent.com/herring101/gather_files/main/install.ps1 | iex
```

#### Linux/macOS
```bash
curl -sSL https://raw.githubusercontent.com/herring101/gather_files/main/install.sh | sh
```

### Option 3: バイナリを直接ダウンロード

[GitHubリリースページ](https://github.com/herring101/gather_files/releases/latest)から、お使いのプラットフォーム向けのバイナリをダウンロードしてください。

## 使用方法

基本的な使用方法:

```bash
gather_files <ディレクトリパス>
```

オプション:
- `--output`, `-o`: 出力ファイルのパスを指定
- `--config-file`, `-c`: 設定ファイルのパスを指定
- `--max-lines`, `-m`: 各ファイルから読み込む最大行数
- `--max-file-size`: 指定サイズを超えるファイルをスキップ
- `--patterns`, `-p`: 追加の除外パターン
- `--skip-patterns`, `-s`: 追加のスキップパターン
- `--include-extensions`, `-i`: 含める拡張子を指定
- `--timestamp`: 出力ファイル名にタイムスタンプを付与
- `--no-open`: .gatherを自動でVS Codeで開かない

### 設定ファイル (.gather)

```ini
[settings]
max_lines=1000
max_file_size=500000
skip_binary=yes
output_dir=gather

[exclude]
.git
gather
.gather

[skip]
*.pdf

[include]
# 拡張子を指定（未指定の場合はすべて含む）
# .py
```

## ライセンス

MIT License

## 貢献

Issue、Pull Requestは大歓迎です！