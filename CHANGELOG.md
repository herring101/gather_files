# Changelog

## [v0.2.6] - 2025-03-06

### Fixed
- GLIBC互換性問題の修正
  - Linuxバイナリを MUSLベースに変更し、幅広い互換性を確保
  - WSL環境を含む様々なLinuxディストリビューションでの動作を改善
- バージョン表示の修正
  - CLIバージョン表示をCargo.tomlから自動取得するように変更
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
- CLIバージョン表示の修正
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
- .gatherテンプレートの改善
  - より詳細な使用例とコメントを追加

[v0.2.2]: https://github.com/herring101/gather_files/compare/v0.2.1...v0.2.2

## [v0.2.1] - 2024-12-25

### Changed
- ファイル処理情報の改善
  - 処理済み/スキップファイルの詳細なカウント表示
  - 処理サマリーの追加（合計、スキップ理由等）
- コードベースの改善
  - scanner.rsの整理（モジュール分割）
  - コード品質の向上

[v0.2.1]: https://github.com/herring101/gather_files/compare/v0.2.0...v0.2.1

## [v0.2.0] - 2024-12-23

### Added
- .gitignoreの内容を[exclude]セクションに統合する機能
  - `--use-gitignore`フラグでCLIから制御可能
  - .gatherファイルの`use_gitignore`設定で制御可能
  - 重複パターンは自動的に除外

### Changed
- .gatherファイルのデフォルトテンプレートを更新
  - `use_gitignore=yes`をデフォルト設定として追加

## [v0.1.0] - 2024-12-22

### Added
- 基本的なファイル収集機能
  - 再帰的なディレクトリ走査
  - ファイル内容の収集
- 設定ファイル(.gather)サポート
  - max_lines制限
  - max_file_size制限
  - バイナリファイルのスキップ
  - 除外パターン([exclude])
  - 内容スキップパターン([skip])
  - 含める拡張子([include])
- CLIオプション
  - カスタム出力先(`--output`)
  - タイムスタンプ付きファイル名(`--timestamp`)
  - VS Codeで自動で開く(`--no-open`で無効化)
- クロスプラットフォーム対応
  - Windows, macOS, Linux用のバイナリ
  - プラットフォーム固有のインストールスクリプト

[v0.2.0]: https://github.com/herring101/gather_files/compare/v0.1.0...v0.2.0
[v0.1.0]: https://github.com/herring101/gather_files/releases/tag/v0.1.0