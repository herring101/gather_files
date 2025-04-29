use gather_files_lib as lib;

fn main() -> anyhow::Result<()> {
    if let Some(cmd) = std::env::args().nth(1) {
        if cmd == "self-update" || cmd == "update" {
            if let Err(e) = lib::updater::run() {
                eprintln!("Self-update failed: {e}");
                std::process::exit(1);
            }
            return Ok(());
        }
    }

    let cli_opts = lib::parse_args();
    match lib::run(cli_opts) {
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
