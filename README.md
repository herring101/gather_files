# gather_files

ディレクトリ内のファイルを再帰的に収集し、Markdownフォーマットでテキスト出力するツールです。

## インストール方法

### Windows

```powershell
irm https://raw.githubusercontent.com/herring101/gather_files/main/install.ps1 | iex
```

### macOS / Linux

```bash
curl -sSL https://raw.githubusercontent.com/herring101/gather_files/main/install.sh | sh
```

## 使用方法

基本的な使用方法:

```bash
gather_files <対象ディレクトリ>
```

オプション付きの例:

```bash
gather_files . --max-lines 500 --timestamp --patterns "*.log" --skip-patterns "secret.txt"
```

## コマンドラインオプション

| オプション | 短縮形 | 説明 | デフォルト値 |
|------------|--------|------|--------------|
| --output | -o | 出力ファイルのパス | gather/output.txt |
| --config-file | -c | 設定ファイルのパス | .gather |
| --max-lines | -m | 各ファイルから読み込む最大行数 | 1000 |
| --max-file-size | なし | スキップするファイルサイズ閾値（バイト） | なし |
| --patterns | -p | 追加の除外パターン（複数指定可） | なし |
| --skip-patterns | -s | 追加の内容スキップパターン（複数指定可） | なし |
| --include-extensions | -i | 含める拡張子（複数指定可） | なし |
| --timestamp | なし | 出力ファイル名にタイムスタンプを付与 | false |
| --no-open | なし | .gatherファイルを自動でVSCodeで開かない | false |

## 設定ファイル (.gather)

プロジェクトルートに`.gather`ファイルを配置することで、デフォルトの動作をカスタマイズできます。
ファイルが存在しない場合は自動的に作成されます。

設定ファイルの例:

```ini
[settings]
# 各ファイルから読み込む最大行数
max_lines=1000

# このサイズ(バイト)を超えるファイルをスキップ
max_file_size=500000

# バイナリファイルをスキップするか
skip_binary=yes

# 出力先ディレクトリ
output_dir=gather

# 出力ファイル名にタイムスタンプを付与するか
use_timestamp=no

# .gatherファイルを自動でVSCodeで開くか
open_in_vscode=no

[exclude]
# 除外するファイル/ディレクトリパターン
.git
gather
.gather
*.log

[skip]
# 内容をスキップするファイルパターン
*.pdf
secret.txt

[include]
# 含める拡張子（指定がない場合はすべて含む）
.rs
.py
```

### 設定ファイルのオプション説明

#### [settings]セクション

| 設定キー | 説明 | デフォルト値 |
|----------|------|--------------|
| max_lines | 各ファイルから読み込む最大行数 | 1000 |
| max_file_size | スキップするファイルサイズ閾値（バイト） | なし |
| skip_binary | バイナリファイルをスキップするか | false |
| output_dir | 出力先ディレクトリ | gather |
| use_timestamp | 出力ファイル名にタイムスタンプを付与するか | false |
| open_in_vscode | .gatherファイルを自動でVSCodeで開くか | false |

#### [exclude]セクション
除外するファイルやディレクトリのパターンを指定します。
ディレクトリの場合は末尾に`/`を付けることで、そのディレクトリ以下をすべて除外します。

#### [skip]セクション
内容の出力をスキップするファイルパターンを指定します。
マッチしたファイルは`(略)`として出力されます。

#### [include]セクション
含める拡張子を指定します。
指定がない場合は、すべての拡張子が対象となります。

## パターンの書き方

ファイルパターンはグロブ形式で指定します：

- `*` : 任意の文字列
- `?` : 任意の1文字
- `[abc]` : a, b, c のいずれかの1文字
- `[!abc]` : a, b, c 以外の1文字

例：
- `*.txt` : すべてのtxtファイル
- `secret_*.log` : secret_ で始まるすべてのlogファイル
- `logs/` : logsディレクトリ以下をすべて

## 出力例

````markdown
```
/
    src/
        main.rs
        lib.rs
    tests/
        test_main.rs
    Cargo.toml
    README.md
```

### src/main.rs
```rust
fn main() {
    println!("Hello, world!");
}
```

### src/lib.rs
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
````

## 注意事項

- バイナリファイルは自動的に検出され、`skip_binary=yes`の場合はスキップされます
- 大きなファイルは`max_file_size`の設定に基づいてスキップされます
- 出力ファイルは常にUTF-8でエンコードされます