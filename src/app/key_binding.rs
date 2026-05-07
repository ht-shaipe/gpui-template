use gpui::{App, KeyBinding};

use crate::app::actions::{Quit, ToggleSearch};

// 快捷键绑定初始化
pub fn init(cx: &mut App) {
    cx.bind_keys([
        // 全局快捷键
        KeyBinding::new("/", ToggleSearch, None),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-o", crate::app::actions::Open, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-o", crate::app::actions::Open, None),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-q", Quit, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("alt-f4", Quit, None),
    ]);
}