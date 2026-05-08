use std::path::PathBuf;

use gpui::{App, SharedString, px};
use gpui_component::{ActiveTheme as _, Theme, ThemeRegistry};
use serde::{Deserialize, Serialize};

use crate::app::actions::{SwitchTheme, SwitchThemeMode};
use crate::panels::AppSettings;

/// Persisted state
#[derive(Debug, Clone, Serialize, Deserialize)]
struct State {
    theme: SharedString,
    #[serde(default)]
    app_settings: Option<AppSettings>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: "Default Light".into(),
            app_settings: None,
        }
    }
}

fn state_file_path() -> PathBuf {
    let dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::env::temp_dir())
        .join("gpui-template");
    let _ = std::fs::create_dir_all(&dir);
    dir.join("state.json")
}

fn load_state() -> State {
    let path = state_file_path();
    let json = std::fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&json).unwrap_or_default()
}

fn write_state(state: &State) {
    if let Ok(json) = serde_json::to_string_pretty(state) {
        let _ = std::fs::write(state_file_path(), json);
    }
}

/// Initialize themes
pub fn init(cx: &mut App) {
    let state = load_state();

    // Initialize AppSettings globally
    let app_settings = state.app_settings.unwrap_or_default();
    cx.set_global::<AppSettings>(app_settings.clone());

    // Watch themes directory
    let themes_dir = if cfg!(debug_assertions) {
        PathBuf::from("./themes")
    } else {
        let dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("./"))
            .join("gpui-template")
            .join("themes");
        let _ = std::fs::create_dir_all(&dir);
        dir
    };

    if let Err(err) = ThemeRegistry::watch_dir(themes_dir, cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx)
            .themes()
            .get(&state.theme)
            .cloned()
        {
            Theme::global_mut(cx).apply_config(&theme);
            sync_font_size(cx);
            cx.refresh_windows();
        }
    }) {
        tracing::error!("Failed to watch themes directory: {}", err);
    }

    // Sync font size
    Theme::global_mut(cx).font_size = px(app_settings.font_size as f32);
    cx.refresh_windows();
    save_state(cx);

    // Observe theme changes → persist
    cx.observe_global::<Theme>(|cx| {
        save_state(cx);
    })
    .detach();

    // Observe settings changes → sync font_size
    cx.observe_global::<AppSettings>(|cx| {
        let font_size = AppSettings::global(cx).font_size;
        Theme::global_mut(cx).font_size = px(font_size as f32);
        save_state(cx);
    })
    .detach();

    // Theme switch action
    cx.on_action(|switch: &SwitchTheme, cx| {
        if let Some(theme_config) = ThemeRegistry::global(cx).themes().get(&switch.0).cloned() {
            Theme::global_mut(cx).apply_config(&theme_config);
            sync_font_size(cx);
        }
        cx.refresh_windows();
        save_state(cx);
    });

    // Theme mode switch action
    cx.on_action(|switch: &SwitchThemeMode, cx| {
        Theme::change(switch.0, None, cx);
        cx.refresh_windows();
        save_state(cx);
    });
}

fn sync_font_size(cx: &mut App) {
    let font_size = AppSettings::global(cx).font_size;
    Theme::global_mut(cx).font_size = px(font_size as f32);
}

pub(crate) fn save_state(cx: &mut App) {
    let state = State {
        theme: cx.theme().theme_name().clone(),
        app_settings: Some(AppSettings::global(cx).clone()),
    };
    write_state(&state);
}
