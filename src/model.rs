use std::path::PathBuf;

/// CLI で受け取るパラメータ
#[derive(Debug)]
pub struct CLIOptions {
    pub target_dir: PathBuf,
    pub output_file: Option<PathBuf>,
    pub config_file: Option<PathBuf>,
    pub max_lines: Option<usize>,
    pub max_file_size: Option<u64>,
    pub extra_exclude_patterns: Vec<String>,
    pub extra_skip_patterns: Vec<String>,
    pub include_patterns: Vec<String>, // includeパターン（グロブ形式）
    pub use_timestamp: bool, // --timestamp
    pub no_open: bool,       // --no-open
    pub use_gitignore: bool, // --use-gitignore
}

/// 設定ファイル(.gather) + CLIを合体して最終的に使うパラメータ
#[derive(Debug)]
pub struct ConfigParams {
    pub max_lines: usize,
    pub max_file_size: Option<u64>,
    pub skip_binary: bool,
    pub output_dir: Option<String>,
    pub exclude_patterns: Vec<String>,
    pub skip_content_patterns: Vec<String>,
    pub include_patterns: Vec<String>, // includeパターン（グロブ形式）
    pub use_timestamp: bool, // 追加: タイムスタンプ付きの出力ファイル名を使用
    pub open_output: bool,   // 追加: 出力ファイルをVSCodeで開く
    pub use_gitignore: bool, // 追加: .gitignore を使用
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
            use_timestamp: false, // デフォルト: false
            open_output: true,    // デフォルト: true
            use_gitignore: false, // デフォルト: false
        }
    }
}
