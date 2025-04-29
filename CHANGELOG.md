# Changelog

## [v0.2.8] - 2025-04-29

### Fixed

- **directory‐exclude bug**  
  パターン末尾に `/` が付いたディレクトリ（例: `gather/`）が  
  ツリー表示に残ってしまう問題を修正しました。  
  いまは `**/dir`, `dir/**`, `**/dir/**` の 3 種類を内部展開して
  ディレクトリ自体／その配下すべてを完全に除外します。

### Added

- **ユニットテスト強化**
  - `args`, `config`, `utils`, `walker` にテストを追加
  - CI カバレッジ向上
- **コードベースのリファクタ**
  - `args.rs` を _clap derive_ に刷新
  - `config.rs`・`walker.rs` を簡潔化
  - 不要な `mut`, 重複ロジックを削除

[v0.2.8]: https://github.com/herring101/gather_files/compare/v0.2.7...v0.2.8

## [v0.2.7] - 2025-03-06

### Added

- Enhanced first-run experience
  - First-run now prompts users to review configuration after creating the settings file
  - Added automatic detection of large directories
  - Auto-excludes common large directories like node_modules and venv

### Changed

- Extended configuration file options
  - Added first_run_completed flag
  - Added max_files_per_dir threshold setting
  - Added max_auto_file_size threshold setting

[v0.2.7]: https://github.com/herring101/gather_files/compare/v0.2.6...v0.2.7

## [v0.2.6] - 2025-03-06

### Fixed

- GLIBC 互換性問題の修正
  - Linux バイナリを MUSL ベースに変更し、幅広い互換性を確保
  - WSL 環境を含む様々な Linux ディストリビューションでの動作を改善
- バージョン表示の修正
  - CLI バージョン表示を Cargo.toml から自動取得するように変更
  - バージョン番号のハードコードを排除

[v0.2.6]: https://github.com/herring101/gather_files/compare/v0.2.5...v0.2.6

## [v0.2.5] - 2025-03-06

### Fixed

- [include]パターン使用時のディレクトリツリー表示を改善
  - マッチするファイルの親フォルダ階層が表示されるように修正
  - 例: `*tts*.py`のようなパターンを指定した場合でも、ファイルの親フォルダ構造が表示される

[v0.2.5]: https://github.com/herring101/gather_files/compare/v0.2.4...v0.2.5

## [v0.2.4] - 2025-03-04

### Fixed

- [include]セクションの問題修正
  - ディレクトリツリー出力に対しても[include]パターンを適用
  - 拡張子だけの指定（例：`.py`）を`**/*.py`に変換
  - 単純なファイル名パターンの扱いを改善
- CLI バージョン表示の修正
  - コマンド名を`gather_files`に統一
  - バージョン表示を正確に反映

[v0.2.4]: https://github.com/herring101/gather_files/compare/v0.2.2...v0.2.4

## [v0.2.2] - 2025-03-04

### Changed

- [include]セクションの機能強化
  - 拡張子だけでなく、様々なグロブパターンをサポート
  - 例
    - `*.md`
    - `src/**/*.rs`
    - `*.{js,ts}`
- .gather テンプレートの改善
  - より詳細な使用例とコメントを追加

[v0.2.2]: https://github.com/herring101/gather_files/compare/v0.2.1...v0.2.2

## [v0.2.1] - 2024-12-25

### Changed

- ファイル処理情報の改善
  - 処理済み/スキップファイルの詳細なカウント表示
  - 処理サマリーの追加（合計、スキップ理由等）
- コードベースの改善
  - scanner.rs の整理（モジュール分割）
  - コード品質の向上

[v0.2.1]: https://github.com/herring101/gather_files/compare/v0.2.0...v0.2.1

## [v0.2.0] - 2024-12-23

### Added

- .gitignore の内容を[exclude]セクションに統合する機能
  - `--use-gitignore`フラグで CLI から制御可能
  - .gather ファイルの`use_gitignore`設定で制御可能
  - 重複パターンは自動的に除外

### Changed

- .gather ファイルのデフォルトテンプレートを更新
  - `use_gitignore=yes`をデフォルト設定として追加

## [v0.1.0] - 2024-12-22

### Added

- 基本的なファイル収集機能
  - 再帰的なディレクトリ走査
  - ファイル内容の収集
- 設定ファイル(.gather)サポート
  - max_lines 制限
  - max_file_size 制限
  - バイナリファイルのスキップ
  - 除外パターン([exclude])
  - 内容スキップパターン([skip])
  - 含める拡張子([include])
- CLI オプション
  - カスタム出力先(`--output`)
  - タイムスタンプ付きファイル名(`--timestamp`)
  - VS Code で自動で開く(`--no-open`で無効化)
- クロスプラットフォーム対応
  - Windows, macOS, Linux 用のバイナリ
  - プラットフォーム固有のインストールスクリプト

[v0.2.0]: https://github.com/herring101/gather_files/compare/v0.1.0...v0.2.0
[v0.1.0]: https://github.com/herring101/gather_files/releases/tag/v0.1.0
