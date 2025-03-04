// src/scanner/mod.rs

mod counter;
mod sort;
mod utils;
mod walker;

use counter::ProcessCounter;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use crate::model::ConfigParams;
use sort::compare_dir_entry;
use utils::{build_globset, is_binary_file};
use walker::collect_entries;

/// メインの走査関数
pub fn run(
    target_dir: &Path,
    output_file: &Path,
    config: &ConfigParams,
    _cli_include_patterns: &[String],
) -> Result<(), String> {
    let mut counter = ProcessCounter::new();

    // 出力ファイルを開く
    let mut outfile = File::create(output_file).map_err(|e| {
        format!(
            "出力ファイルを作成できません: {} - {}",
            output_file.display(),
            e
        )
    })?;

    // globsetの構築
    let exclude_globset = build_globset(&config.exclude_patterns);
    let skip_globset = build_globset(&config.skip_content_patterns);
    
    // include patterns
    // 空の場合はNoneを返すので、空の場合は全ファイルを含める
    let include_globset = if config.include_patterns.is_empty() {
        None
    } else {
        build_globset(&config.include_patterns)
    };

    // ディレクトリツリーの出力
    let mut entries = collect_entries(target_dir, &exclude_globset, false);
    entries.sort_by(|a, b| compare_dir_entry(a, b, target_dir));

    writeln!(outfile, "```").ok();
    for entry in entries {
        let path = entry.path();
        let rel = match path.strip_prefix(target_dir) {
            Ok(r) => r,
            Err(_) => path,
        };

        if path.is_dir() {
            let level = rel.components().count();
            let indent = "    ".repeat(level.saturating_sub(1));
            let name = path
                .file_name()
                .map(|s| s.to_string_lossy())
                .unwrap_or_default();
            writeln!(outfile, "{}{}/", indent, name).ok();
        } else {
            let rel_str = rel.to_string_lossy();

            // exclude
            if let Some(gs) = &exclude_globset {
                if gs.is_match(Path::new(&*rel_str)) {
                    counter.increment_skipped_pattern();
                    continue;
                }
            }

            // include patterns
            if let Some(gs) = &include_globset {
                if !gs.is_match(Path::new(&*rel_str)) {
                    counter.increment_skipped_extension();
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

    // ファイル内容の出力
    let mut file_entries = collect_entries(target_dir, &exclude_globset, true);

    file_entries.sort_by(|a, b| compare_dir_entry(a, b, target_dir));

    let total_files = file_entries.len();
    counter.set_total_files(total_files);

    for (idx, entry) in file_entries.iter().enumerate() {
        let path = entry.path();
        let rel = match path.strip_prefix(target_dir) {
            Ok(r) => r,
            Err(_) => path,
        };
        let rel_str = rel.to_string_lossy();

        // exclude
        if let Some(gs) = &exclude_globset {
            if gs.is_match(Path::new(&*rel_str)) {
                counter.increment_skipped_pattern();
                continue;
            }
        }

        // include patterns
        if let Some(gs) = &include_globset {
            if !gs.is_match(Path::new(&*rel_str)) {
                counter.increment_skipped_extension();
                continue;
            }
        }

        eprintln!(
            "({}/{}) Processing: {}",
            idx + 1,
            total_files,
            path.display()
        );

        // skip pattern
        if let Some(gs) = &skip_globset {
            if gs.is_match(Path::new(&*rel_str)) {
                writeln!(outfile, "### {}", rel_str).ok();
                writeln!(outfile, "```").ok();
                writeln!(outfile, "(略)").ok();
                writeln!(outfile, "```").ok();
                writeln!(outfile).ok();
                counter.increment_processed();
                continue;
            }
        }

        // バイナリチェック
        if config.skip_binary && is_binary_file(path) {
            writeln!(outfile, "### {}", rel_str).ok();
            writeln!(outfile, "```").ok();
            writeln!(outfile, "(略) バイナリファイル").ok();
            writeln!(outfile, "```").ok();
            writeln!(outfile).ok();
            counter.increment_skipped_binary();
            continue;
        }

        // サイズ制限
        if let Some(max_size) = config.max_file_size {
            if let Ok(meta) = fs::metadata(path) {
                if meta.len() > max_size {
                    writeln!(outfile, "### {}", rel_str).ok();
                    writeln!(outfile, "```").ok();
                    writeln!(outfile, "(略)").ok();
                    writeln!(outfile, "```").ok();
                    writeln!(outfile).ok();
                    counter.increment_skipped_size();
                    continue;
                }
            }
        }

        // ファイル内容を出力
        writeln!(outfile, "### {}", rel_str).ok();
        writeln!(outfile, "```").ok();

        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                writeln!(outfile, "Error: {}", e).ok();
                writeln!(outfile).ok();
                counter.increment_processed();
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
        counter.increment_processed();
    }

    // 処理サマリーの表示
    counter.print_summary();

    Ok(())
}
