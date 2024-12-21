# gather_files

`gather_files` は、指定ディレクトリを再帰的に走査して、ディレクトリツリーおよびファイル内容をテキストとしてまとめるツールです。  
バイナリファイルのスキップ、拡張子フィルタ、ファイルサイズ上限、フォルダごと除外など、柔軟な除外設定が可能です。

## 特徴

1. **ディレクトリ全体のツリー構造を表示** (Markdown風)
2. **除外フォルダ**/ファイル を設定して、表示や出力から完全に除外可能
3. **ファイル内容スキップ** (globパターンにマッチするファイルの本文は (略) と表示)
4. **最大行数 (`max_lines`)** / **最大ファイルサイズ (`max_file_size`)** 指定で大きなファイルを一部だけ表示
5. **`.gather` (独自フォーマット) および CLI オプションによる柔軟な設定**  

## インストール

1. Rust 環境がある場合  
   ```bash
   git clone https://github.com/yourname/gather_files.git
   cd gather_files
   cargo build --release
   ```
   すると、`target/release/gather_files` というバイナリが生成されます。

2. そのバイナリを `$PATH` の通った場所へコピーするなどして利用可能  
   ```bash
   cp target/release/gather_files ~/.local/bin/
   # or
   sudo cp target/release/gather_files /usr/local/bin/gather
   ```

## 使い方

```bash
gather <TARGET_DIR> [OPTIONS]
```

**主なオプション:**

- `-o, --output <PATH>`: 出力ファイルまたはディレクトリ  
  - ディレクトリを指定した場合、`gather_{timestamp}.txt` というファイル名で作成します  
- `-c, --config-file <FILE>`: `.gather` (設定ファイル) のパス  
- `-m, --max-lines <N>`: 各ファイルで表示する最大行数  
- `--max-file-size <BYTES>`: これを超えたファイルは内容スキップ  
- `-p, --patterns <PATTERN>`: 除外パターン (1パターンにつき1回指定)  
  - `target/` と書けば、そのディレクトリ以下を再帰的に除外  
- `-s, --skip-patterns <PATTERN>`: 内容スキップパターン (本文を(略)にする)  
- `-i, --include-extensions <EXT>`: 表示したい拡張子 (例: `.rs`)  
- `--init-config`: 実行ディレクトリ（または `TARGET_DIR`）にサンプル `.gather` を生成

### `.gather` ファイル

独自フォーマットの例:  

```ini
[settings]
max_lines=1000
max_file_size=500000
skip_binary=yes
output_dir=out

[exclude]
.git
target/
*.log

[skip]
*.md
*.pdf

[include]
.rs
.py
```

- `[settings]`  
  - `max_lines`: ファイル本文の最大行数  
  - `max_file_size`: このサイズを超えたファイルは本文スキップ  
  - `skip_binary`: `yes/true/1` でバイナリをスキップ  
  - `output_dir`: 出力先ディレクトリ (CLIの `-o` がなければ、ここに `gather_{timestamp}.txt` を作成)  

- `[exclude]`  
  - 縦に1行ずつ除外したいパターンを書く  
  - `target/` のように末尾 `/` を付ければそのフォルダ以下を再帰的に除外  

- `[skip]`  
  - 縦に1行ずつ内容スキップパターンを書く (本文は(略)表示)  

- `[include]`  
  - 縦に1行ずつ拡張子を書く (例: `.rs`)  
  - 空の場合は全拡張子が対象  

### 例コマンド

```bash
# 1) デフォルトの .gather を生成する
gather <TARGET_DIR> --init-config

# 2) ターゲットを解析 (設定ファイルがあれば読み込む)
gather <TARGET_DIR>

# 3) 出力先ディレクトリを指定
gather <TARGET_DIR> -o results_dir

# 4) 一部オプションをCLIで上書き/追加
gather <TARGET_DIR> \
  -m 300 \
  -p target/ \
  -s "*.md" \
  -i ".rs"
```

## 除外の仕組み

- `target/` というフォルダを `[exclude]` や CLI の `--patterns target/` で指定すると、そのディレクトリやファイルはディレクトリツリーにも表示されません。  
- バイナリファイル (`skip_binary`) やファイルサイズ (`max_file_size`) などの判定も自動で行い、それらは内容をスキップします。

## ライセンスなど

(プロジェクトのライセンスや作者情報を追記)