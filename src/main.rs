mod args;
mod config;
mod gitignore;
mod model;
mod scanner;
mod updater;

use crate::args::parse_args;
use crate::config::load_config_file;
use crate::gitignore::parse_gitignore;
use crate::scanner::detector::{detect_large_directories, generate_exclude_patterns};
use crate::scanner::run;

use chrono::Local;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};

fn main() {
    // ------------------------------------------------------------------
    // 0) “self-update” サブコマンドの特判
    // ------------------------------------------------------------------
    {
        // argv[0] はプログラム名なので skip
        let mut iter = std::env::args().skip(1);
        if let Some(cmd) = iter.next() {
            if cmd == "self-update" || cmd == "update" {
                if let Err(e) = updater::run() {
                    eprintln!("Self-update failed: {e}");
                    process::exit(1);
                }
                return; // 更新完了
            }
        }
    }

    // ------------------------------------------------------------------
    // 1) CLI オプション取得
    // ------------------------------------------------------------------
    let cli_opts = parse_args();

    // 以下、元の処理はそのまま ─────────────────────────────
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
    let is_first_run = !gather_path.exists();
    if is_first_run {
        // 大規模ディレクトリを検出
        eprintln!("プロジェクト内の大規模ディレクトリを検出しています...");
        let large_dirs = detect_large_directories(&cli_opts.target_dir, 100, 1000000);

        // 除外パターンを生成
        let auto_exclude_patterns = generate_exclude_patterns(&large_dirs, &cli_opts.target_dir);

        // 検出結果を表示
        if !large_dirs.is_empty() {
            eprintln!(
                "以下の大規模ディレクトリを検出しました:
"
            );
            for dir in &large_dirs {
                let rel_path = match dir.path.strip_prefix(&cli_opts.target_dir) {
                    Ok(p) => p.to_string_lossy(),
                    Err(_) => dir.path.to_string_lossy(),
                };
                eprintln!(
                    "  - {}/: {} ファイル ({:.2} MB) - {}",
                    rel_path,
                    dir.file_count,
                    dir.total_size as f64 / 1_000_000.0,
                    dir.reason
                );
            }
            eprintln!();
        }

        // .gather ファイルのテンプレートを作成
        let mut sample = String::from(
            r#"[settings]
max_lines=1000
max_file_size=500000
skip_binary=yes
output_dir=gather
use_timestamp=no
open_output=yes
use_gitignore=yes
first_run_completed=no
max_files_per_dir=100
max_auto_file_size=1000000

[exclude]
gather/
.gather
"#,
        );

        // 検出した大規模ディレクトリを除外パターンに追加
        if !auto_exclude_patterns.is_empty() {
            for pattern in auto_exclude_patterns {
                sample.push_str(&format!(
                    "{}
",
                    pattern
                ));
            }
        }

        // スキップセクションとインクルードセクションを追加
        sample.push_str(
            r#"
[skip]
*.pdf

[include]
# (パターン未指定の場合、すべて含む想定)
# 例：
# *.md         # すべてのMarkdownファイル
# src/**/*.rs  # srcディレクトリ以下のRustファイル
# *.{js,ts}    # すべてのJavaScriptとTypeScriptファイル
"#,
        );

        match fs::write(&gather_path, sample) {
            Ok(_) => {
                eprintln!(
                    "
.gatherファイルを生成しました: {}
",
                    gather_path.display()
                );
                eprintln!("プロジェクトを効率的に収集するために、まず.gatherファイルを開いて設定を確認・調整してください。");
                eprintln!("特に[exclude]セクションで、不要なディレクトリやファイルを除外することをお勧めします。");
                eprintln!(
                    "設定が完了したら、再度コマンドを実行してください。
"
                );

                // .gather ファイルをエディタで開く
                match Command::new("code").arg(&gather_path).status() {
                    Ok(_) => (),
                    Err(e) => eprintln!(
                        "Warning: VS Code で.gatherファイルを開けませんでした: {}",
                        e
                    ),
                }

                // 初回実行時はここで終了
                process::exit(0);
            }
            Err(e) => {
                eprintln!("作成に失敗しました: {}", e);
                process::exit(1);
            }
        }
    }

    // 5) .gather 読み込み
    let mut config_params = load_config_file(&gather_path);

    // 初回実行完了フラグをチェック
    if !config_params.first_run_completed {
        // .gatherファイルを更新して初回実行完了フラグをセット
        let content = match fs::read_to_string(&gather_path) {
            Ok(content) => content.replace("first_run_completed=no", "first_run_completed=yes"),
            Err(_) => {
                eprintln!("Warning: .gatherファイルの読み込みに失敗しました");
                // 読み込み失敗時はそのまま続行
                config_params.first_run_completed = true;
                String::new()
            }
        };

        if !content.is_empty() {
            if let Err(e) = fs::write(&gather_path, content) {
                eprintln!("Warning: .gatherファイルの更新に失敗しました: {}", e);
            } else {
                // メモリ上のフラグも更新
                config_params.first_run_completed = true;
            }
        }
    }

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
    if !cli_opts.include_patterns.is_empty() {
        config_params
            .include_patterns
            .extend(cli_opts.include_patterns);
    }
    if cli_opts.use_timestamp {
        config_params.use_timestamp = true;
    }
    if cli_opts.no_open {
        config_params.open_output = false;
    }
    if cli_opts.use_gitignore {
        config_params.use_gitignore = true;
    }

    // 7) .gitignoreの統合
    if config_params.use_gitignore {
        let gitignore_path = cli_opts.target_dir.join(".gitignore");
        if gitignore_path.exists() {
            match parse_gitignore(&gitignore_path) {
                Ok(patterns) => {
                    // 空でないパターンのみを追加
                    let valid_patterns: Vec<String> =
                        patterns.into_iter().filter(|p| !p.is_empty()).collect();

                    if !valid_patterns.is_empty() {
                        eprintln!(
                            "Info: .gitignoreから{}個のパターンを追加します",
                            valid_patterns.len()
                        );

                        // 重複を避けるために既存のパターンと比較
                        for pattern in valid_patterns {
                            if !config_params.exclude_patterns.contains(&pattern) {
                                config_params.exclude_patterns.push(pattern);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: .gitignoreの読み込みに失敗: {}", e);
                }
            }
        } else {
            eprintln!("Info: .gitignoreファイルが見つかりません");
        }
    }

    // 8) 出力ファイルのパス決定
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
        let file_name = if config_params.use_timestamp {
            let ts = Local::now().format("%Y%m%d%H%M%S").to_string();
            format!("output_{}.txt", ts)
        } else {
            "output.txt".to_string()
        };
        default_dir.join(file_name)
    };

    // 9) スキャナ実行
    if let Err(e) = run(&cli_opts.target_dir, &output_path, &config_params, &[]) {
        eprintln!("エラー: {}", e);
        process::exit(1);
    }

    eprintln!("Done! Output => {}", output_path.display());

    // 10) 出力ファイルを開く
    if config_params.open_output {
        match Command::new("code").arg(&output_path).status() {
            Ok(_) => (),
            Err(e) => eprintln!("Warning: VS Code で出力ファイルを開けませんでした: {}", e),
        }
    }
}
