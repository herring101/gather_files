//! src/gather.rs
//!
//! 「ファイル収集 (gather)」ワークフローの実装。
//! CLI からは lib::run() 経由で呼び出される。

use crate::config::load_config_file;
use crate::gitignore::parse_gitignore;
use crate::model::{CLIOptions as GatherOptions, ConfigParams};
use crate::scanner::run as scan_run;

use anyhow::Context;
use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/* =======================================================================
public API
======================================================================= */

/// gather-mode のエントリーポイント。  
/// 成功時に **生成された出力ファイルの絶対パス** を返す。
pub fn gather_files(opts: GatherOptions) -> anyhow::Result<PathBuf> {
    /* --- 前提チェック -------------------------------------------------- */
    if !opts.target_dir.is_dir() {
        anyhow::bail!(
            "指定ディレクトリが存在しません: {}",
            opts.target_dir.display()
        );
    }

    /* --- .gather パス決定 --------------------------------------------- */
    let gather_path = opts
        .config_file
        .clone()
        .unwrap_or_else(|| opts.target_dir.join(".gather"));

    /* ─────────────── 初回実行：テンプレ生成 ─────────────── */
    if !gather_path.exists() {
        create_gather_template(&opts, &gather_path)?;
        eprintln!(
            ".gather を生成しました (デフォルト設定でスキャンを続行します。後で編集してください)…"
        );
    }

    /* --- 設定読み込み & CLI 反映 -------------------------------------- */
    let mut cfg = load_config_file(&gather_path);
    merge_cli_into_config(&opts, &mut cfg)?;

    /* --- .gitignore 取り込み (オプション) ------------------------------ */
    if cfg.use_gitignore {
        let gi = opts.target_dir.join(".gitignore");
        if gi.exists() {
            if let Ok(pats) = parse_gitignore(&gi) {
                for p in pats.into_iter().filter(|p| !p.is_empty()) {
                    if !cfg.exclude_patterns.contains(&p) {
                        cfg.exclude_patterns.push(p);
                    }
                }
            }
        }
    }

    /* --- 出力パス決定 -------------------------------------------------- */
    let output_path = determine_output_path(&opts, &cfg)?;

    /* --- 走査 ---------------------------------------------------------- */
    scan_run(&opts.target_dir, &output_path, &cfg, &[])
        .map_err(|e| anyhow::anyhow!(e))
        .context("scanner failed")?;

    /* --- VS Code で開く ------------------------------------------------ */
    if cfg.open_output {
        let _ = Command::new("code").arg(&output_path).status();
    }

    Ok(output_path)
}

/* =======================================================================
shared helper – outline からも再利用するため pub(crate)
======================================================================= */

/// gather / outline 共通の “出力ファイル名決定” ロジック
pub(crate) fn determine_output_path(
    opts: &GatherOptions,
    cfg: &ConfigParams,
) -> anyhow::Result<PathBuf> {
    if let Some(ref p) = opts.output_file {
        return Ok(p.clone());
    }

    let dir = opts.target_dir.join("gather");
    if !dir.is_dir() {
        fs::create_dir_all(&dir)?;
    }

    let fname = if cfg.use_timestamp {
        format!("output_{}.txt", Local::now().format("%Y%m%d%H%M%S"))
    } else {
        "output.txt".into()
    };
    Ok(dir.join(fname))
}

/* =======================================================================
private helpers
======================================================================= */

fn create_gather_template(opts: &GatherOptions, path: &Path) -> anyhow::Result<()> {
    use crate::scanner::detector::{detect_large_directories, generate_exclude_patterns};

    eprintln!("初回実行: .gather を生成します …");

    /* 大規模ディレクトリ自動検出 */
    let dirs = detect_large_directories(&opts.target_dir, 100, 1_000_000);
    let auto = generate_exclude_patterns(&dirs, &opts.target_dir);

    /* テンプレート組み立て */
    let mut tmpl = include_str!("templates/gather_default.toml").to_string();
    if !auto.is_empty() {
        let header = "[exclude]";
        if let Some(pos) = tmpl.find(header) {
            let insert_at = tmpl[pos..]
                .find('\n')
                .map(|off| pos + off + 1)
                .unwrap_or_else(|| tmpl.len());
            let block: String = auto.iter().map(|p| format!("{p}\n")).collect();
            tmpl.insert_str(insert_at, &block);
        } else {
            tmpl.push_str("\n[exclude]\n");
            for p in &auto {
                tmpl.push_str(&format!("{p}\n"));
            }
        }
    }

    fs::write(path, tmpl)?;
    let _ = Command::new("code").arg(path).status();
    Ok(())
}

fn merge_cli_into_config(opts: &GatherOptions, cfg: &mut ConfigParams) -> anyhow::Result<()> {
    if let Some(n) = opts.max_lines {
        cfg.max_lines = n;
    }
    if let Some(b) = opts.max_file_size {
        cfg.max_file_size = Some(b);
    }
    if !opts.extra_exclude_patterns.is_empty() {
        cfg.exclude_patterns
            .extend(opts.extra_exclude_patterns.clone());
    }
    if !opts.extra_skip_patterns.is_empty() {
        cfg.skip_content_patterns
            .extend(opts.extra_skip_patterns.clone());
    }
    if !opts.include_patterns.is_empty() {
        cfg.include_patterns.extend(opts.include_patterns.clone());
    }
    cfg.use_timestamp |= opts.use_timestamp;
    cfg.open_output &= !opts.no_open;
    cfg.use_gitignore |= opts.use_gitignore;
    Ok(())
}
