//! 動的 OutlineProvider レジストリ

use std::path::Path;

use once_cell::sync::Lazy;

use super::provider::{OutlineProvider, Symbol};
use super::rust::RustOutlineProvider;

/* ---------- トレイト ---------- */

/// Scanner / outline の両方から呼び出せる動的トレイト
pub trait DynProvider: Send + Sync {
    fn supports_dyn(&self, path: &Path) -> bool;
    fn extract_dyn(&self, path: &Path, src: &str) -> anyhow::Result<Vec<Symbol>>;
}

impl<T> DynProvider for T
where
    T: OutlineProvider + Send + Sync + 'static,
{
    fn supports_dyn(&self, path: &Path) -> bool {
        T::supports(path)
    }
    fn extract_dyn(&self, path: &Path, src: &str) -> anyhow::Result<Vec<Symbol>> {
        T::extract(path, src)
    }
}

/* ---------- プロバイダ一覧 ---------- */

type DynProviderBox = Box<dyn DynProvider>;

static PROVS: Lazy<Vec<DynProviderBox>> = Lazy::new(|| {
    vec![
        Box::new(RustOutlineProvider) as DynProviderBox,
        // 今後言語を追加するときはここに push!
    ]
});

/// グローバル参照を返す
pub fn providers() -> &'static [DynProviderBox] {
    &PROVS
}
