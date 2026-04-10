mod plugins;

pub use plugins::*;

pub static MAIN_CSS_BYTES: &[u8] = include_bytes!("../../resources/main/css/main.css");
pub static SEARCH_JS_BYTES: &[u8] = include_bytes!("../../resources/main/js/ddoc-search.js");
pub static TOC_ACTIVATE_JS_BYTES: &[u8] =
    include_bytes!("../../resources/main/js/ddoc-toc-activate-visible-item.js");
