use gpui::{App, Global, SharedString};
use serde::{Deserialize, Serialize};

/// Application-wide settings persisted to state file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub auto_switch_theme: bool,
    pub font_family: SharedString,
    pub font_size: f64,
    #[serde(default = "default_locale")]
    pub locale: SharedString,
    pub line_height: f64,
    pub resettable: bool,
    pub group_variant: SharedString,
    #[serde(default)]
    pub show_settings: bool,
    #[serde(default)]
    pub show_left_panel: bool,
    #[serde(default)]
    pub show_right_panel: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_switch_theme: false,
            font_family: "Arial".into(),
            font_size: 14.0,
            locale: default_locale(),
            line_height: 12.0,
            resettable: true,
            group_variant: "Fill".into(),
            show_settings: false,
            show_left_panel: true,
            show_right_panel: true,
        }
    }
}

impl Global for AppSettings {}

impl AppSettings {
    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<Self>()
    }
}

fn default_locale() -> SharedString {
    detect_system_locale().unwrap_or_else(|| "en".into())
}

fn detect_system_locale() -> Option<SharedString> {
    let raw = sys_locale::get_locale().or_else(|| std::env::var("LANG").ok())?;
    normalize_locale(&raw).map(SharedString::from)
}

fn normalize_locale(locale: &str) -> Option<&'static str> {
    let lower = locale.to_lowercase();
    if lower.starts_with("zh") {
        return Some("zh-CN");
    }
    if lower.starts_with("en") {
        return Some("en");
    }
    None
}

/// Minimal app state
pub struct AppState {
    app_title: SharedString,
}

impl AppState {
    pub fn init(cx: &mut App) {
        cx.set_global::<AppState>(Self {
            app_title: SharedString::from(""),
        });
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<Self>()
    }

    pub fn set_app_title(&mut self, title: SharedString) {
        self.app_title = title;
    }

    pub fn app_title(&self) -> &SharedString {
        &self.app_title
    }
}

impl Global for AppState {}
