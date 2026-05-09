//! Theme JSON embedded at compile time for WASM (no filesystem).

use std::collections::HashMap;

pub fn embedded_themes() -> HashMap<&'static str, &'static str> {
    let mut themes = HashMap::new();
    themes.insert(
        "Default Light",
        include_str!("../../themes/Default Light.json"),
    );
    themes
}
