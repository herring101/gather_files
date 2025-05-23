mod provider;
pub mod registry;
mod rust; // ← pub にした

use crate::model::OutlineFormat;
use provider::Symbol;
use registry::providers; // 共有プロバイダ
use serde_json::json;
use std::fs;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

/* ----------- 以下は元のまま ----------- */

pub fn run(dir: &Path, output: &Path, fmt: OutlineFormat) -> anyhow::Result<()> {
    let mut out = fs::File::create(output)?;

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let src = fs::read_to_string(path).unwrap_or_default();

        if let Some(p) = providers().iter().find(|p| p.supports_dyn(path)) {
            let symbols = p.extract_dyn(path, &src)?;
            if symbols.is_empty() {
                continue;
            }
            match fmt {
                OutlineFormat::Md => write_md(&mut out, path, symbols)?,
                OutlineFormat::Json => write_json(&mut out, path, symbols)?,
            }
        }
    }
    Ok(())
}

/* ---------------- writers ------------------------------------------ */

fn write_md(out: &mut fs::File, path: &Path, symbols: Vec<Symbol>) -> std::io::Result<()> {
    writeln!(out, "### {}", path.display())?;
    for s in symbols {
        writeln!(out, "- **{}** {}", s.kind, s.ident)?;
    }
    writeln!(out)?;
    Ok(())
}

fn write_json(out: &mut fs::File, path: &Path, symbols: Vec<Symbol>) -> std::io::Result<()> {
    let v = json!({
        "file": path.to_string_lossy(),
        "symbols": symbols.iter().map(|s| json!({"kind": s.kind, "ident": s.ident})).collect::<Vec<_>>()
    });
    writeln!(out, "{}", v)
}
