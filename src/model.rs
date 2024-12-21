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
    pub init_config: bool, // --init-config
}

/// 設定ファイル(.gather) + CLIを合体して最終的に使うパラメータ
#[derive(Debug)]
pub struct ConfigParams {
    pub max_lines: usize,
    pub max_file_size: Option<u64>,
    pub skip_binary: bool,
    /// 出力先ディレクトリ（[settings] output_dir=xxx）を使う
    /// ただし CLI で -o を指定した場合はそちらが優先される想定
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
