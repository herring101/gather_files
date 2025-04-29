//! lib.rs – crate ファサード / オーケストレーション
//!
//! ```text
//!  ・RunMode::Gather   → gather::gather_files()
//!  ・RunMode::Outline  → outline::run() + VSCode オープン
//! ```
//! それ以外の実装詳細は個別モジュールへ委譲し、ここを薄く保つ。

#![deny(warnings)]
#![warn(missing_docs)]

/* ───────────────────── module graph ───────────────────── */

mod args;
mod config;
mod gather; // ← NEW
mod gitignore;
mod model;
mod outline;
mod scanner;
pub mod updater;

/* ──────────────────── public re-exports ────────────────── */

pub use crate::args::parse_args;
pub use gather::gather_files as gather; // 旧 API 継続
pub use model::{CLIOptions as GatherOptions, ConfigParams, OutlineFormat, RunMode};

/* ───────────────────────── deps ────────────────────────── */

use anyhow::Context;
use std::path::PathBuf;
use std::process::Command;

/* ─────────────────── public façade ─────────────────────── */

/// CLI から呼ばれるトップレベル関数。  
/// 完了した出力ファイルのパスを返す。
pub fn run(opts: GatherOptions) -> anyhow::Result<PathBuf> {
    match opts.mode {
        RunMode::Gather => gather::gather_files(opts),
        RunMode::Outline(fmt) => run_outline(opts, fmt),
    }
}

/* -----------------------------------------------------------------
   outline wrapper
----------------------------------------------------------------- */

fn run_outline(opts: GatherOptions, fmt: OutlineFormat) -> anyhow::Result<PathBuf> {
    // gather と同じ出力パス決定ロジックを再利用
    let output = gather::determine_output_path(&opts, &ConfigParams::default())?;

    outline::run(&opts.target_dir, &output, fmt).context("outline failed")?;

    if !opts.no_open {
        let _ = Command::new("code").arg(&output).status();
    }
    Ok(output)
}
