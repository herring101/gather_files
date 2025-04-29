//! Integration test – end-to-end flow of `gather` (v0.3.1)

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

/// 1. `.gather` が自動生成される  
/// 2. そのままスキャンが完走して exit-code 0  
/// 3. gather/output.txt が作成される
#[test]
fn first_run_creates_gather_and_succeeds() {
    /* --- temp プロジェクト作成 --- */
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("src/main.rs"), "fn main() {}").unwrap();

    /* --- 1st run: 成功しつつ .gather を生成 --- */
    Command::cargo_bin("gather")
        .unwrap()
        .current_dir(root)
        .arg(".")
        .assert()
        .success() // ← 旧テストは `failure()`
        .stderr(predicate::str::contains(".gather を生成しました"));

    /* --- .gather が生成されたか --- */
    assert!(root.join(".gather").exists(), ".gather should exist");

    /* --- 出力ファイルが作成されたか --- */
    let gather_dir = root.join("gather");
    let outputs: Vec<_> = fs::read_dir(&gather_dir).unwrap().collect();
    assert!(
        !outputs.is_empty(),
        "expected at least one file in {:?}",
        gather_dir
    );
}
