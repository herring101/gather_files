//! src/args.rs

use clap::{ArgAction, ArgGroup, Parser, ValueEnum};
use std::path::PathBuf;

use crate::model::{CLIOptions, OutlineFormat, RunMode};

/// outline サブオプション
#[derive(Debug, Clone, ValueEnum)]
pub enum ModeArg {
    Gather,
    Outline,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum FormatArg {
    Md,
    Json,
}

/// 内部用 – clap 派生構造体
#[derive(Debug, Parser)]
#[command(
    name = "gather",
    version,
    author = "herring101",
    about = "Collect project files OR generate outline for LLM context.",
    disable_help_subcommand = true,
    arg_required_else_help = true,
    group(
        ArgGroup::new("outline_opts")
            .requires("mode")
            .args(["outline_format"])
    )
)]
struct Args {
    /// 実行モード: gather (既定) / outline
    #[arg(long, value_enum, value_name = "MODE", default_value = "gather")]
    mode: ModeArg,

    /// outline 時のフォーマット: md (既定) / json
    #[arg(long = "outline-format", value_enum, value_name = "FMT")]
    outline_format: Option<FormatArg>,

    /// 解析対象ディレクトリ
    #[arg(value_name = "DIR")]
    target_directory: PathBuf,

    // ・・・既存オプションはそのまま・・・
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
    #[arg(long, action = ArgAction::SetTrue)]
    timestamp: bool,
    #[arg(short, long, value_name = "FILE")]
    config_file: Option<PathBuf>,
    #[arg(short, long, value_name = "N")]
    max_lines: Option<usize>,
    #[arg(long, value_name = "BYTES")]
    max_file_size: Option<u64>,
    #[arg(short = 'p', long = "patterns", value_name = "PATTERN", action = ArgAction::Append)]
    patterns: Vec<String>,
    #[arg(short = 's', long = "skip-patterns", value_name = "PATTERN", action = ArgAction::Append)]
    skip_patterns: Vec<String>,
    #[arg(short = 'i', long = "include-patterns", value_name = "PATTERN", action = ArgAction::Append)]
    include_patterns: Vec<String>,
    #[arg(long, action = ArgAction::SetTrue)]
    no_open: bool,
    #[arg(long, action = ArgAction::SetTrue)]
    use_gitignore: bool,
}

/// 既存 API 互換ラッパ
pub fn parse_args() -> CLIOptions {
    let a = Args::parse();

    let format = match a.outline_format.unwrap_or(FormatArg::Md) {
        FormatArg::Md => OutlineFormat::Md,
        FormatArg::Json => OutlineFormat::Json,
    };
    let mode = match a.mode {
        ModeArg::Gather => RunMode::Gather,
        ModeArg::Outline => RunMode::Outline(format),
    };

    CLIOptions {
        mode,
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

/* --------------------------------------------------------------------- */
/* tests                                                                 */
/* --------------------------------------------------------------------- */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mode_is_gather() {
        let args = Args::try_parse_from(["gather", "."]).unwrap();
        assert!(matches!(args.mode, ModeArg::Gather));
    }

    #[test]
    fn outline_mode_parses_with_format() {
        let args = Args::try_parse_from([
            "gather",
            "--mode",
            "outline",
            "--outline-format",
            "json",
            ".",
        ])
        .unwrap();
        assert!(matches!(args.mode, ModeArg::Outline));
        assert!(matches!(args.outline_format, Some(FormatArg::Json)));
    }
}
