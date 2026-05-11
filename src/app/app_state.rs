use gpui::{App, Global, SharedString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    #[serde(default = "default_true")]
    pub show_left_panel: bool,
    #[serde(default = "default_true")]
    pub show_right_panel: bool,
    #[serde(default = "default_true")]
    pub show_bottom_panel: bool,
    
    // Update settings
    #[serde(default)]
    pub auto_check_on_startup: bool,
    #[serde(default = "default_true")]
    pub notifications_enabled: bool,
    #[serde(default)]
    pub auto_update: bool,
    #[serde(default = "default_check_frequency")]
    pub check_frequency_days: f64,
    
    // Terminal state for bottom panel (not persisted)
    #[serde(skip)]
    pub terminal_tabs: HashMap<usize, TerminalTabState>,
    #[serde(skip)]
    pub active_terminal_tab_id: usize,
    #[serde(skip)]
    pub next_terminal_tab_id: usize,
}

/// State for a single terminal tab (kept in memory, not persisted)
#[derive(Debug, Clone)]
pub struct TerminalTabState {
    pub id: usize,
    pub title: SharedString,
    pub output: Vec<SharedString>,
}

impl Default for AppSettings {
    fn default() -> Self {
        let mut this = Self {
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
            show_bottom_panel: true,
            auto_check_on_startup: false,
            notifications_enabled: true,
            auto_update: false,
            check_frequency_days: 7.0,
            terminal_tabs: HashMap::new(),
            active_terminal_tab_id: 0,
            next_terminal_tab_id: 1,
        };
        // Create initial terminal tab
        this.add_terminal_tab();
        this
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
    
    pub fn add_terminal_tab(&mut self) -> usize {
        let id = self.next_terminal_tab_id;
        self.next_terminal_tab_id += 1;
        self.terminal_tabs.insert(id, TerminalTabState {
            id,
            title: format!("Terminal {}", id).into(),
            output: vec!["$ ".into()],
        });
        self.active_terminal_tab_id = id;
        id
    }
    
    pub fn close_terminal_tab(&mut self, id: usize) {
        if self.terminal_tabs.len() <= 1 {
            return;
        }
        self.terminal_tabs.remove(&id);
        if self.active_terminal_tab_id == id {
            self.active_terminal_tab_id = self.terminal_tabs.keys().last().copied().unwrap_or(0);
        }
    }
    
    pub fn set_active_terminal_tab(&mut self, id: usize) {
        if self.terminal_tabs.contains_key(&id) {
            self.active_terminal_tab_id = id;
        }
    }
    
    pub fn get_active_terminal_tab(&self) -> Option<&TerminalTabState> {
        self.terminal_tabs.get(&self.active_terminal_tab_id)
    }
    
    pub fn get_all_terminal_tabs(&self) -> Vec<&TerminalTabState> {
        self.terminal_tabs.values().collect()
    }
}

fn default_true() -> bool {
    true
}

fn default_check_frequency() -> f64 {
    7.0
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
