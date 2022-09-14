mod components;
mod css_collection;
mod css_macro;

#[doc(hidden)]
pub use sir_macro::{compile_global_style, compile_style_item};

pub use components::*;
use css_collection::CssCollection;
use once_cell::sync::Lazy;

#[doc(hidden)]
pub static DEFAULT_CSS_COLLECTION: Lazy<CssCollection> = Lazy::new(|| CssCollection::new());
