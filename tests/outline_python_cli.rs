//! outline モードで Python ファイルが処理されるかテスト

use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn outline_python_file() {
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    fs::write(
        root.join("main.py"),
        "class Foo:\n    pass\n\ndef bar():\n    pass\n",
    )
    .unwrap();

    Command::cargo_bin("gather")
        .unwrap()
        .current_dir(root)
        .args(["--mode", "outline", "."])
        .assert()
        .success();

    let out = root.join("gather/output.txt");
    let content = fs::read_to_string(out).unwrap();
    assert!(content.contains("**class** Foo"), "expected class outline");
    assert!(content.contains("**def** bar"), "expected def outline");
}

