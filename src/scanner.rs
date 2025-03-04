use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::{DirEntry, WalkDir};

use crate::model::ConfigParams;

/// 簡易バイナリ判定
fn is_binary_file(path: &Path) -> bool {
    use std::io::Read;
    if let Ok(mut f) = File::open(path) {
        let mut buffer = [0; 1024];
        if let Ok(n) = f.read(&mut buffer) {
            let non_text_count = buffer[..n]
                .iter()
                .filter(|&&b| b == 0 || (b < 7 || b == 127))
                .count();
            // 1/8 以上が非テキストならバイナリ判定
            if non_text_count > (n / 8) {
                return true;
            }
        }
    }
    false
}

/// build_globset
fn build_globset(patterns: &[String]) -> Option<GlobSet> {
    if patterns.is_empty() {
        return None;
    }
    let mut builder = GlobSetBuilder::new();
    for pat in patterns {
        let expanded = if pat.ends_with('/') {
            // ディレクトリパターンっぽい場合は再帰的に除外
            format!("{}**", pat)
        } else {
            pat.clone()
        };
        match Glob::new(&expanded) {
            Ok(g) => {
                builder.add(g);
            }
            Err(e) => {
                eprintln!("Warning: invalid glob pattern '{}': {}", pat, e);
            }
        }
    }
    match builder.build() {
        Ok(gs) => Some(gs),
        Err(e) => {
            eprintln!("Error building globset: {}", e);
            None
        }
    }
}

/// exclude_dir のためのチェック
fn should_skip_dir(entry: &DirEntry, target_dir: &Path, exclude_globset: &Option<GlobSet>) -> bool {
    if let Some(gs) = exclude_globset {
        let rel = match entry.path().strip_prefix(target_dir) {
            Ok(r) => r,
            Err(_) => entry.path(),
        };
        let rel_str = rel.to_string_lossy();
        if gs.is_match(Path::new(&*rel_str)) {
            return true;
        }
    }
    false
}

// ------------------ ここから: ディレクトリ階層を考慮したソート ------------------

/// ディレクトリは末尾に "/" を付与した文字列を返す
/// これをナチュラルソートにかけることで、
/// 「dir1/」 < 「dir1/file1.md」 < 「dir2/」 のように正しい順序が得られる。
fn path_string_for_sort(entry: &DirEntry, target_dir: &Path) -> String {
    let rel_path = entry
        .path()
        .strip_prefix(target_dir)
        .unwrap_or(entry.path());
    let mut s = rel_path.to_string_lossy().to_string();
    if entry.file_type().is_dir() && !s.ends_with('/') {
        s.push('/');
    }
    s
}

/// ナチュラルソート (数字を数値として比較)
/// 自前実装の例: "doc2" < "doc10" のようにしたい
fn natural_compare(a: &str, b: &str) -> std::cmp::Ordering {
    let ta = tokenize_for_natural_sort(a);
    let tb = tokenize_for_natural_sort(b);

    let mut i = 0;
    while i < ta.len() && i < tb.len() {
        let (tok_a, tok_b) = (&ta[i], &tb[i]);
        match (tok_a.is_number, tok_b.is_number) {
            (true, true) => {
                // 両方数字なら数値化して比較
                let na = tok_a.value.parse::<u64>().unwrap_or(0);
                let nb = tok_b.value.parse::<u64>().unwrap_or(0);
                if na != nb {
                    return na.cmp(&nb);
                }
            }
            (false, false) => {
                let ord = tok_a.value.cmp(&tok_b.value);
                if ord != std::cmp::Ordering::Equal {
                    return ord;
                }
            }
            (true, false) => {
                // 数字を文字より先に(または後に)するかは好み
                // ここでは数字を前にしてみる
                return std::cmp::Ordering::Less;
            }
            (false, true) => {
                return std::cmp::Ordering::Greater;
            }
        }
        i += 1;
    }

    // どちらかが先にトークンを使い切ったら、短い方を前に
    ta.len().cmp(&tb.len())
}

/// "doc10.md" -> [("doc", false), ("10", true), (".md", false)]
#[derive(Debug)]
struct Token {
    value: String,
    is_number: bool,
}

fn tokenize_for_natural_sort(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_str = String::new();
    let mut current_is_digit = None;

    for c in s.chars() {
        let c_is_digit = c.is_ascii_digit();
        match current_is_digit {
            Some(isdigit) if isdigit == c_is_digit => {
                // 同種が続く
                current_str.push(c);
            }
            Some(_) => {
                // 種類が変わった
                tokens.push(Token {
                    value: current_str,
                    is_number: current_is_digit.unwrap(),
                });
                current_str = String::new();
                current_str.push(c);
                current_is_digit = Some(c_is_digit);
            }
            None => {
                current_str.push(c);
                current_is_digit = Some(c_is_digit);
            }
        }
    }
    if !current_str.is_empty() {
        tokens.push(Token {
            value: current_str,
            is_number: current_is_digit.unwrap_or(false),
        });
    }
    tokens
}

fn compare_dir_entry(a: &DirEntry, b: &DirEntry, target_dir: &Path) -> std::cmp::Ordering {
    let sa = path_string_for_sort(a, target_dir);
    let sb = path_string_for_sort(b, target_dir);
    natural_compare(&sa, &sb)
}

// ------------------ ここまで: ディレクトリ階層を考慮したソート ------------------

/// メインの走査関数
pub fn run(
    target_dir: &Path,
    output_file: &Path,
    config: &ConfigParams,
    _cli_include_exts: &[String],
) -> Result<(), String> {
    // 出力ファイルを開く
    let mut outfile = File::create(output_file).map_err(|e| {
        format!(
            "出力ファイルを作成できません: {} - {}",
            output_file.display(),
            e
        )
    })?;

    // exclude / skip 用の globset
    let exclude_globset = build_globset(&config.exclude_patterns);
    let skip_globset = build_globset(&config.skip_content_patterns);

    // include patterns
    // 空の場合はNoneを返すので、空の場合は全ファイルを含める
    let include_globset = if config.include_patterns.is_empty() {
        None
    } else {
        build_globset(&config.include_patterns)
    };

    // --------------------------------------------------
    // (1) ディレクトリツリーを出力
    // --------------------------------------------------
    let mut entries = Vec::new();
    let walker_for_tree = WalkDir::new(target_dir).into_iter().filter_entry(|entry| {
        // excludeパターンに当たるディレクトリは再帰しない
        if entry.file_type().is_dir() {
            !should_skip_dir(entry, target_dir, &exclude_globset)
        } else {
            true
        }
    });

    for e in walker_for_tree {
        if let Ok(entry) = e {
            entries.push(entry);
        }
    }

    // ディレクトリもファイルも、相対パス全体でナチュラルソート
    entries.sort_by(|a, b| compare_dir_entry(a, b, target_dir));

    writeln!(outfile, "```").ok();
    for entry in &entries {
        let path = entry.path();
        let rel = match path.strip_prefix(target_dir) {
            Ok(r) => r,
            Err(_) => path,
        };

        if path.is_dir() {
            // ディレクトリ
            let level = rel.components().count();
            let indent = "    ".repeat(level.saturating_sub(1));
            let name = path
                .file_name()
                .map(|s| s.to_string_lossy())
                .unwrap_or_default();
            writeln!(outfile, "{}{}/", indent, name).ok();
        } else {
            // ファイル
            let rel_str = rel.to_string_lossy();

            // exclude
            if let Some(gs) = &exclude_globset {
                if gs.is_match(Path::new(&*rel_str)) {
                    continue;
                }
            }
            // include patterns
            if let Some(gs) = &include_globset {
                // include_patternsが空でない場合のみチェック
                if !gs.is_match(Path::new(&*rel_str)) {
                    continue;
                }
            }

            let level = rel.components().count();
            let indent = "    ".repeat(level.saturating_sub(1));
            let name = path
                .file_name()
                .map(|s| s.to_string_lossy())
                .unwrap_or_default();
            writeln!(outfile, "{}{}", indent, name).ok();
        }
    }
    writeln!(outfile, "```").ok();
    writeln!(outfile).ok();

    // --------------------------------------------------
    // (2) ファイル内容の出力
    // --------------------------------------------------
    let mut file_entries = Vec::new();
    let walker_for_files = WalkDir::new(target_dir).into_iter().filter_entry(|entry| {
        if entry.file_type().is_dir() {
            !should_skip_dir(entry, target_dir, &exclude_globset)
        } else {
            let rel = match entry.path().strip_prefix(target_dir) {
                Ok(r) => r,
                Err(_) => entry.path(),
            };
            let rel_str = rel.to_string_lossy();

            // exclude
            if let Some(gs) = &exclude_globset {
                if gs.is_match(Path::new(&*rel_str)) {
                    return false;
                }
            }
            // include patterns
            if let Some(gs) = &include_globset {
                // include_patternsが空でない場合のみチェック
                if !gs.is_match(Path::new(&*rel_str)) {
                    return false;
                }
            }
            true
        }
    });

    for e in walker_for_files {
        if let Ok(entry) = e {
            if entry.file_type().is_file() {
                file_entries.push(entry);
            }
        }
    }

    // ファイルのみ、相対パス全体でナチュラルソート
    file_entries.sort_by(|a, b| compare_dir_entry(a, b, target_dir));

    let total = file_entries.len();
    let mut count = 1;

    for entry in file_entries {
        let path = entry.path();
        let rel = match path.strip_prefix(target_dir) {
            Ok(r) => r,
            Err(_) => path,
        };
        let rel_str = rel.to_string_lossy();

        eprintln!("({}/{}) Processing: {}", count, total, path.display());
        count += 1;

        // skip pattern
        if let Some(gs) = &skip_globset {
            if gs.is_match(Path::new(&*rel_str)) {
                writeln!(outfile, "### {}", rel_str).ok();
                writeln!(outfile, "```").ok();
                writeln!(outfile, "(略)").ok();
                writeln!(outfile, "```").ok();
                writeln!(outfile).ok();
                continue;
            }
        }

        // バイナリチェック
        if config.skip_binary && is_binary_file(path) {
            writeln!(outfile, "### {}", rel_str).ok();
            writeln!(outfile, "```").ok();
            writeln!(outfile, "(略) バイナリファイル").ok();
            writeln!(outfile, "```").ok();
            writeln!(outfile).ok();
            continue;
        }

        // サイズ制限
        if let Some(max_size) = config.max_file_size {
            if let Ok(meta) = fs::metadata(path) {
                if meta.len() > max_size {
                    writeln!(outfile, "### {}", rel_str).ok();
                    writeln!(outfile, "```").ok();
                    writeln!(outfile, "(略)").ok();
                    writeln!(outfile, "```").ok();
                    writeln!(outfile).ok();
                    continue;
                }
            }
        }

        // ファイル内容を出力
        writeln!(outfile, "### {}", rel_str).ok();
        writeln!(outfile, "```").ok();

        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                writeln!(outfile, "Error: {}", e).ok();
                writeln!(outfile).ok();
                continue;
            }
        };
        let reader = BufReader::new(file);

        let mut line_count = 0;
        for line_result in reader.lines() {
            match line_result {
                Ok(line) => {
                    if line_count >= config.max_lines {
                        writeln!(outfile, "...").ok();
                        writeln!(outfile, "(省略)").ok();
                        break;
                    }
                    writeln!(outfile, "{}", line).ok();
                    line_count += 1;
                }
                Err(e) => {
                    writeln!(outfile, "Error reading line: {}", e).ok();
                    break;
                }
            }
        }

        writeln!(outfile, "```").ok();
        writeln!(outfile).ok();
    }

    Ok(())
}
