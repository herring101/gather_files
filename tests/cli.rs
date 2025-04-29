//! Integration test – end-to-end flow of `gather`.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn first_run_creates_gather_then_success_on_second_run() {
    /* --- create a minimal sample project in a temp dir --- */
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    // mimic a tiny Rust project
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("src").join("main.rs"), "fn main() {}").unwrap();

    /* --- 1st run: should create .gather and exit with non-zero --- */
    Command::cargo_bin("gather")
        .unwrap()
        .current_dir(root) // run inside temp dir for cleaner paths
        .arg(".") // scan current directory
        .assert()
        .failure()
        .stderr(predicate::str::contains(".gather を生成しました"));

    /* mark first_run_completed = yes so 2nd run succeeds */
    let gather_path = root.join(".gather");
    let cfg = fs::read_to_string(&gather_path).unwrap();
    let patched = cfg.replace("first_run_completed=no", "first_run_completed=yes");
    fs::write(&gather_path, patched).unwrap();

    /* --- 2nd run: should succeed and create output file --- */
    Command::cargo_bin("gather")
        .unwrap()
        .current_dir(root)
        .arg(".")
        .assert()
        .success()
        .stderr(predicate::str::contains("Done!"));

    /* --- output file exists? --- */
    let gather_dir = root.join("gather");
    let entries: Vec<_> = fs::read_dir(&gather_dir).unwrap().collect();
    assert!(
        !entries.is_empty(),
        "expected at least one file in {:?}",
        gather_dir
    );
}
