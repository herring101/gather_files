# Changelog

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