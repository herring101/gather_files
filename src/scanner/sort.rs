// src/scanner/sort.rs

use std::path::Path;
use walkdir::DirEntry;

/// ディレクトリは末尾に "/" を付与した文字列を返す
pub fn path_string_for_sort(entry: &DirEntry, target_dir: &Path) -> String {
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

/// ナチュラルソート用のトークン
#[derive(Debug)]
struct Token {
    value: String,
    is_number: bool,
}

/// "doc10.md" -> [("doc", false), ("10", true), (".md", false)]
fn tokenize_for_natural_sort(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_str = String::new();
    let mut current_is_digit = None;

    for c in s.chars() {
        let c_is_digit = c.is_ascii_digit();
        match current_is_digit {
            Some(isdigit) if isdigit == c_is_digit => {
                current_str.push(c);
            }
            Some(_) => {
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

/// 自然順序での文字列比較
fn natural_compare(a: &str, b: &str) -> std::cmp::Ordering {
    let ta = tokenize_for_natural_sort(a);
    let tb = tokenize_for_natural_sort(b);

    let mut i = 0;
    while i < ta.len() && i < tb.len() {
        let (tok_a, tok_b) = (&ta[i], &tb[i]);
        match (tok_a.is_number, tok_b.is_number) {
            (true, true) => {
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
            (true, false) => return std::cmp::Ordering::Less,
            (false, true) => return std::cmp::Ordering::Greater,
        }
        i += 1;
    }

    ta.len().cmp(&tb.len())
}

pub fn compare_dir_entry(a: &DirEntry, b: &DirEntry, target_dir: &Path) -> std::cmp::Ordering {
    let sa = path_string_for_sort(a, target_dir);
    let sb = path_string_for_sort(b, target_dir);
    natural_compare(&sa, &sb)
}
