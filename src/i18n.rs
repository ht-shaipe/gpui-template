use gpui::App;

use crate::app::actions::SelectLocale;
use crate::panels::AppSettings;

/// Initialize i18n
pub fn init(cx: &mut App) {
    let locale = AppSettings::global(cx).locale.clone();
    rust_i18n::set_locale(locale.as_ref());

    cx.on_action(|action: &SelectLocale, cx| {
        change_locale(action.0.as_ref());
        AppSettings::global_mut(cx).locale = action.0.clone();
        cx.refresh_windows();
    });
}

pub fn change_locale(locale: &str) {
    rust_i18n::set_locale(locale);
}
