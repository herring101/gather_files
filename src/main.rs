mod args;
mod config;
mod model;
mod scanner;

use crate::args::parse_args;
use crate::config::load_config_file;
use crate::scanner::run;

use chrono::Local;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};

fn main() {
    // 1) CLIオプション取得
    let cli_opts = parse_args();

    // 2) ターゲットディレクトリ存在チェック
    if !cli_opts.target_dir.is_dir() {
        eprintln!(
            "ディレクトリが存在しません: {}",
            cli_opts.target_dir.display()
        );
        process::exit(1);
    }

    // 3) .gather パスを決定
    let gather_path = cli_opts
        .config_file
        .clone()
        .unwrap_or_else(|| cli_opts.target_dir.join(".gather"));

    // 4) .gather が無ければ自動生成
    if !gather_path.exists() {
        // -- デフォルト内容: [settings], [exclude], [skip], [include] をすべて含む --
        let sample = r#"[settings]
max_lines=1000
max_file_size=500000
skip_binary=yes
output_dir=

[exclude]
.git
gather
.gather

[skip]
*.pdf

[include]
# (拡張子未指定の場合、すべて含む想定)
# .py
"#;

        match fs::write(&gather_path, sample) {
            Ok(_) => {
                eprintln!(".gatherファイルを生成しました: {}", gather_path.display());
                // 生成後に code で開く (no_open が指定されていない場合のみ)
                if !cli_opts.no_open {
                    let _ = Command::new("code").arg(&gather_path).spawn().map_err(|e| {
                        eprintln!("Warning: VS Code を起動できませんでした: {}", e);
                    });
                }
            }
            Err(e) => {
                eprintln!("作成に失敗しました: {}", e);
                process::exit(1);
            }
        }
    } else {
        // 既に存在する場合、開くかどうかは好みに応じて
        // 今回は特に開かない例 (開きたい場合は以下のようにする)
        /*
        if !cli_opts.no_open {
            let _ = Command::new("code")
                .arg(&gather_path)
                .spawn()
                .map_err(|e| {
                    eprintln!("Warning: VS Code を起動できませんでした: {}", e);
                });
        }
        */
    }

    // 5) .gather 読み込み
    let mut config_params = load_config_file(&gather_path);

    // 6) CLIオプションを上書き & 結合
    if let Some(m) = cli_opts.max_lines {
        config_params.max_lines = m;
    }
    if let Some(mf) = cli_opts.max_file_size {
        config_params.max_file_size = Some(mf);
    }
    if !cli_opts.extra_exclude_patterns.is_empty() {
        config_params
            .exclude_patterns
            .extend(cli_opts.extra_exclude_patterns);
    }
    if !cli_opts.extra_skip_patterns.is_empty() {
        config_params
            .skip_content_patterns
            .extend(cli_opts.extra_skip_patterns);
    }
    if !cli_opts.include_exts.is_empty() {
        config_params.include_exts.extend(cli_opts.include_exts);
    }

    // 7) 出力ファイルのパス決定
    let output_path: PathBuf = if let Some(ref out) = cli_opts.output_file {
        out.clone()
    } else {
        // デフォルトは gather/output.txt
        let default_dir = cli_opts.target_dir.join("gather");
        if !default_dir.is_dir() {
            if let Err(e) = fs::create_dir_all(&default_dir) {
                eprintln!("outputディレクトリの作成に失敗: {}", e);
                process::exit(1);
            }
        }
        let file_name = if cli_opts.use_timestamp {
            let ts = Local::now().format("%Y%m%d%H%M%S").to_string();
            format!("output_{}.txt", ts)
        } else {
            "output.txt".to_string()
        };
        default_dir.join(file_name)
    };

    // 8) スキャナ実行
    if let Err(e) = run(&cli_opts.target_dir, &output_path, &config_params, &[]) {
        eprintln!("エラー: {}", e);
        process::exit(1);
    }

    eprintln!("Done! Output => {}", output_path.display());
}
