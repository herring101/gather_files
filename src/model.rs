#![allow(missing_docs)]

use std::path::PathBuf;

/* ---------- CLI -> Core options ---------- */

#[derive(Debug)]
pub struct CLIOptions {
    pub target_dir: PathBuf,
    pub output_file: Option<PathBuf>,
    pub config_file: Option<PathBuf>,
    pub max_lines: Option<usize>,
    pub max_file_size: Option<u64>,
    pub extra_exclude_patterns: Vec<String>,
    pub extra_skip_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
    pub use_timestamp: bool,
    pub no_open: bool,
    pub use_gitignore: bool,
}

/* ---------- Effective config after merge ---------- */

#[derive(Debug)]
pub struct ConfigParams {
    pub max_lines: usize,
    pub max_file_size: Option<u64>,
    pub skip_binary: bool,
    pub output_dir: Option<String>,
    pub exclude_patterns: Vec<String>,
    pub skip_content_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
    pub use_timestamp: bool,
    pub open_output: bool,
    pub use_gitignore: bool,
    pub first_run_completed: bool,
    pub max_files_per_dir: usize,
    pub max_auto_file_size: u64,
}

impl Default for ConfigParams {
    fn default() -> Self {
        Self {
            max_lines: 1000,
            max_file_size: None,
            skip_binary: false,
            output_dir: None,
            exclude_patterns: vec![],
            skip_content_patterns: vec![],
            include_patterns: vec![],
            use_timestamp: false,
            open_output: true,
            use_gitignore: false,
            first_run_completed: false,
            max_files_per_dir: 100,
            max_auto_file_size: 1_000_000,
        }
    }
}
