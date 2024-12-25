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
        let expanded = if pat.ends_with('/') {
            format!("{}**", pat)
        } else {
            pat.clone()
        };
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
