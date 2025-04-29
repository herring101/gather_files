// src/args.rs
//! CLI 引数パーサ
//!
//! **使い方**
//! ```bash
//! gather_files <DIR> [OPTIONS]
//! ```

use clap::{ArgAction, Parser};
use std::path::PathBuf;

use crate::model::CLIOptions;

/// 内部用 – clap 派生構造体
#[derive(Debug, Parser)]
#[command(
    name = "gather_files",
    version,
    author = "herring101",
    about = "Collect project files and format them for LLM context.",
    disable_help_subcommand = true,
    arg_required_else_help = true
)]
struct Args {
    /// 解析したいディレクトリ
    #[arg(value_name = "DIR", required = true)]
    target_directory: PathBuf,

    /// 出力ファイルのパス (デフォルト: gather/output.txt)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// 出力ファイル名にタイムスタンプを付与する
    #[arg(long, action = ArgAction::SetTrue)]
    timestamp: bool,

    /// 設定ファイル (.gather)
    #[arg(short, long, value_name = "FILE")]
    config_file: Option<PathBuf>,

    /// 各ファイルから読み込む最大行数
    #[arg(short, long, value_name = "N")]
    max_lines: Option<usize>,

    /// このサイズ (BYTE) を超えるファイルをスキップ
    #[arg(long, value_name = "BYTES")]
    max_file_size: Option<u64>,

    /// 追加の除外パターン (複数回指定可)
    #[arg(short = 'p', long = "patterns", value_name = "PATTERN", action = ArgAction::Append)]
    patterns: Vec<String>,

    /// 追加の内容スキップパターン (複数回指定可)
    #[arg(short = 's', long = "skip-patterns", value_name = "PATTERN", action = ArgAction::Append)]
    skip_patterns: Vec<String>,

    /// 含めたいファイルパターン (複数回指定可)
    #[arg(short = 'i', long = "include-patterns", value_name = "PATTERN", action = ArgAction::Append)]
    include_patterns: Vec<String>,

    /// .gather を自動で VSCode で開かない
    #[arg(long, action = ArgAction::SetTrue)]
    no_open: bool,

    /// .gitignore の内容を [exclude] に統合
    #[arg(long, action = ArgAction::SetTrue)]
    use_gitignore: bool,
}

/// 既存 API 互換ラッパ
pub fn parse_args() -> CLIOptions {
    let a = Args::parse();

    CLIOptions {
        target_dir: a.target_directory,
        output_file: a.output,
        config_file: a.config_file,
        max_lines: a.max_lines,
        max_file_size: a.max_file_size,
        extra_exclude_patterns: a.patterns,
        extra_skip_patterns: a.skip_patterns,
        include_patterns: a.include_patterns,
        use_timestamp: a.timestamp,
        no_open: a.no_open,
        use_gitignore: a.use_gitignore,
    }
}

// ---------------------------------------------------------------------
// tests
// ---------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positional_directory_is_required() {
        // omit DIR → clap should error
        let res = Args::try_parse_from(["gather_files"]);
        assert!(res.is_err());
    }

    #[test]
    fn parse_multiple_patterns() {
        let args = Args::try_parse_from(["gather_files", "./proj", "-p", "*.log", "-p", "build/"])
            .unwrap();
        assert_eq!(args.patterns, vec!["*.log", "build/"]);
    }
}
