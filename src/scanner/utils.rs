// src/scanner/utils.rs

//! Utility helpers: binary‑file detection and glob‑pattern normalisation.

use globset::{Glob, GlobSet, GlobSetBuilder};
use std::{fs::File, io::Read, path::Path};

/// Quick heuristic to decide whether a file is binary.
pub fn is_binary_file(path: &Path) -> bool {
    const SAMPLE: usize = 1024;
    const NON_TEXT_THRESHOLD: f32 = 0.125;

    let mut buf = [0u8; SAMPLE];
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    let n = match file.read(&mut buf) {
        Ok(n) => n,
        Err(_) => return false,
    };
    if n == 0 {
        return false;
    }

    let non_text = buf[..n]
        .iter()
        .filter(|&&b| b == 0 || (b < 0x09 && b != b'\n' && b != b'\r') || b == 0x7F)
        .count();
    (non_text as f32) / (n as f32) > NON_TEXT_THRESHOLD
}

/// Build a [`GlobSet`] from user‑supplied glob patterns.
///
/// Normalisation rules:
/// * `dir/`   ⇒ `**/dir`, `**/dir/**`, `dir/**`
/// * `.rs`    ⇒ `**/*.rs`
/// * `foo.md` ⇒ `**/foo.md`
pub fn build_globset(patterns: &[String]) -> Option<GlobSet> {
    if patterns.is_empty() {
        return None;
    }

    let mut builder = GlobSetBuilder::new();
    let mut add = |pat: &str| match Glob::new(pat) {
        Ok(g) => {
            builder.add(g);
        }
        Err(e) => eprintln!("invalid glob '{}': {}", pat, e),
    };

    for raw in patterns {
        if raw.ends_with('/') {
            let dir = raw.trim_end_matches('/');
            // directory itself at any depth
            add(&format!("**/{}", dir));
            // anything under that directory at any depth
            add(&format!("**/{}/**", dir));
            // top‑level convenience (backward compat)
            add(&format!("{}/**", dir));
        } else if raw.starts_with('.') && !raw.contains('/') && !raw.contains('*') {
            // extension only
            add(&format!("**/*{}", raw));
        } else if !raw.contains('/') && !raw.contains('*') {
            // plain filename
            add(&format!("**/{}", raw));
        } else {
            add(raw);
        }
    }

    builder.build().ok()
}

// ---------------------------------------------------------------------
// tests
// ---------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    fn gs(pats: &[&str]) -> GlobSet {
        build_globset(&pats.iter().map(|s| s.to_string()).collect::<Vec<_>>()).unwrap()
    }

    #[test]
    fn dir_pattern_covers_all_cases() {
        let g = gs(&["gather/"]);
        assert!(g.is_match(Path::new("gather")));
        assert!(g.is_match(Path::new("gather/output.txt")));
        assert!(g.is_match(Path::new("a/b/gather")));
        assert!(g.is_match(Path::new("a/b/gather/file")));
    }

    #[test]
    fn ext_pattern_matches_any_depth() {
        let g = gs(&[".rs"]);
        assert!(g.is_match(Path::new("src/main.rs")));
        assert!(g.is_match(Path::new("deep/lib.rs")));
    }

    #[test]
    fn filename_pattern_matches() {
        let g = gs(&["Cargo.toml"]);
        assert!(g.is_match(Path::new("Cargo.toml")));
        assert!(g.is_match(Path::new("nested/Cargo.toml")));
    }
}
