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
use std::process;

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

    // 3) --init-config: サンプル .gather を作成して終了
    if cli_opts.init_config {
        let default_path = cli_opts.target_dir.join(".gather");
        if default_path.exists() {
            eprintln!("既に設定ファイルが存在します: {}", default_path.display());
        } else {
            // これがサンプル: 縦書きでフォルダ除外などを記載
            let sample = r#"[settings]
max_lines=1000
max_file_size=500000
skip_binary=yes
output_dir=out

[exclude]
.git
target/
*.log

[skip]
*.md
*.pdf

[include]
.rs
.py
"#;
            match fs::write(&default_path, sample) {
                Ok(_) => {
                    eprintln!(".gatherファイルを生成しました: {}", default_path.display());
                }
                Err(e) => {
                    eprintln!("作成に失敗しました: {}", e);
                    process::exit(1);
                }
            }
        }
        process::exit(0);
    }

    // 4) .gather 読み込み (なければデフォルト)
    let config_file = cli_opts
        .config_file
        .clone()
        .unwrap_or_else(|| cli_opts.target_dir.join(".gather"));
    let mut config_params = load_config_file(&config_file);

    // 5) CLIオプションを上書き & 結合
    // (a) settings セクション相当: max_lines, max_file_size, skip_binary, output_dir は CLI優先
    if let Some(m) = cli_opts.max_lines {
        config_params.max_lines = m;
    }
    if let Some(mf) = cli_opts.max_file_size {
        config_params.max_file_size = Some(mf);
    }
    // skip_binary は今回 CLI引数で定義してないので省略

    // (b) patterns 相当: exclude_patterns, skip_content_patterns, include_exts は結合
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

    // 6) 出力先
    //   - もし CLI で --output があった場合はそれを優先
    //   - なければ config_params.output_dir があればディレクトリとして使う
    //   - それもなければ gather_{timestamp}.txt
    let output_path: PathBuf = if let Some(ref out) = cli_opts.output_file {
        // これがファイルかディレクトリかはユーザー次第
        // いったん「拡張子がない or ディレクトリなら、その下に gather_{timestamp}.txt を生成」とする
        if out.is_dir() || out.extension().is_none() {
            let ts = Local::now().format("%Y%m%d%H%M%S").to_string();
            let file_name = format!("gather_{}.txt", ts);
            out.join(file_name)
        } else {
            out.clone()
        }
    } else if let Some(dir) = &config_params.output_dir {
        // 設定ファイル上で output_dir が指定されていた場合
        let dir_path = PathBuf::from(dir);
        if !dir_path.is_dir() {
            // なければ作る
            if let Err(e) = fs::create_dir_all(&dir_path) {
                eprintln!("output_dir の作成に失敗: {}", e);
                process::exit(1);
            }
        }
        let ts = Local::now().format("%Y%m%d%H%M%S").to_string();
        dir_path.join(format!("gather_{}.txt", ts))
    } else {
        // 何も指定がなければカレントに gather_{timestamp}.txt
        let ts = Local::now().format("%Y%m%d%H%M%S").to_string();
        PathBuf::from(format!("gather_{}.txt", ts))
    };

    // 7) スキャナ実行
    if let Err(e) = run(&cli_opts.target_dir, &output_path, &config_params, &[]) {
        eprintln!("エラー: {}", e);
        process::exit(1);
    }

    eprintln!("Done! Output => {}", output_path.display());
}
