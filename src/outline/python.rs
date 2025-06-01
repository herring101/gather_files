//! src/outline/python.rs
//!
//! Python のアウトライン実装 (シンプルな行解析)

use crate::outline::provider::{OutlineProvider, Symbol};
use std::path::Path;

pub struct PythonOutlineProvider;

impl OutlineProvider for PythonOutlineProvider {
    fn supports(path: &Path) -> bool {
        matches!(path.extension(), Some(ext) if ext == "py")
    }

    fn extract(_path: &Path, src: &str) -> anyhow::Result<Vec<Symbol>> {
        let mut symbols = Vec::new();
        for line in src.lines() {
            let trimmed = line.trim_start();
            // top-level only
            if trimmed.starts_with("class ") && trimmed.len() == line.len() {
                if let Some(name) = trimmed[6..]
                    .split(|c: char| c == '(' || c == ':' || c.is_whitespace())
                    .next()
                {
                    if !name.is_empty() {
                        symbols.push(Symbol {
                            kind: "class".into(),
                            ident: name.to_string(),
                        });
                    }
                }
            } else if trimmed.starts_with("async def ") && trimmed.len() == line.len() {
                if let Some(name) = trimmed[10..]
                    .split(|c: char| c == '(' || c.is_whitespace())
                    .next()
                {
                    if !name.is_empty() {
                        symbols.push(Symbol {
                            kind: "def".into(),
                            ident: name.to_string(),
                        });
                    }
                }
            } else if trimmed.starts_with("def ") && trimmed.len() == line.len() {
                if let Some(name) = trimmed[4..]
                    .split(|c: char| c == '(' || c.is_whitespace())
                    .next()
                {
                    if !name.is_empty() {
                        symbols.push(Symbol {
                            kind: "def".into(),
                            ident: name.to_string(),
                        });
                    }
                }
            }
        }
        Ok(symbols)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_top_level_defs() {
        let src = r#"
class Foo:
    pass

def bar():
    pass

async def baz():
    pass
"#;
        let syms = PythonOutlineProvider::extract(Path::new("dummy.py"), src).unwrap();
        let kinds: Vec<_> = syms.iter().map(|s| (&s.kind[..], &s.ident[..])).collect();
        assert_eq!(
            kinds,
            vec![("class", "Foo"), ("def", "bar"), ("def", "baz")]
        );
    }

    #[test]
    fn ignore_indented_defs() {
        let src = r#"
class Foo:
    def method(self):
        pass

def bar():
    pass
"#;
        let syms = PythonOutlineProvider::extract(Path::new("dummy.py"), src).unwrap();
        let kinds: Vec<_> = syms.iter().map(|s| (&s.kind[..], &s.ident[..])).collect();
        assert_eq!(kinds, vec![("class", "Foo"), ("def", "bar")]);
    }
}

