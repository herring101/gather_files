//! Large-directory detector used on first run to propose `[exclude]` patterns.

use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Result of a large-directory scan.
///
/// *The fields other than `path` are currently only used for
/// informational logging on the very first run, so they are allowed
/// to remain unread without triggering warnings.*
#[allow(dead_code)]
#[derive(Debug)]
pub struct DetectionResult {
    pub path: PathBuf,
    pub file_count: usize,
    pub total_size: u64,
    pub reason: DetectionReason,
}

/// Why a directory was flagged as “large”.
#[allow(dead_code)]
#[derive(Debug)]
pub enum DetectionReason {
    KnownDirectory,
    TooManyFiles,
    TooLarge,
}

impl std::fmt::Display for DetectionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectionReason::KnownDirectory => write!(f, "一般的な大規模ディレクトリ"),
            DetectionReason::TooManyFiles => write!(f, "ファイル数超過"),
            DetectionReason::TooLarge => write!(f, "サイズ超過"),
        }
    }
}

/* ───── same implementation as before (unchanged code omitted) ───── */

const KNOWN_LARGE_DIRS: &[&str] = &[
    "node_modules",
    "venv",
    ".venv",
    "env",
    "target",
    "dist",
    "build",
    "vendor",
    "__pycache__",
    ".git",
];

pub fn detect_large_directories(
    target_dir: &Path,
    max_files_per_dir: usize,
    max_file_size: u64,
) -> Vec<DetectionResult> {
    // …既存実装そのまま…
    let mut results = Vec::new();
    let entries = match fs::read_dir(target_dir) {
        Ok(e) => e,
        Err(_) => return results,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let dir_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        /* known dirs */
        if KNOWN_LARGE_DIRS.contains(&dir_name.as_str()) {
            let (cnt, size) = count_files_and_size(&path);
            results.push(DetectionResult {
                path,
                file_count: cnt,
                total_size: size,
                reason: DetectionReason::KnownDirectory,
            });
            continue;
        }

        let (cnt, size) = count_files_and_size(&path);
        if cnt > max_files_per_dir {
            results.push(DetectionResult {
                path,
                file_count: cnt,
                total_size: size,
                reason: DetectionReason::TooManyFiles,
            });
        } else if size > max_file_size {
            results.push(DetectionResult {
                path,
                file_count: cnt,
                total_size: size,
                reason: DetectionReason::TooLarge,
            });
        }
    }
    results
}

fn count_files_and_size(dir: &Path) -> (usize, u64) {
    let mut cnt = 0;
    let mut size = 0;
    for entry in WalkDir::new(dir)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() {
            cnt += 1;
            if let Ok(m) = fs::metadata(entry.path()) {
                size += m.len();
            }
        }
    }
    (cnt, size)
}

pub fn generate_exclude_patterns(results: &[DetectionResult], root: &Path) -> Vec<String> {
    let mut v = Vec::new();
    for r in results {
        if let Ok(rel) = r.path.strip_prefix(root) {
            let mut p = rel.to_string_lossy().to_string();
            if !p.ends_with('/') {
                p.push('/');
            }
            v.push(p);
        }
    }
    v
}
