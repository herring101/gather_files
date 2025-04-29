//! src/outline/rust.rs
//!
//! Rust のアウトライン実装 (syn ベース)

use super::{OutlineProvider, Symbol};
use anyhow::Context;
use std::path::Path;
use syn::{visit::Visit, File, Item, Visibility};

pub struct RustOutlineProvider;

impl OutlineProvider for RustOutlineProvider {
    fn supports(path: &Path) -> bool {
        matches!(path.extension(), Some(ext) if ext == "rs")
    }

    fn extract(path: &Path, src: &str) -> anyhow::Result<Vec<Symbol>> {
        let file: File =
            syn::parse_file(src).with_context(|| format!("failed to parse {:?}", path))?;
        let mut v = Collector::default();
        v.visit_file(&file);
        Ok(v.symbols)
    }
}

/* ------------------------------------------------------------------ */

#[derive(Default)]
struct Collector {
    symbols: Vec<Symbol>,
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_item(&mut self, i: &'ast Item) {
        use Item::*;

        // push() ヘルパ
        let push = |kind: &str, ident: &syn::Ident, vis: &Visibility, symbols: &mut Vec<Symbol>| {
            if matches!(vis, Visibility::Public(_) | Visibility::Restricted(_)) {
                symbols.push(Symbol {
                    kind: kind.into(),
                    ident: ident.to_string(),
                });
            }
        };

        match i {
            Mod(item) => push("mod", &item.ident, &item.vis, &mut self.symbols),
            Struct(item) => push("struct", &item.ident, &item.vis, &mut self.symbols),
            Enum(item) => push("enum", &item.ident, &item.vis, &mut self.symbols),
            Trait(item) => push("trait", &item.ident, &item.vis, &mut self.symbols),
            Fn(item) => push("fn", &item.sig.ident, &item.vis, &mut self.symbols),
            Const(item) => push("const", &item.ident, &item.vis, &mut self.symbols),
            Static(item) => push("static", &item.ident, &item.vis, &mut self.symbols),
            Type(item) => push("type", &item.ident, &item.vis, &mut self.symbols),
            _ => {}
        }

        // 再帰
        syn::visit::visit_item(self, i);
    }
}

/* ------------------------------------------------------------------ */
/* tests                                                              */
/* ------------------------------------------------------------------ */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_public_items_only() {
        let src = r#"
            pub struct PubSt;
            struct PrivSt;
            pub(crate) fn inner() {}
        "#;
        let syms = RustOutlineProvider::extract(Path::new("dummy.rs"), src).unwrap();
        let kinds: Vec<_> = syms.iter().map(|s| (&s.kind[..], &s.ident[..])).collect();
        assert_eq!(kinds, vec![("struct", "PubSt"), ("fn", "inner")]);
    }
}
