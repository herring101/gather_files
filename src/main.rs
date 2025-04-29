//! Binary entry-point – thin CLI wrapper around the library.
//!
//! ```bash
//! # 初回は .gather を生成してエラー終了
//! gather .
//!
//! # .gather を編集して再実行
//! gather .
//!
//! # 自己アップデート
//! gather self-update
//! ```

use gather_files_lib as lib;

fn main() -> anyhow::Result<()> {
    /* --- self-update subcommand ----------------------------------- */
    if let Some(cmd) = std::env::args().nth(1) {
        if cmd == "self-update" || cmd == "update" {
            if let Err(e) = lib::updater::run() {
                eprintln!("Self-update failed: {e}");
                std::process::exit(1);
            }
            return Ok(());
        }
    }

    /* --- parse CLI args & run gather ------------------------------ */
    let cli_opts = lib::parse_args();

    match lib::gather(cli_opts) {
        Ok(path) => {
            eprintln!("Done! Output => {}", path.display());
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {e:?}");
            std::process::exit(1);
        }
    }
}
