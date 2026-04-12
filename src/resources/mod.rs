mod plugins;

pub use plugins::*;

pub static MAIN_CSS_BYTES: &[u8] = include_bytes!("../../resources/main/css/main.css");
