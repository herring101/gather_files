# Contributing to gather_files

## ブランチ戦略

このプロジェクトでは以下のブランチ構成を採用しています：

- `main`: 安定版のコード（リリース用）
- `develop`: 開発版のコード（次期リリースの準備用）
- `feature/*`: 新機能開発用
- `fix/*`: バグ修正用

## 開発の流れ

1. Issueの作成
   - バグ報告や機能要望は、まずIssueを作成してください
   - 既存のIssueがないか確認してください

2. ブランチの作成
   - 新機能開発: `feature/機能名`
   - バグ修正: `fix/バグ内容`
   - developブランチから分岐してください

3. 開発とコミット
   - コミットメッセージは具体的に
   - テストを追加してください
   - `cargo fmt`と`cargo clippy`を実行してください

4. プルリクエスト
   - developブランチに向けてPRを作成
   - PRの説明には関連するIssue番号を含めてください
   - CIが通過することを確認してください

5. レビュー
   - レビューのコメントに対応してください
   - 必要に応じて変更を加えてください

6. マージ
   - レビューが承認されたらマージされます
   - マージ後、feature/fixブランチは削除されます

## リリースプロセス

1. developブランチでの準備
   - バージョン番号の更新
   - CHANGELOGの更新

2. mainブランチへのマージ
   - developからmainへのPR作成
   - 最終確認とマージ

3. タグ付けとリリース
   - バージョンタグの作成
   - GitHub Releaseの作成

## 開発環境のセットアップ

```bash
# リポジトリのクローン
git clone https://github.com/herring101/gather_files
cd gather_files

# 開発ブランチの作成
git checkout -b feature/your-feature develop

# 依存関係のインストール
cargo build

# テストの実行
cargo test

# フォーマットの確認
cargo fmt -- --check

# Lintの実行
cargo clippy
```

## ヘルプが必要な場合

質問や支援が必要な場合は、お気軽にIssueを作成してください。
