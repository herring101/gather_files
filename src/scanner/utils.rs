// src/scanner/utils.rs

use globset::{Glob, GlobSet, GlobSetBuilder};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// 簡易バイナリ判定
pub fn is_binary_file(path: &Path) -> bool {
    if let Ok(mut f) = File::open(path) {
        let mut buffer = [0; 1024];
        if let Ok(n) = f.read(&mut buffer) {
            let non_text_count = buffer[..n]
                .iter()
                .filter(|&&b| b == 0 || (b < 7 || b == 127))
                .count();
            // 1/8 以上が非テキストならバイナリ判定
            return non_text_count > (n / 8);
        }
    }
    false
}

/// GlobSetを構築
pub fn build_globset(patterns: &[String]) -> Option<GlobSet> {
    if patterns.is_empty() {
        return None;
    }
    let mut builder = GlobSetBuilder::new();
    for pat in patterns {
        // パターンの標準化
        let expanded = if pat.ends_with('/') {
            format!("{}**", pat) // ディレクトリパターンの場合
        } else if pat.starts_with('.')
            && pat.len() > 1
            && pat.chars().skip(1).all(|c| !c.is_whitespace() && c != '/')
        {
            // 拡張子のみの指定の場合（例: .py）
            format!("**/*{}", pat)
        } else if !pat.contains('/') && !pat.contains('*') {
            // ファイル名のみの場合
            format!("**/{}", pat)
        } else {
            // その他のパターン（すでに**やワイルドカードが含まれている場合はそのまま）
            pat.clone()
        };

        eprintln!("Processing pattern: '{}' -> '{}'", pat, expanded);

        match Glob::new(&expanded) {
            Ok(g) => {
                builder.add(g);
            }
            Err(e) => {
                eprintln!("Warning: invalid glob pattern '{}': {}", pat, e);
            }
        }
    }
    match builder.build() {
        Ok(gs) => Some(gs),
        Err(e) => {
            eprintln!("Error building globset: {}", e);
            None
        }
    }
}
