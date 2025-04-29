//! Library entry-point for **gather_files**
//!
//! すべてのビジネスロジックをここに集約し、`src/main.rs` は
//! *薄い CLI ラッパ* として保守します。

#![deny(warnings)] // ← **全警告をコンパイルエラー扱い** に戻す
#![warn(missing_docs)] // （missing_docs も含めて解決済み）

/* ────────────────────── module graph ────────────────────── */

mod args;
mod config;
mod gitignore;
mod model;
mod scanner;
pub mod updater; // self-update

/* ────────────────── public surface re-exports ───────────── */

pub use crate::args::parse_args;
pub use model::{CLIOptions as GatherOptions, ConfigParams};

/* ────────────────────────── deps ────────────────────────── */

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Context;

/* ─────────────────────── API: gather ─────────────────────── */

/// Scan & gather source files according to `GatherOptions`.
///
/// Returns the **absolute path** of the generated output file.
pub fn gather(opts: GatherOptions) -> anyhow::Result<PathBuf> {
    use crate::config::load_config_file;
    use crate::gitignore::parse_gitignore;
    use crate::scanner::run;

    if !opts.target_dir.is_dir() {
        anyhow::bail!(
            "指定ディレクトリが存在しません: {}",
            opts.target_dir.display()
        );
    }

    /* --- .gather path --- */
    let gather_path = opts
        .config_file
        .clone()
        .unwrap_or_else(|| opts.target_dir.join(".gather"));

    /* --- create template on first run --- */
    if !gather_path.exists() {
        create_gather_template(&opts, &gather_path)?;
        anyhow::bail!(".gather を生成しました。編集後に再実行してください");
    }

    /* --- load & merge config --- */
    let mut cfg = load_config_file(&gather_path);
    merge_cli_into_config(&opts, &mut cfg)?;

    /* --- integrate .gitignore if requested --- */
    if cfg.use_gitignore {
        let gi_path = opts.target_dir.join(".gitignore");
        if gi_path.exists() {
            if let Ok(patterns) = parse_gitignore(&gi_path) {
                for p in patterns.into_iter().filter(|p| !p.is_empty()) {
                    if !cfg.exclude_patterns.contains(&p) {
                        cfg.exclude_patterns.push(p);
                    }
                }
            }
        }
    }

    /* --- decide output path --- */
    let output_path = determine_output_path(&opts, &cfg)?;

    /* --- scanning --- */
    run(&opts.target_dir, &output_path, &cfg, &[])
        .map_err(|e| anyhow::anyhow!(e))
        .context("scanner failed")?;

    /* --- open in VS Code if requested --- */
    if cfg.open_output {
        let _ = Command::new("code").arg(&output_path).status();
    }

    Ok(output_path)
}

/* ─────────────────── helper functions ───────────────────── */

fn create_gather_template(opts: &GatherOptions, path: &Path) -> anyhow::Result<()> {
    use crate::scanner::detector::{detect_large_directories, generate_exclude_patterns};
    use std::fs;

    eprintln!("初回実行: .gather を生成します …");

    /* auto-detect large directories */
    let dirs = detect_large_directories(&opts.target_dir, 100, 1_000_000);
    let auto = generate_exclude_patterns(&dirs, &opts.target_dir);

    /* default template + auto patterns */
    let mut tmpl = include_str!("templates/gather_default.toml").to_string();
    for p in auto {
        tmpl.push_str(&format!("{p}\n"));
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

fn determine_output_path(opts: &GatherOptions, cfg: &ConfigParams) -> anyhow::Result<PathBuf> {
    use chrono::Local;
    use std::fs;

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
