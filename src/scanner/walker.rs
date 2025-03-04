// src/scanner/walker.rs

use globset::GlobSet;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn should_skip_dir(
    entry: &DirEntry,
    target_dir: &Path,
    exclude_globset: &Option<GlobSet>,
) -> bool {
    if let Some(gs) = exclude_globset {
        let rel = match entry.path().strip_prefix(target_dir) {
            Ok(r) => r,
            Err(_) => entry.path(),
        };
        let rel_str = rel.to_string_lossy();
        gs.is_match(Path::new(&*rel_str))
    } else {
        false
    }
}

pub fn collect_entries(
    target_dir: &Path,
    exclude_globset: &Option<GlobSet>,
    only_files: bool,
) -> Vec<DirEntry> {
    let walker = WalkDir::new(target_dir).into_iter().filter_entry(|entry| {
        if entry.file_type().is_dir() {
            !should_skip_dir(entry, target_dir, exclude_globset)
        } else {
            true
        }
    });

    if only_files {
        walker
            .flatten()
            .filter(|entry| entry.file_type().is_file())
            .collect()
    } else {
        walker.flatten().collect()
    }
}
