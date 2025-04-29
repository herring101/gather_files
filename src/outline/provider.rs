//! src/outline/provider.rs
//!
//! Outline 用共通インターフェース。

use std::path::Path;

/// 抽出されたシンボル 1 件
#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: String,
    pub ident: String,
}

/// 言語ごとのアウトライン抽出器トレイト
pub trait OutlineProvider {
    /// このファイル拡張子をサポートするか
    fn supports(path: &Path) -> bool
    where
        Self: Sized;

    /// ソース文字列から公開シンボルを抽出
    fn extract(path: &Path, src: &str) -> anyhow::Result<Vec<Symbol>>;
}
