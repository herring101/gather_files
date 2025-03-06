use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 大規模ディレクトリ検出の結果
#[derive(Debug)]
pub struct DetectionResult {
    pub path: PathBuf,
    pub file_count: usize,
    pub total_size: u64,
    pub reason: DetectionReason,
}

/// 検出理由
#[derive(Debug)]
pub enum DetectionReason {
    /// 一般的な大規模ディレクトリ名（node_modules, venv など）
    KnownDirectory,
    /// ファイル数が閾値を超えている
    TooManyFiles,
    /// ディレクトリサイズが閾値を超えている
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

/// 一般的な大規模ディレクトリ名のリスト
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

/// 大規模ディレクトリを検出する
///
/// # 引数
///
/// * `target_dir` - 検査対象のディレクトリパス
/// * `max_files_per_dir` - ディレクトリ内のファイル数の閾値
/// * `max_file_size` - ファイルサイズの閾値（バイト）
///
/// # 戻り値
///
/// 検出された大規模ディレクトリのリスト
pub fn detect_large_directories(
    target_dir: &Path,
    max_files_per_dir: usize,
    max_file_size: u64,
) -> Vec<DetectionResult> {
    let mut results = Vec::new();

    // ルートディレクトリの直下のみを検査（深さ制限）
    let entries = match fs::read_dir(target_dir) {
        Ok(entries) => entries,
        Err(_) => return results,
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        // 一般的な大規模ディレクトリ名のチェック
        let dir_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => continue,
        };

        if KNOWN_LARGE_DIRS.contains(&dir_name.as_str()) {
            // ファイル数とサイズを計算
            let (file_count, total_size) = count_files_and_size(&path);
            results.push(DetectionResult {
                path,
                file_count,
                total_size,
                reason: DetectionReason::KnownDirectory,
            });
            continue;
        }

        // ファイル数とサイズを計算
        let (file_count, total_size) = count_files_and_size(&path);

        // ファイル数が閾値を超えているかチェック
        if file_count > max_files_per_dir {
            results.push(DetectionResult {
                path,
                file_count,
                total_size,
                reason: DetectionReason::TooManyFiles,
            });
            continue;
        }

        // ディレクトリサイズが閾値を超えているかチェック
        if total_size > max_file_size {
            results.push(DetectionResult {
                path,
                file_count,
                total_size,
                reason: DetectionReason::TooLarge,
            });
        }
    }

    results
}

/// ディレクトリ内のファイル数とサイズを計算する
///
/// # 引数
///
/// * `dir` - 対象ディレクトリのパス
///
/// # 戻り値
///
/// (ファイル数, 合計サイズ) のタプル
fn count_files_and_size(dir: &Path) -> (usize, u64) {
    let mut file_count = 0;
    let mut total_size = 0;

    // 最大深さを3に制限して軽量スキャン
    for entry in WalkDir::new(dir)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            file_count += 1;
            if let Ok(metadata) = fs::metadata(path) {
                total_size += metadata.len();
            }
        }
    }

    (file_count, total_size)
}

/// 検出結果から除外パターンを生成する
///
/// # 引数
///
/// * `results` - 検出結果のベクター
/// * `target_dir` - ベースディレクトリ
///
/// # 戻り値
///
/// 除外パターンのベクター
pub fn generate_exclude_patterns(results: &[DetectionResult], target_dir: &Path) -> Vec<String> {
    let mut patterns = Vec::new();

    for result in results {
        if let Ok(rel_path) = result.path.strip_prefix(target_dir) {
            let pattern = rel_path.to_string_lossy().to_string();
            // ディレクトリパターンには末尾にスラッシュを追加
            if !pattern.ends_with('/') {
                patterns.push(format!("{}/", pattern));
            } else {
                patterns.push(pattern);
            }
        }
    }

    patterns
}
