//! skip と outline が重複した場合は **skip が優先** し、
//! アウトライン本文が一切出力されないことを確認する。

use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn skip_beats_outline() {
    /* ---------- 仮プロジェクト ---------- */
    let tmp = tempdir().unwrap();
    let root = tmp.path();

    // 対象ファイル
    fs::write(root.join("main.txt"), "dummy").unwrap();

    // .gather: 同じパターンを skip と outline の両方に指定
    fs::write(
        root.join(".gather"),
        r#"
[skip]
*.txt
[outline]
*.txt
"#,
    )
    .unwrap();

    /* ---------- 実行 ---------- */
    Command::cargo_bin("gather")
        .unwrap()
        .current_dir(root)
        .arg(".")
        .assert()
        .success();

    /* ---------- 検証 ---------- */
    let out = fs::read_to_string(root.join("gather/output.txt")).unwrap();

    // 1) ディレクトリツリーには省略ラベルが付く
    assert!(
        out.contains("[omitted:pattern]"),
        "tree should contain omitted label"
    );

    // 2) 本文セクション (### main.txt) が生成されていない
    assert!(
        !out.contains("### main.txt"),
        "file body / outline block must NOT appear when skip wins"
    );
}
