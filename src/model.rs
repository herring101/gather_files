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
    pub include_exts: Vec<String>,

    // --- ここから変更 ---
    pub use_timestamp: bool, // --timestamp
    pub no_open: bool,       // --no-open
                             // --- ここまで変更 ---
}

/// 設定ファイル(.gather) + CLIを合体して最終的に使うパラメータ
#[derive(Debug)]
pub struct ConfigParams {
    pub max_lines: usize,
    pub max_file_size: Option<u64>,
    pub skip_binary: bool,
    /// 出力先ディレクトリの代わりに、今回の仕様では固定ファイル名を優先するが、
    /// 既存コードを最小限変更するために残しておく
    pub output_dir: Option<String>,

    /// 除外ファイル / フォルダパターン
    pub exclude_patterns: Vec<String>,
    /// 内容スキップパターン
    pub skip_content_patterns: Vec<String>,
    /// 含めたい拡張子
    pub include_exts: Vec<String>,
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
            include_exts: vec![],
        }
    }
}
