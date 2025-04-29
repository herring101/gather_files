// src/scanner/walker.rs
use globset::GlobSet;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn collect_entries(
    target_dir: &Path,
    exclude_globset: &Option<GlobSet>,
    files_only: bool,
) -> Vec<DirEntry> {
    let skip_dir = |entry: &DirEntry| -> bool {
        if let Some(gs) = exclude_globset {
            let rel: PathBuf = entry
                .path()
                .strip_prefix(target_dir)
                .unwrap_or(entry.path())
                .into();
            gs.is_match(&rel)
        } else {
            false
        }
    };

    WalkDir::new(target_dir)
        .into_iter()
        .filter_entry(|e| !(e.depth() > 0 && e.file_type().is_dir() && skip_dir(e)))
        .flatten()
        .filter(|e| !files_only || e.file_type().is_file())
        .collect()
}

// ---------------------------------------------------------------------
// tests
// ---------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::utils::build_globset;
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    fn exclude_dir_is_skipped() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("keep")).unwrap();
        fs::create_dir_all(root.join("skip")).unwrap();
        File::create(root.join("keep/file")).unwrap();
        File::create(root.join("skip/file")).unwrap();

        let gs = build_globset(&["skip/".to_string()]).unwrap();
        let entries = collect_entries(root, &Some(gs), true);
        let paths: Vec<_> = entries
            .iter()
            .map(|e| e.path().strip_prefix(root).unwrap().to_path_buf())
            .collect();
        assert_eq!(paths, vec![PathBuf::from("keep/file")]);
    }
}
