use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::{DirEntry, WalkDir};

use crate::model::ConfigParams;

/// 簡易バイナリ判定
fn is_binary_file(path: &Path) -> bool {
    use std::io::Read;
    if let Ok(mut f) = File::open(path) {
        let mut buffer = [0; 1024];
        if let Ok(n) = f.read(&mut buffer) {
            let non_text_count = buffer[..n]
                .iter()
                .filter(|&&b| b == 0 || (b < 7 || b == 127))
                .count();
            if non_text_count > (n / 8) {
                return true;
            }
        }
    }
    false
}

/// build_globset
/// - 末尾が '/' のパターンは自動的に `/**` を付与し、再帰的に除外
fn build_globset(patterns: &[String]) -> Option<GlobSet> {
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

/// ディレクトリエントリが excludeパターンに合致するかどうかを判定し、
/// 合致するならこのエントリ以下の再帰をスキップする。
fn should_skip_dir(entry: &DirEntry, target_dir: &Path, exclude_globset: &Option<GlobSet>) -> bool {
    if let Some(gs) = exclude_globset {
        let rel = match entry.path().strip_prefix(target_dir) {
            Ok(r) => r,
            Err(_) => entry.path(),
        };
        let rel_str = rel.to_string_lossy();
        // ディレクトリパスが excludeパターンに合致すれば skip
        if gs.is_match(&*rel_str) {
            return true;
        }
    }
    false
}

/// メインの走査
pub fn run(
    target_dir: &Path,
    output_file: &Path,
    config: &ConfigParams,
    _cli_include_exts: &[String],
) -> Result<(), String> {
    // 1) 出力ファイルオープン
    let mut outfile = File::create(output_file).map_err(|e| {
        format!(
            "出力ファイルを作成できません: {} - {}",
            output_file.display(),
            e
        )
    })?;

    // 2) exclude / skip_content 用の GlobSet
    let exclude_globset = build_globset(&config.exclude_patterns);
    let skip_globset = build_globset(&config.skip_content_patterns);

    // 3) include_exts
    let all_exts = &config.include_exts;

    // --------------------------------------------------
    // (1) ディレクトリツリーの出力
    // --------------------------------------------------

    // WalkDir に filter_entry() を使い、
    // 「exclude パターンに当たるディレクトリなら再帰しない」ようにする
    writeln!(outfile, "```").ok();

    let walker1 = WalkDir::new(target_dir).into_iter().filter_entry(|entry| {
        if entry.file_type().is_dir() {
            // excludeパターンにマッチするディレクトリなら false を返して再帰しない
            !should_skip_dir(entry, target_dir, &exclude_globset)
        } else {
            true
        }
    });

    for entry in walker1 {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("WalkDir error: {}", e);
                continue;
            }
        };
        let path = entry.path();
        let rel = match path.strip_prefix(target_dir) {
            Ok(r) => r,
            Err(_) => path,
        };

        // ディレクトリ表示
        if path.is_dir() {
            let level = rel.components().count();
            let indent = "    ".repeat(level.saturating_sub(1));
            let name = path
                .file_name()
                .map(|s| s.to_string_lossy())
                .unwrap_or_default();
            writeln!(outfile, "{}{}/", indent, name).ok();
        } else {
            // ファイル
            let rel_str = rel.to_string_lossy();

            // excludeパターンにファイル自体がマッチするかどうか
            if let Some(gs) = &exclude_globset {
                if gs.is_match(&*rel_str) {
                    continue;
                }
            }
            // include_exts
            if !all_exts.is_empty() {
                let ext = path
                    .extension()
                    .map(|x| format!(".{}", x.to_string_lossy()))
                    .unwrap_or_default();
                if !all_exts.contains(&ext) {
                    continue;
                }
            }

            let level = rel.components().count();
            let indent = "    ".repeat(level.saturating_sub(1));
            let name = path
                .file_name()
                .map(|s| s.to_string_lossy())
                .unwrap_or_default();
            writeln!(outfile, "{}{}", indent, name).ok();
        }
    }

    writeln!(outfile, "```").ok();
    writeln!(outfile).ok();

    // --------------------------------------------------
    // (2) ファイル内容の出力
    // --------------------------------------------------

    // ふたたび filter_entry() を使い、excludeディレクトリを再帰しない
    let mut all_files = Vec::new();
    let walker2 = WalkDir::new(target_dir).into_iter().filter_entry(|entry| {
        if entry.file_type().is_dir() {
            !should_skip_dir(entry, target_dir, &exclude_globset)
        } else {
            true
        }
    });

    for entry in walker2 {
        if let Ok(e) = entry {
            if e.path().is_file() {
                all_files.push(e.into_path());
            }
        }
    }
    let total = all_files.len();
    let mut count = 1;

    for file_path in all_files {
        let rel = match file_path.strip_prefix(target_dir) {
            Ok(r) => r,
            Err(_) => file_path.as_path(),
        };
        let rel_str = rel.to_string_lossy();

        // excludeファイル判定
        if let Some(gs) = &exclude_globset {
            if gs.is_match(&*rel_str) {
                continue;
            }
        }
        // include_exts
        if !all_exts.is_empty() {
            let ext = file_path
                .extension()
                .map(|x| format!(".{}", x.to_string_lossy()))
                .unwrap_or_default();
            if !all_exts.contains(&ext) {
                continue;
            }
        }

        eprintln!("({}/{}) Processing: {}", count, total, file_path.display());
        count += 1;

        // skip_content
        if let Some(gs) = &skip_globset {
            if gs.is_match(&*rel_str) {
                writeln!(outfile, "### {}", rel_str).ok();
                writeln!(outfile, "```").ok();
                writeln!(outfile, "(略)").ok();
                writeln!(outfile, "```").ok();
                writeln!(outfile).ok();
                continue;
            }
        }

        // バイナリチェック
        if config.skip_binary && is_binary_file(&file_path) {
            writeln!(outfile, "### {}", rel_str).ok();
            writeln!(outfile, "```").ok();
            writeln!(outfile, "(略) バイナリファイル").ok();
            writeln!(outfile, "```").ok();
            writeln!(outfile).ok();
            continue;
        }

        // ファイルサイズ
        if let Some(max_size) = config.max_file_size {
            if let Ok(meta) = fs::metadata(&file_path) {
                if meta.len() > max_size {
                    writeln!(outfile, "### {}", rel_str).ok();
                    writeln!(outfile, "```").ok();
                    writeln!(outfile, "(略)").ok();
                    writeln!(outfile, "```").ok();
                    writeln!(outfile).ok();
                    continue;
                }
            }
        }

        // ファイル内容
        writeln!(outfile, "### {}", rel_str).ok();
        writeln!(outfile, "```").ok();

        let file = match File::open(&file_path) {
            Ok(f) => f,
            Err(e) => {
                writeln!(outfile, "Error: {}", e).ok();
                writeln!(outfile).ok();
                continue;
            }
        };
        let reader = BufReader::new(file);
        let mut line_count = 0;
        for line_result in reader.lines() {
            match line_result {
                Ok(line) => {
                    if line_count >= config.max_lines {
                        writeln!(outfile, "...").ok();
                        writeln!(outfile, "(省略)").ok();
                        break;
                    }
                    writeln!(outfile, "{}", line).ok();
                    line_count += 1;
                }
                Err(e) => {
                    writeln!(outfile, "Error reading line: {}", e).ok();
                    break;
                }
            }
        }

        writeln!(outfile, "```").ok();
        writeln!(outfile).ok();
    }

    Ok(())
}
