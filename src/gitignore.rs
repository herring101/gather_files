// src/gitignore.rs

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

/// .gitignoreファイルを解析して除外パターンのリストを返す
pub fn parse_gitignore(path: &Path) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut patterns = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        // 空行やコメントをスキップ
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // パターンを標準化
        let pattern = normalize_pattern(trimmed);
        patterns.push(pattern);
    }

    Ok(patterns)
}

/// gitignoreパターンを[exclude]セクション用に標準化
fn normalize_pattern(pattern: &str) -> String {
    let mut normalized = pattern.to_string();

    // 先頭の'/'を除去（相対パスに変換）
    if normalized.starts_with('/') {
        normalized = normalized[1..].to_string();
    }

    // 末尾の'/'はそのまま（ディレクトリ判定に使用）
    // '**'パターンもそのまま（globsetが対応）

    // '!'（否定）で始まるパターンは現時点ではサポートしない
    if normalized.starts_with('!') {
        return String::new();
    }

    normalized
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_normalize_pattern() {
        assert_eq!(normalize_pattern("/node_modules"), "node_modules");
        assert_eq!(normalize_pattern("*.log"), "*.log");
        assert_eq!(normalize_pattern("dist/"), "dist/");
        assert_eq!(normalize_pattern("**/*.tmp"), "**/*.tmp");
        assert_eq!(normalize_pattern("!temp"), ""); // 否定パターンは空文字列に
    }

    #[test]
    fn test_parse_gitignore() -> io::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join(".gitignore");
        let mut file = File::create(&file_path)?;

        writeln!(file, "# Node.js")?;
        writeln!(file, "/node_modules")?;
        writeln!(file, "*.log")?;
        writeln!(file, "")?;
        writeln!(file, "dist/")?;
        writeln!(file, "# 否定パターン")?;
        writeln!(file, "!keep.log")?;

        let patterns = parse_gitignore(&file_path)?;

        // パターンを表示して確認
        println!("Detected patterns: {:?}", patterns);

        // 期待されるパターン
        let expected: Vec<String> = vec![
            "node_modules".to_string(),
            "*.log".to_string(),
            "dist/".to_string(),
        ];

        // 空でないパターンのみをフィルタリング
        let filtered_patterns: Vec<String> =
            patterns.into_iter().filter(|p| !p.is_empty()).collect();

        assert_eq!(filtered_patterns, expected);

        Ok(())
    }
}
