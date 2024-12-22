use std::fs;
use std::path::Path;

use crate::model::ConfigParams;

/// 独自フォーマットの .gather を読む
///
/// 例:
/// [settings]
/// max_lines=1000
/// max_file_size=1000000
/// skip_binary=yes
/// output_dir=out
///
/// [exclude]
/// .git
/// target/
/// *.md
///
/// [skip]
/// *.pdf
///
/// [include]
/// .rs
/// .py
///
pub fn load_config_file(path: &Path) -> ConfigParams {
    let mut params = ConfigParams::default();
    if !path.exists() {
        return params; // 存在しなければデフォルト
    }

    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "Warning: Could not read config file: {} - {}",
                path.display(),
                e
            );
            return params;
        }
    };

    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        // 空行やコメント(# など)はスキップ（必要に応じて拡張してください）
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // セクション行 ([settings], [exclude], etc.)
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].to_lowercase();
            continue;
        }

        match current_section.as_str() {
            "settings" => {
                // settings セクションは key=value の形を想定
                if let Some((k, v)) = parse_key_value(line) {
                    let k_lower = k.to_lowercase();
                    match k_lower.as_str() {
                        "max_lines" => {
                            if let Ok(n) = v.parse::<usize>() {
                                params.max_lines = n;
                            }
                        }
                        "max_file_size" => {
                            if let Ok(n) = v.parse::<u64>() {
                                params.max_file_size = Some(n);
                            }
                        }
                        "skip_binary" => {
                            let v_lower = v.to_lowercase();
                            if ["yes", "true", "1"].contains(&v_lower.as_str()) {
                                params.skip_binary = true;
                            } else {
                                params.skip_binary = false;
                            }
                        }
                        "output_dir" => {
                            if !v.is_empty() {
                                params.output_dir = Some(v);
                            }
                        }
                        _ => {
                            // 未知キーは無視か、ログ出力してもいい
                            eprintln!("Unknown setting key: {}", k);
                        }
                    }
                }
            }
            "exclude" => {
                // 1行につき1パターン
                params.exclude_patterns.push(line.to_string());
            }
            "skip" => {
                // 1行につき1パターン
                params.skip_content_patterns.push(line.to_string());
            }
            "include" => {
                // 1行につき1拡張子
                params.include_exts.push(line.to_string());
            }
            _ => {
                // それ以外のセクションや行は無視
            }
        }
    }

    params
}

/// "key=value" をパースして (key, value) を返す
/// 例: "max_lines=1000" -> Some(("max_lines", "1000"))
fn parse_key_value(line: &str) -> Option<(String, String)> {
    let mut split_iter = line.splitn(2, '=');
    let key = split_iter.next()?.trim();
    let val = split_iter.next()?.trim();
    if key.is_empty() {
        return None;
    }
    Some((key.to_string(), val.to_string()))
}