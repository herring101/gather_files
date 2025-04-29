//! outline モード E2E テスト
//!
//! - `.rs` ファイルを 1 つだけ持つミニプロジェクトを作成
//! - `gather --mode outline` を実行
//! - 出力ファイルに公開シンボルが Markdown で含まれることを確認

use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn outline_runs_and_creates_output() {
    /* --- 仮プロジェクト ---------- */
    let tmp = tempdir().unwrap();
    let root = tmp.path();
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("src/lib.rs"),
        r#"
            pub struct Foo;
            struct Bar;
            pub fn baz() {}
        "#,
    )
    .unwrap();

    /* --- コマンド実行 ------------- */
    Command::cargo_bin("gather")
        .unwrap()
        .current_dir(root)
        .args(["--mode", "outline", "."])
        .assert()
        .success();

    /* --- 出力確認 ------------------ */
    let out = root.join("gather/output.txt");
    let content = fs::read_to_string(out).unwrap();

    // Markdown の行が含まれるか
    assert!(
        content.contains("**struct** Foo"),
        "expected '**struct** Foo' in outline"
    );
    assert!(
        content.contains("**fn** baz"),
        "expected '**fn** baz' in outline"
    );
}
