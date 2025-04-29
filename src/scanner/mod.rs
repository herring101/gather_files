// src/scanner/mod.rs – v0.3.1-fix

mod counter;
pub mod detector;
mod sort;
mod utils;
mod walker;

use counter::ProcessCounter;
use sort::compare_dir_entry;
use utils::{build_globset, is_binary_file};
use walker::collect_entries;

use crate::model::ConfigParams;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// 省略理由
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OmitReason {
    Binary,
    TooLarge,
    Pattern,
}

impl std::fmt::Display for OmitReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OmitReason::Binary => write!(f, "binary"),
            OmitReason::TooLarge => write!(f, "too-large"),
            OmitReason::Pattern => write!(f, "pattern"),
        }
    }
}

/// メインの走査関数
pub fn run(
    target_dir: &Path,
    output_file: &Path,
    config: &ConfigParams,
    _cli_include_patterns: &[String],
) -> Result<(), String> {
    let mut counter = ProcessCounter::new();

    /* ---------- globset 構築 ---------- */
    let exclude_globset = build_globset(&config.exclude_patterns);
    let skip_globset = build_globset(&config.skip_content_patterns);
    let include_globset = if config.include_patterns.is_empty() {
        None
    } else {
        build_globset(&config.include_patterns)
    };

    /* ============================================================
    1st pass – 省略判定マップを作成
    ============================================================ */
    let mut file_entries = collect_entries(target_dir, &exclude_globset, true);
    file_entries.sort_by(|a, b| compare_dir_entry(a, b, target_dir));
    counter.set_total_files(file_entries.len());

    let mut omitted: HashMap<PathBuf, OmitReason> = HashMap::new();

    for entry in &file_entries {
        let path = entry.path();
        let rel: PathBuf = path.strip_prefix(target_dir).unwrap_or(path).to_path_buf();

        /* include フィルタ – マッチしないものは省略 */
        if let Some(gs) = &include_globset {
            if !gs.is_match(&rel) {
                omitted.insert(rel, OmitReason::Pattern);
                continue;
            }
        }

        /* skip パターン */
        if let Some(gs) = &skip_globset {
            if gs.is_match(&rel) {
                omitted.insert(rel, OmitReason::Pattern);
                continue;
            }
        }

        /* バイナリ検出 */
        if config.skip_binary && is_binary_file(path) {
            omitted.insert(rel, OmitReason::Binary);
            continue;
        }

        /* サイズ制限 */
        if let Some(max) = config.max_file_size {
            if let Ok(m) = fs::metadata(path) {
                if m.len() > max {
                    omitted.insert(rel, OmitReason::TooLarge);
                }
            }
        }
    }

    /* ============================================================
    2nd pass – ディレクトリツリー出力
    ============================================================ */
    let mut tree_entries = walker::collect_entries(target_dir, &exclude_globset, false);
    tree_entries.sort_by(|a, b| compare_dir_entry(a, b, target_dir));

    let mut outfile = File::create(output_file).map_err(|e| {
        format!(
            "出力ファイルを作成できません: {} - {}",
            output_file.display(),
            e
        )
    })?;

    writeln!(outfile, "```").ok();
    for entry in &tree_entries {
        let path = entry.path();
        let rel: PathBuf = path.strip_prefix(target_dir).unwrap_or(path).to_path_buf();
        let rel_str = rel.to_string_lossy();
        let indent = "    ".repeat(rel.components().count().saturating_sub(1));
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy())
            .unwrap_or_else(|| rel_str.clone());

        if let Some(reason) = omitted.get(&rel) {
            writeln!(outfile, "{indent}{name}   [omitted:{reason}]").ok();
        } else if path.is_dir() {
            writeln!(outfile, "{indent}{name}/").ok();
        } else {
            writeln!(outfile, "{indent}{name}").ok();
        }
    }
    writeln!(outfile, "```").ok();
    writeln!(outfile).ok();

    /* ============================================================
    3rd pass – ファイル内容出力（省略対象はスキップ）
    ============================================================ */
    for (idx, entry) in file_entries.iter().enumerate() {
        let path = entry.path();
        let rel: PathBuf = path.strip_prefix(target_dir).unwrap_or(path).to_path_buf();
        let rel_str = rel.to_string_lossy();

        /* 省略対象ならカウンタだけ更新 */
        if let Some(reason) = omitted.get(&rel) {
            match reason {
                OmitReason::Pattern => counter.increment_skipped_pattern(),
                OmitReason::Binary => counter.increment_skipped_binary(),
                OmitReason::TooLarge => counter.increment_skipped_size(),
            }
            continue;
        }

        /* ---- 本文を出力 ---- */
        eprintln!(
            "({}/{}) Processing: {}",
            idx + 1,
            file_entries.len(),
            path.display()
        );

        writeln!(outfile, "### {}", rel_str).ok();
        writeln!(outfile, "```").ok();

        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                writeln!(outfile, "Error: {}", e).ok();
                writeln!(outfile, "```").ok();
                writeln!(outfile).ok();
                counter.increment_processed();
                continue;
            }
        };
        let reader = BufReader::new(file);

        let mut lines = 0;
        for line in reader.lines() {
            match line {
                Ok(l) => {
                    if lines >= config.max_lines {
                        writeln!(outfile, "...").ok();
                        writeln!(outfile, "(省略)").ok();
                        break;
                    }
                    writeln!(outfile, "{l}").ok();
                    lines += 1;
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

    /* ============================================================
    summary
    ============================================================ */
    counter.print_summary();
    Ok(())
}
