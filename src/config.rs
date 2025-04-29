// src/config.rs
//! `.gather` 設定ファイルパーサ
//! セクション見出しの末尾 `]` 以降にコメント／空白があっても許容する。

use std::{collections::HashMap, fs, path::Path};

use crate::model::ConfigParams;

/// 読み込み。存在しなければ `default()` を返す。
pub fn load_config_file(path: &Path) -> ConfigParams {
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return ConfigParams::default(),
    };

    let mut params = ConfigParams::default();

    /* ---------- settings キー → 更新クロージャ ---------- */
    type Setter = fn(&mut ConfigParams, &str);
    let mut map: HashMap<&str, Setter> = HashMap::new();
    macro_rules! set_bool {
        ($field:ident) => {
            |p: &mut ConfigParams, v: &str| {
                let b = matches!(v.trim().to_lowercase().as_str(), "yes" | "true" | "1");
                p.$field = b;
            }
        };
    }
    map.insert("max_lines", |p, v| p.max_lines = v.parse().unwrap_or(p.max_lines));
    map.insert("max_file_size", |p, v| p.max_file_size = v.parse().ok());
    map.insert("skip_binary", set_bool!(skip_binary));
    map.insert("output_dir", |p, v| {
        if !v.is_empty() {
            p.output_dir = Some(v.to_string())
        }
    });
    map.insert("use_timestamp", set_bool!(use_timestamp));
    map.insert("open_output", set_bool!(open_output));
    map.insert("use_gitignore", set_bool!(use_gitignore));
    map.insert("first_run_completed", set_bool!(first_run_completed));
    map.insert("max_files_per_dir", |p, v| {
        p.max_files_per_dir = v.parse().unwrap_or(p.max_files_per_dir)
    });
    map.insert("max_auto_file_size", |p, v| {
        p.max_auto_file_size = v.parse().unwrap_or(p.max_auto_file_size)
    });

    /* ---------- 行ループ ---------- */
    enum Section {
        None,
        Settings,
        Exclude,
        Skip,
        Include,
    }
    let mut section = Section::None;

    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // [section] 行の判定をゆるくする
        if line.starts_with('[') {
            if let Some(end) = line.find(']') {
                let name = &line[1..end];
                section = match name.trim().to_lowercase().as_str() {
                    "settings" => Section::Settings,
                    "exclude" => Section::Exclude,
                    "skip" => Section::Skip,
                    "include" => Section::Include,
                    _ => Section::None,
                };
                continue;
            }
        }

        match section {
            Section::Settings => {
                if let Some((k, v)) = line.split_once('=') {
                    if let Some(set) = map.get(k.trim().to_lowercase().as_str()) {
                        set(&mut params, v.trim());
                    }
                }
            }
            Section::Exclude => push_pattern(&mut params.exclude_patterns, line),
            Section::Skip => push_pattern(&mut params.skip_content_patterns, line),
            Section::Include => push_pattern(&mut params.include_patterns, line),
            Section::None => {}
        }
    }
    params
}

fn push_pattern(vec: &mut Vec<String>, line: &str) {
    let pat = line.split('#').next().unwrap_or("").trim();
    if !pat.is_empty() {
        vec.push(pat.to_string());
    }
}

/* --------------------------------------------------------------------
   unit tests
   -------------------------------------------------------------------- */
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    const SAMPLE: &str = r#"
[settings]
use_gitignore = yes

[exclude]   # dir patterns
node_modules/   # comment OK
*.log

[skip] # skip content only
*.pdf

[include]
*.rs
"#;

    #[test]
    fn parse_section_with_trailing_comment() {
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "{}", SAMPLE).unwrap();
        let cfg = load_config_file(tmp.path());

        assert_eq!(
            cfg.exclude_patterns,
            vec!["node_modules/".to_string(), "*.log".to_string()]
        );
        assert_eq!(cfg.skip_content_patterns, vec!["*.pdf"]);
        assert_eq!(cfg.include_patterns, vec!["*.rs"]);
    }
}
