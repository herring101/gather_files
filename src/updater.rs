//! Self‑update implementation – downloads the latest "gather" binary.
//!
//! ```bash
//! gather self-update
//! ```
//!
//! 成功時は `Updated 🎉 → vX.Y.Z`、最新版の場合は
//! `Already up-to-date` を標準出力へ返す。

use self_update::{backends::github::Update, Status};
use std::error::Error;

/// Run the self‑update process.
pub fn run() -> Result<(), Box<dyn Error>> {
    let target = platform_target();

    let status = Update::configure()
        .repo_owner("herring101")
        .repo_name("gather_files") // GitHub リポジトリ名は据え置き
        .bin_name("gather") // ← 新バイナリ名
        .target(&target) // gather‑<target>.* を期待
        .current_version(env!("CARGO_PKG_VERSION"))
        .show_download_progress(true)
        .build()?
        .update()?;

    match status {
        Status::Updated(new) => println!("Updated 🎉  → v{new}"),
        Status::UpToDate(ver) => println!("Already up-to-date (v{ver})"),
    }
    Ok(())
}

/* ───────────────────────── helpers ───────────────────────── */

/// Return the self_update "target" string (without bin prefix).
fn platform_target() -> String {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => "linux-musl-amd64".to_string(),
        ("linux", "aarch64") => "linux-arm64".to_string(),
        ("macos", "x86_64") => "macos-amd64".to_string(),
        ("macos", "aarch64") => "macos-arm64".to_string(),
        ("windows", "x86_64") => "windows-amd64.exe".to_string(),
        _ => format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
    }
}
