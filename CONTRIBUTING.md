# Contributing to gather_files

## ブランチ戦略

このプロジェクトでは以下のブランチ構成を採用しています：

- `main`: 安定版のコード（リリース用）
- `feature/*`: 新機能開発用
- `fix/*`: バグ修正用

## 開発の流れ

1. Issue の作成

   - バグ報告や機能要望は、まず Issue を作成してください
   - 既存の Issue がないか確認してください

2. ブランチの作成

   - 新機能開発: `feature/機能名`
   - バグ修正: `fix/バグ内容`
   - main ブランチから分岐してください

3. 開発とコミット

   - コミットメッセージは具体的に
   - テストを追加してください
   - `cargo fmt`と`cargo clippy`を実行してください

4. プルリクエスト

   - main ブランチに向けて PR を作成
   - PR の説明には関連する Issue 番号を含めてください
   - CI が通過することを確認してください

5. レビュー

   - レビューのコメントに対応してください
   - 必要に応じて変更を加えてください

6. マージ
   - レビューが承認されたらマージされます
   - マージ後、feature/fix ブランチは削除されます

## リリースプロセス

1. リリース準備

   - バージョン番号の更新
   - CHANGELOG の更新
   - main ブランチで直接行うか、PR を通して行います

2. タグ付けとリリース
   - バージョンタグの作成 (`git tag -a vX.Y.Z -m "Version X.Y.Z - 説明")`
   - タグをプッシュ (`git push origin vX.Y.Z`)
   - GitHub Actions によって自動的にリリースが作成されます

## 開発環境のセットアップ

```bash
# リポジトリのクローン
git clone https://github.com/herring101/gather_files
cd gather_files

# 開発ブランチの作成
git checkout -b feature/your-feature main

# 依存関係のインストール
cargo build

# テストの実行
cargo test

# フォーマットの修正＋確認
cargo fmt --all
cargo fmt -- --check

# Lintの実行
cargo clippy
```

## ヘルプが必要な場合

質問や支援が必要な場合は、お気軽に Issue を作成してください。
