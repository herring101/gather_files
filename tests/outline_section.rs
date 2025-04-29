//! `[outline]` セクションで Rust ファイルがアウトライン化されるか

use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn outline_section_outputs_outline_only() {
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    // ソース生成
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("src/lib.rs"), "pub fn foo() {}\n").unwrap();

    // .gather
    fs::write(
        root.join(".gather"),
        r#"
[outline]
*.rs
"#,
    )
    .unwrap();

    Command::cargo_bin("gather")
        .unwrap()
        .current_dir(root)
        .arg(".")
        .assert()
        .success();

    let out = fs::read_to_string(root.join("gather/output.txt")).unwrap();
    assert!(out.contains("**fn** foo"), "outline not found");
    assert!(
        !out.contains("pub fn foo()"),
        "source body should be absent"
    );
}
