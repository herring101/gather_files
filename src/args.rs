use crate::model::CLIOptions;
use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;

/// CLIオプションを clap でパース
pub fn parse_args() -> CLIOptions {
    let matches = Command::new("gather")
        .version("0.1.0")
        .author("Your Name")
        .about("Collect files recursively and output them as text.")
        .arg(
            Arg::new("target_directory")
                .help("解析したいディレクトリを指定")
                .required(true)
                .num_args(1)
                .value_name("DIR")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("出力ファイルのパス (ディレクトリでも可)")
                .num_args(1)
                .value_name("FILE_OR_DIR")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("config_file")
                .long("config-file")
                .short('c')
                .help("設定ファイル (例: .gather)")
                .num_args(1)
                .value_name("FILE")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("max_lines")
                .long("max-lines")
                .short('m')
                .help("各ファイルから読み込む最大行数")
                .num_args(1)
                .value_name("N")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("max_file_size")
                .long("max-file-size")
                .help("このサイズ(BYTE)を超えるファイルをスキップ")
                .num_args(1)
                .value_name("BYTES")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("patterns")
                .long("patterns")
                .short('p')
                .help("追加の除外パターン (1パターンずつ複数回指定可)")
                .action(ArgAction::Append)
                .value_name("PATTERN"),
        )
        .arg(
            Arg::new("skip_patterns")
                .long("skip-patterns")
                .short('s')
                .help("追加の内容スキップパターン (1パターンずつ複数回指定可)")
                .action(ArgAction::Append)
                .value_name("PATTERN"),
        )
        .arg(
            Arg::new("include_extensions")
                .long("include-extensions")
                .short('i')
                .help("含めたい拡張子 (1つずつ複数回指定可)")
                .action(ArgAction::Append)
                .value_name("EXT"),
        )
        .arg(
            Arg::new("init_config")
                .long("init-config")
                .help(".gather のサンプルファイルを生成して終了します")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let target_dir_path = PathBuf::from(
        matches
            .get_one::<String>("target_directory")
            .expect("required"),
    );

    let output_file = matches.get_one::<String>("output").map(PathBuf::from);
    let config_file = matches.get_one::<String>("config_file").map(PathBuf::from);

    let max_lines = matches
        .get_one::<String>("max_lines")
        .and_then(|s| s.parse::<usize>().ok());

    let max_file_size = matches
        .get_one::<String>("max_file_size")
        .and_then(|s| s.parse::<u64>().ok());

    let extra_exclude_patterns = matches
        .get_many::<String>("patterns")
        .map(|vals| vals.map(|v| v.to_string()).collect())
        .unwrap_or_default();

    let extra_skip_patterns = matches
        .get_many::<String>("skip_patterns")
        .map(|vals| vals.map(|v| v.to_string()).collect())
        .unwrap_or_default();

    let include_exts = matches
        .get_many::<String>("include_extensions")
        .map(|vals| vals.map(|v| v.to_string()).collect())
        .unwrap_or_default();

    let init_config = matches.get_flag("init_config");

    CLIOptions {
        target_dir: target_dir_path,
        output_file,
        config_file,
        max_lines,
        max_file_size,
        extra_exclude_patterns,
        extra_skip_patterns,
        include_exts,
        init_config,
    }
}
